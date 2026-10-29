//! Runs the ANTLR-generated LaTeX parser and converts its parse tree into a
//! plain owned tree, so the converter does not depend on runtime generics.

use std::cell::Cell;
use std::rc::Rc;

use antlr4rust::error_strategy::BailErrorStrategy;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::errors::ANTLRError;
use antlr4rust::input_stream::InputStream;
use antlr4rust::recognizer::Recognizer;
use antlr4rust::token::{Token, TOKEN_EOF};
use antlr4rust::token_factory::TokenFactory;
use antlr4rust::TokenSource;
use antlr4rust::Parser;
use antlr4rust::prediction_mode::PredictionMode;
#[allow(unused_imports)]
use antlr4rust::interval_set;

use crate::gen::pslexer::PSLexer;
use crate::gen::psparser::{PSParser, PSParserContextType};

pub use crate::gen::psparser::ruleNames as RULE_NAMES;

/// A lexed token. `start`/`stop` are code-point indices into the input.
#[derive(Debug, Clone)]
pub struct Tok {
    pub ttype: i32,
    pub text: String,
    pub start: usize,
    pub stop: usize,
}

#[derive(Debug, Clone)]
pub enum Node {
    Rule(RuleNode),
    Leaf(Tok),
}

#[derive(Debug, Clone)]
pub struct RuleNode {
    pub rule: usize,
    pub children: Vec<Node>,
    /// Index of the first and last token covered (token-stream indices).
    pub start_tok: usize,
    pub stop_tok: usize,
}

pub struct ParseOutput {
    pub root: RuleNode,
    pub tokens: Vec<Tok>,
    pub chars: Vec<char>,
}

#[derive(Clone)]
struct Flag(Rc<Cell<bool>>);

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for Flag {
    fn syntax_error(
        &self,
        _r: &T,
        _o: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        _l: isize,
        _c: isize,
        _m: &str,
        _e: Option<&ANTLRError>,
    ) {
        self.0.set(true);
    }
}

fn lex_all(data: &[u32]) -> Result<Vec<Tok>, String> {
    let flag = Flag(Rc::new(Cell::new(false)));
    let mut lexer = PSLexer::new(InputStream::new(data));
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(flag.clone()));
    let mut out = Vec::new();
    loop {
        let t = lexer.next_token();
        let tt = t.get_token_type();
        if tt == TOKEN_EOF {
            break;
        }
        out.push(Tok {
            ttype: tt,
            text: t.get_text().to_string(),
            start: t.get_start() as usize,
            stop: t.get_stop() as usize,
        });
    }
    if flag.0.get() {
        return Err("lexer error".into());
    }
    Ok(out)
}

/// Parse `latex` with the `math` start rule. Any lexer or parser error is a failure,
/// matching latex2sympy2_extended, whose error listener raises on the first error.
///
/// Two-stage parsing: fast SLL prediction first, full LL only when SLL reports a
/// syntax error (SLL success implies the same tree as LL for this corpus; verified
/// against Python ANTLR in harness/py_tree_dump.py).
pub fn parse_math(latex: &str) -> Result<ParseOutput, String> {
    let chars: Vec<char> = latex.chars().collect();
    let data: Vec<u32> = chars.iter().map(|c| *c as u32).collect();
    let tokens = lex_all(&data)?;
    let root = match parse_with(&data, &tokens, PredictionMode::SLL) {
        Ok(r) => r,
        Err(_) => parse_with(&data, &tokens, PredictionMode::LL)?,
    };
    Ok(ParseOutput { root, tokens, chars })
}

fn parse_with(data: &[u32], tokens: &[Tok], mode: PredictionMode) -> Result<RuleNode, String> {
    let flag = Flag(Rc::new(Cell::new(false)));
    let mut lexer = PSLexer::new(InputStream::new(data));
    lexer.remove_error_listeners();
    let ts = CommonTokenStream::new(lexer);
    // Bail on the first syntax error, like latex2sympy2_extended's raising error listener.
    // Default error recovery gives the same verdict but can take seconds on long garbage.
    let mut parser = PSParser::with_strategy(ts, Box::new(BailErrorStrategy::new()));
    parser.get_interpreter().set_prediction_mode(mode);
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(flag.clone()));
    let math = parser.math().map_err(|e| format!("{e:?}"))?;
    if flag.0.get() {
        return Err("syntax error".into());
    }
    let mut next = 0usize;
    match convert(&*math as &dyn crate::gen::psparser::PSParserContext, tokens, &mut next)?
        .ok_or("empty tree")?
    {
        Node::Rule(r) => Ok(r),
        Node::Leaf(_) => Err("unexpected leaf root".into()),
    }
}

fn convert<'i>(
    ctx: &(dyn crate::gen::psparser::PSParserContext<'i> + 'i),
    tokens: &[Tok],
    next: &mut usize,
) -> Result<Option<Node>, String> {
    let rule = ctx.get_rule_index();
    if rule == usize::MAX {
        // Leaves are visited in token order; EOF is the only leaf past the last token.
        let Some(t) = tokens.get(*next) else {
            return Ok(None);
        };
        if ctx.get_text() != t.text {
            return Err("token/leaf mismatch".into());
        }
        *next += 1;
        return Ok(Some(Node::Leaf(t.clone())));
    }
    let start_tok = *next;
    let mut children = Vec::with_capacity(ctx.get_child_count());
    for c in ctx.get_children() {
        if let Some(n) = convert(&*c, tokens, next)? {
            children.push(n);
        }
    }
    Ok(Some(Node::Rule(RuleNode {
        rule,
        children,
        start_tok,
        stop_tok: next.saturating_sub(1),
    })))
}

#[allow(dead_code)]
fn _assert_ctx_type(_: PSParserContextType) {}

impl RuleNode {
    pub fn name(&self) -> &'static str {
        RULE_NAMES[self.rule]
    }

    /// LISP-style dump using rule names and token texts (like ANTLR toStringTree).
    pub fn dump(&self) -> String {
        let mut s = String::new();
        self.dump_into(&mut s);
        s
    }

    fn dump_into(&self, s: &mut String) {
        s.push('(');
        s.push_str(self.name());
        for c in &self.children {
            s.push(' ');
            match c {
                Node::Rule(r) => r.dump_into(s),
                Node::Leaf(t) => s.push_str(&t.text),
            }
        }
        s.push(')');
    }
}
