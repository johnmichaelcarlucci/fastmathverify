// Generated from PS.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::pslistener::*;
use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

const _: () = {
    assert!(
        antlr4rust::const_check::version("0", "6"),
        "Incompatible version: either switch to v0.6 or regenerate with this version"
    );
};

		pub const PS_T__0:i32=1; 
		pub const PS_T__1:i32=2; 
		pub const PS_T__2:i32=3; 
		pub const PS_T__3:i32=4; 
		pub const PS_T__4:i32=5; 
		pub const PS_T__5:i32=6; 
		pub const PS_T__6:i32=7; 
		pub const PS_T__7:i32=8; 
		pub const PS_T__8:i32=9; 
		pub const PS_T__9:i32=10; 
		pub const PS_T__10:i32=11; 
		pub const PS_T__11:i32=12; 
		pub const PS_WS:i32=13; 
		pub const PS_THINSPACE:i32=14; 
		pub const PS_MEDSPACE:i32=15; 
		pub const PS_THICKSPACE:i32=16; 
		pub const PS_QUAD:i32=17; 
		pub const PS_QQUAD:i32=18; 
		pub const PS_NEGTHINSPACE:i32=19; 
		pub const PS_NEGMEDSPACE:i32=20; 
		pub const PS_NEGTHICKSPACE:i32=21; 
		pub const PS_DOLLAR_SIGN:i32=22; 
		pub const PS_IGNORE:i32=23; 
		pub const PS_ADD:i32=24; 
		pub const PS_SUB:i32=25; 
		pub const PS_MUL:i32=26; 
		pub const PS_DIV:i32=27; 
		pub const PS_L_PAREN:i32=28; 
		pub const PS_R_PAREN:i32=29; 
		pub const PS_L_PAREN_VISUAL:i32=30; 
		pub const PS_R_PAREN_VISUAL:i32=31; 
		pub const PS_L_GROUP:i32=32; 
		pub const PS_R_GROUP:i32=33; 
		pub const PS_L_BRACE:i32=34; 
		pub const PS_R_BRACE:i32=35; 
		pub const PS_L_BRACE_VISUAL:i32=36; 
		pub const PS_R_BRACE_VISUAL:i32=37; 
		pub const PS_L_BRACE_CMD:i32=38; 
		pub const PS_R_BRACE_CMD:i32=39; 
		pub const PS_L_BRACKET:i32=40; 
		pub const PS_R_BRACKET:i32=41; 
		pub const PS_L_BRACK:i32=42; 
		pub const PS_R_BRACK:i32=43; 
		pub const PS_PHANTOM_CMD:i32=44; 
		pub const PS_BOXED_CMD:i32=45; 
		pub const PS_BAR:i32=46; 
		pub const PS_L_VERT:i32=47; 
		pub const PS_R_VERT:i32=48; 
		pub const PS_VERT:i32=49; 
		pub const PS_NORM:i32=50; 
		pub const PS_L_ANGLE:i32=51; 
		pub const PS_R_ANGLE:i32=52; 
		pub const PS_L_FLOOR:i32=53; 
		pub const PS_R_FLOOR:i32=54; 
		pub const PS_LL_CORNER:i32=55; 
		pub const PS_LR_CORNER:i32=56; 
		pub const PS_L_CEIL:i32=57; 
		pub const PS_R_CEIL:i32=58; 
		pub const PS_UL_CORNER:i32=59; 
		pub const PS_UR_CORNER:i32=60; 
		pub const PS_FUNC_LIM:i32=61; 
		pub const PS_LIM_APPROACH_SYM:i32=62; 
		pub const PS_FUNC_INT:i32=63; 
		pub const PS_FUNC_SUM:i32=64; 
		pub const PS_FUNC_PROD:i32=65; 
		pub const PS_FUNC_LOG:i32=66; 
		pub const PS_FUNC_LN:i32=67; 
		pub const PS_FUNC_EXP:i32=68; 
		pub const PS_FUNC_SIN:i32=69; 
		pub const PS_FUNC_COS:i32=70; 
		pub const PS_FUNC_TAN:i32=71; 
		pub const PS_FUNC_CSC:i32=72; 
		pub const PS_FUNC_SEC:i32=73; 
		pub const PS_FUNC_COT:i32=74; 
		pub const PS_FUNC_GAMMA:i32=75; 
		pub const PS_FUNC_ARCSIN:i32=76; 
		pub const PS_FUNC_ARCCOS:i32=77; 
		pub const PS_FUNC_ARCTAN:i32=78; 
		pub const PS_FUNC_ARCCSC:i32=79; 
		pub const PS_FUNC_ARCSEC:i32=80; 
		pub const PS_FUNC_ARCCOT:i32=81; 
		pub const PS_FUNC_SINH:i32=82; 
		pub const PS_FUNC_COSH:i32=83; 
		pub const PS_FUNC_TANH:i32=84; 
		pub const PS_FUNC_ARSINH:i32=85; 
		pub const PS_FUNC_ARCOSH:i32=86; 
		pub const PS_FUNC_ARTANH:i32=87; 
		pub const PS_FUNC_ARCSINH:i32=88; 
		pub const PS_FUNC_ARCCOSH:i32=89; 
		pub const PS_FUNC_ARCTANH:i32=90; 
		pub const PS_FUNC_ARSINH_NAME:i32=91; 
		pub const PS_FUNC_ARCSINH_NAME:i32=92; 
		pub const PS_FUNC_ARCOSH_NAME:i32=93; 
		pub const PS_FUNC_ARCCOSH_NAME:i32=94; 
		pub const PS_FUNC_ARTANH_NAME:i32=95; 
		pub const PS_FUNC_ARCTANH_NAME:i32=96; 
		pub const PS_FUNC_GCD_NAME:i32=97; 
		pub const PS_FUNC_LCM_NAME:i32=98; 
		pub const PS_FUNC_FLOOR_NAME:i32=99; 
		pub const PS_FUNC_CEIL_NAME:i32=100; 
		pub const PS_FUNC_SQRT:i32=101; 
		pub const PS_FUNC_GCD:i32=102; 
		pub const PS_FUNC_LCM:i32=103; 
		pub const PS_FUNC_FLOOR:i32=104; 
		pub const PS_FUNC_CEIL:i32=105; 
		pub const PS_FUNC_MAX:i32=106; 
		pub const PS_FUNC_MIN:i32=107; 
		pub const PS_FUNC_DET:i32=108; 
		pub const PS_FUNC_EYE_NAME:i32=109; 
		pub const PS_FUNC_ZEROS_NAME:i32=110; 
		pub const PS_FUNC_ONES_NAME:i32=111; 
		pub const PS_FUNC_COLS_NAME:i32=112; 
		pub const PS_FUNC_ROWS_NAME:i32=113; 
		pub const PS_FUNC_DIAG_NAME:i32=114; 
		pub const PS_FUNC_NORM_NAME:i32=115; 
		pub const PS_FUNC_RANK_NAME:i32=116; 
		pub const PS_FUNC_TRACE_NAME:i32=117; 
		pub const PS_FUNC_RREF_NAME:i32=118; 
		pub const PS_FUNC_HSTACK_NAME:i32=119; 
		pub const PS_FUNC_VSTACK_NAME:i32=120; 
		pub const PS_FUNC_ORTHOGONALIZE_NAME:i32=121; 
		pub const PS_FUNC_NULLSPACE_NAME:i32=122; 
		pub const PS_FUNC_DIAGONALIZE_NAME:i32=123; 
		pub const PS_FUNC_EIGENVALS_NAME:i32=124; 
		pub const PS_FUNC_EIGENVECTORS_NAME:i32=125; 
		pub const PS_FUNC_SVD_NAME:i32=126; 
		pub const PS_CMD_TIMES:i32=127; 
		pub const PS_CMD_CDOT:i32=128; 
		pub const PS_CMD_DIV:i32=129; 
		pub const PS_CMD_FRAC:i32=130; 
		pub const PS_CMD_BINOM:i32=131; 
		pub const PS_CMD_CHOOSE:i32=132; 
		pub const PS_CMD_MOD:i32=133; 
		pub const PS_CMD_MATHIT:i32=134; 
		pub const PS_CMD_OPERATORNAME:i32=135; 
		pub const PS_MATRIX_TYPE_MATRIX:i32=136; 
		pub const PS_MATRIX_TYPE_PMATRIX:i32=137; 
		pub const PS_MATRIX_TYPE_BMATRIX:i32=138; 
		pub const PS_MATRIX_TYPE_DET:i32=139; 
		pub const PS_MATRIX_TYPES:i32=140; 
		pub const PS_CMD_MATRIX_START:i32=141; 
		pub const PS_CMD_MATRIX_END:i32=142; 
		pub const PS_CMD_ARRAY_START:i32=143; 
		pub const PS_CMD_ARRAY_END:i32=144; 
		pub const PS_CMD_DET_START:i32=145; 
		pub const PS_CMD_DET_END:i32=146; 
		pub const PS_MATRIX_DEL_COL:i32=147; 
		pub const PS_MATRIX_DEL_ROW:i32=148; 
		pub const PS_UNDERSCORE:i32=149; 
		pub const PS_CARET:i32=150; 
		pub const PS_COLON:i32=151; 
		pub const PS_SEMICOLON:i32=152; 
		pub const PS_COMMA:i32=153; 
		pub const PS_PERIOD:i32=154; 
		pub const PS_DIFFERENTIAL:i32=155; 
		pub const PS_EXP_E:i32=156; 
		pub const PS_E_NOTATION_E:i32=157; 
		pub const PS_LETTER_NO_E:i32=158; 
		pub const PS_MATRIX_XRIGHTARROW:i32=159; 
		pub const PS_TRANSFORM_EXCHANGE:i32=160; 
		pub const PS_NUMBER:i32=161; 
		pub const PS_E_NOTATION:i32=162; 
		pub const PS_IN:i32=163; 
		pub const PS_ASSIGNMENT:i32=164; 
		pub const PS_EQUAL:i32=165; 
		pub const PS_APPROX:i32=166; 
		pub const PS_LT:i32=167; 
		pub const PS_LTE:i32=168; 
		pub const PS_GT:i32=169; 
		pub const PS_GTE:i32=170; 
		pub const PS_UNEQUAL:i32=171; 
		pub const PS_BANG:i32=172; 
		pub const PS_PERCENT_NUMBER:i32=173; 
		pub const PS_GREEK_CMD:i32=174; 
		pub const PS_OTHER_SYMBOL_CMD:i32=175; 
		pub const PS_SYMBOL:i32=176; 
		pub const PS_VARIABLE:i32=177; 
		pub const PS_SET_NATURALS:i32=178; 
		pub const PS_SET_INTEGERS:i32=179; 
		pub const PS_SET_RATIONALS:i32=180; 
		pub const PS_SET_REALS:i32=181; 
		pub const PS_SET_COMPLEX:i32=182; 
		pub const PS_SET_PRIMES:i32=183; 
		pub const PS_ACCENT:i32=184; 
		pub const PS_UNION:i32=185; 
		pub const PS_INTERSECTION:i32=186; 
		pub const PS_SET_MINUS:i32=187; 
		pub const PS_PLUS_MINUS:i32=188; 
		pub const PS_SET_EMPTY:i32=189; 
		pub const PS_SUPSET:i32=190; 
		pub const PS_SUBSET:i32=191; 
		pub const PS_NOTIN:i32=192;
	pub const PS_EOF:i32=EOF;
	pub const RULE_math:usize = 0; 
	pub const RULE_transpose:usize = 1; 
	pub const RULE_degree:usize = 2; 
	pub const RULE_transform_atom:usize = 3; 
	pub const RULE_transform_scale:usize = 4; 
	pub const RULE_transform_swap:usize = 5; 
	pub const RULE_transform_assignment:usize = 6; 
	pub const RULE_elementary_transform:usize = 7; 
	pub const RULE_elementary_transforms:usize = 8; 
	pub const RULE_matrix:usize = 9; 
	pub const RULE_det:usize = 10; 
	pub const RULE_matrix_row:usize = 11; 
	pub const RULE_relation:usize = 12; 
	pub const RULE_equality:usize = 13; 
	pub const RULE_expr:usize = 14; 
	pub const RULE_additive:usize = 15; 
	pub const RULE_mp:usize = 16; 
	pub const RULE_mp_nofunc:usize = 17; 
	pub const RULE_unary:usize = 18; 
	pub const RULE_unary_nofunc:usize = 19; 
	pub const RULE_postfix:usize = 20; 
	pub const RULE_postfix_nofunc:usize = 21; 
	pub const RULE_postfix_op:usize = 22; 
	pub const RULE_eval_at:usize = 23; 
	pub const RULE_eval_at_sub:usize = 24; 
	pub const RULE_eval_at_sup:usize = 25; 
	pub const RULE_exp:usize = 26; 
	pub const RULE_exp_nofunc:usize = 27; 
	pub const RULE_comp:usize = 28; 
	pub const RULE_comp_nofunc:usize = 29; 
	pub const RULE_group:usize = 30; 
	pub const RULE_formatting_group:usize = 31; 
	pub const RULE_norm_group:usize = 32; 
	pub const RULE_abs_group:usize = 33; 
	pub const RULE_dot_product:usize = 34; 
	pub const RULE_floor_group:usize = 35; 
	pub const RULE_ceil_group:usize = 36; 
	pub const RULE_atom_expr_no_supexpr:usize = 37; 
	pub const RULE_atom_expr:usize = 38; 
	pub const RULE_atom_expr_list:usize = 39; 
	pub const RULE_number_subexpr:usize = 40; 
	pub const RULE_atom:usize = 41; 
	pub const RULE_frac:usize = 42; 
	pub const RULE_binom:usize = 43; 
	pub const RULE_func_normal_functions_single_arg:usize = 44; 
	pub const RULE_func_normal_functions_multi_arg:usize = 45; 
	pub const RULE_func_operator_names_single_arg:usize = 46; 
	pub const RULE_func_operator_names_multi_arg:usize = 47; 
	pub const RULE_func_normal_single_arg:usize = 48; 
	pub const RULE_func_normal_multi_arg:usize = 49; 
	pub const RULE_func:usize = 50; 
	pub const RULE_args:usize = 51; 
	pub const RULE_func_common_args:usize = 52; 
	pub const RULE_limit_sub:usize = 53; 
	pub const RULE_func_single_arg:usize = 54; 
	pub const RULE_func_single_arg_noparens:usize = 55; 
	pub const RULE_func_multi_arg:usize = 56; 
	pub const RULE_func_multi_arg_noparens:usize = 57; 
	pub const RULE_subexpr:usize = 58; 
	pub const RULE_supexpr:usize = 59; 
	pub const RULE_subeq:usize = 60; 
	pub const RULE_supeq:usize = 61; 
	pub const RULE_set_relation:usize = 62; 
	pub const RULE_minus_expr:usize = 63; 
	pub const RULE_union_expr:usize = 64; 
	pub const RULE_intersection_expr:usize = 65; 
	pub const RULE_set_group:usize = 66; 
	pub const RULE_set_atom:usize = 67; 
	pub const RULE_interval:usize = 68; 
	pub const RULE_ordered_tuple:usize = 69; 
	pub const RULE_finite_set:usize = 70; 
	pub const RULE_set_elements_relation:usize = 71; 
	pub const RULE_set_elements:usize = 72; 
	pub const RULE_semicolon_elements:usize = 73; 
	pub const RULE_semicolon_elements_no_relation:usize = 74; 
	pub const RULE_comma_elements:usize = 75; 
	pub const RULE_comma_elements_no_relation:usize = 76; 
	pub const RULE_element_no_relation:usize = 77; 
	pub const RULE_element:usize = 78; 
	pub const RULE_plus_minus_expr:usize = 79; 
	pub const RULE_literal_set:usize = 80;
	pub const ruleNames: [&'static str; 81] =  [
		"math", "transpose", "degree", "transform_atom", "transform_scale", "transform_swap", 
		"transform_assignment", "elementary_transform", "elementary_transforms", 
		"matrix", "det", "matrix_row", "relation", "equality", "expr", "additive", 
		"mp", "mp_nofunc", "unary", "unary_nofunc", "postfix", "postfix_nofunc", 
		"postfix_op", "eval_at", "eval_at_sub", "eval_at_sup", "exp", "exp_nofunc", 
		"comp", "comp_nofunc", "group", "formatting_group", "norm_group", "abs_group", 
		"dot_product", "floor_group", "ceil_group", "atom_expr_no_supexpr", "atom_expr", 
		"atom_expr_list", "number_subexpr", "atom", "frac", "binom", "func_normal_functions_single_arg", 
		"func_normal_functions_multi_arg", "func_operator_names_single_arg", "func_operator_names_multi_arg", 
		"func_normal_single_arg", "func_normal_multi_arg", "func", "args", "func_common_args", 
		"limit_sub", "func_single_arg", "func_single_arg_noparens", "func_multi_arg", 
		"func_multi_arg_noparens", "subexpr", "supexpr", "subeq", "supeq", "set_relation", 
		"minus_expr", "union_expr", "intersection_expr", "set_group", "set_atom", 
		"interval", "ordered_tuple", "finite_set", "set_elements_relation", "set_elements", 
		"semicolon_elements", "semicolon_elements_no_relation", "comma_elements", 
		"comma_elements_no_relation", "element_no_relation", "element", "plus_minus_expr", 
		"literal_set"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;173] = [
		None, Some("'^T'"), Some("'^{T}'"), Some("'^{\\\\top}'"), Some("'''"), 
		Some("'^\\circ'"), Some("'^\\degree'"), Some("'^\\circle'"), Some("'^\\u00B0'"), 
		Some("'^{\\circ}'"), Some("'^{\\degree}'"), Some("'^{\\circle}'"), Some("'^{\\u00B0}'"), 
		None, None, None, None, Some("'\\quad'"), Some("'\\qquad'"), None, Some("'\\negmedspace'"), 
		Some("'\\negthickspace'"), Some("'\\$'"), None, None, None, None, None, 
		Some("'('"), Some("')'"), Some("'\\('"), Some("'\\)'"), Some("'\\lgroup'"), 
		Some("'\\rgroup'"), Some("'{'"), Some("'}'"), Some("'\\{'"), Some("'\\}'"), 
		Some("'\\lbrace'"), Some("'\\rbrace'"), Some("'['"), Some("']'"), Some("'\\lbrack'"), 
		Some("'\\rbrack'"), Some("'\\phantom'"), Some("'\\boxed'"), Some("'|'"), 
		Some("'\\lvert'"), Some("'\\rvert'"), Some("'\\vert'"), Some("'\\|'"), 
		Some("'\\langle'"), Some("'\\rangle'"), Some("'\\lfloor'"), Some("'\\rfloor'"), 
		Some("'\\llcorner'"), Some("'\\lrcorner'"), Some("'\\lceil'"), Some("'\\rceil'"), 
		Some("'\\ulcorner'"), Some("'\\urcorner'"), Some("'\\lim'"), None, Some("'\\int'"), 
		Some("'\\sum'"), Some("'\\prod'"), Some("'\\log'"), Some("'\\ln'"), Some("'\\exp'"), 
		Some("'\\sin'"), Some("'\\cos'"), Some("'\\tan'"), Some("'\\csc'"), Some("'\\sec'"), 
		Some("'\\cot'"), None, Some("'\\arcsin'"), Some("'\\arccos'"), Some("'\\arctan'"), 
		Some("'\\arccsc'"), Some("'\\arcsec'"), Some("'\\arccot'"), Some("'\\sinh'"), 
		Some("'\\cosh'"), Some("'\\tanh'"), Some("'\\arsinh'"), Some("'\\arcosh'"), 
		Some("'\\artanh'"), Some("'\\arcsinh'"), Some("'\\arccosh'"), Some("'\\arctanh'"), 
		Some("'arsinh'"), Some("'arcsinh'"), Some("'arcosh'"), Some("'arccosh'"), 
		Some("'artanh'"), Some("'arctanh'"), Some("'gcd'"), Some("'lcm'"), Some("'floor'"), 
		Some("'ceil'"), Some("'\\sqrt'"), Some("'\\gcd'"), Some("'\\lcm'"), Some("'\\floor'"), 
		Some("'\\ceil'"), Some("'\\max'"), Some("'\\min'"), Some("'\\det'"), Some("'eye'"), 
		Some("'zeros'"), Some("'ones'"), Some("'cols'"), Some("'rows'"), Some("'diag'"), 
		Some("'norm'"), Some("'rank'"), None, Some("'rref'"), Some("'hstack'"), 
		Some("'vstack'"), None, Some("'nullspace'"), None, None, None, None, Some("'\\times'"), 
		Some("'\\cdot'"), Some("'\\div'"), None, None, Some("'\\choose'"), Some("'\\mod'"), 
		Some("'\\mathit'"), Some("'\\operatorname'"), Some("'matrix'"), Some("'pmatrix'"), 
		Some("'bmatrix'"), Some("'vmatrix'"), None, None, None, None, None, None, 
		None, Some("'&'"), Some("'\\\\'"), Some("'_'"), Some("'^'"), Some("':'"), 
		Some("';'"), Some("','"), Some("'.'"), None, None, Some("'E'"), None, 
		None, None, None, None, Some("'\\in'"), None, None, None, None, None, 
		None, None, None, Some("'!'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;193]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, Some("WS"), Some("THINSPACE"), Some("MEDSPACE"), Some("THICKSPACE"), 
		Some("QUAD"), Some("QQUAD"), Some("NEGTHINSPACE"), Some("NEGMEDSPACE"), 
		Some("NEGTHICKSPACE"), Some("DOLLAR_SIGN"), Some("IGNORE"), Some("ADD"), 
		Some("SUB"), Some("MUL"), Some("DIV"), Some("L_PAREN"), Some("R_PAREN"), 
		Some("L_PAREN_VISUAL"), Some("R_PAREN_VISUAL"), Some("L_GROUP"), Some("R_GROUP"), 
		Some("L_BRACE"), Some("R_BRACE"), Some("L_BRACE_VISUAL"), Some("R_BRACE_VISUAL"), 
		Some("L_BRACE_CMD"), Some("R_BRACE_CMD"), Some("L_BRACKET"), Some("R_BRACKET"), 
		Some("L_BRACK"), Some("R_BRACK"), Some("PHANTOM_CMD"), Some("BOXED_CMD"), 
		Some("BAR"), Some("L_VERT"), Some("R_VERT"), Some("VERT"), Some("NORM"), 
		Some("L_ANGLE"), Some("R_ANGLE"), Some("L_FLOOR"), Some("R_FLOOR"), Some("LL_CORNER"), 
		Some("LR_CORNER"), Some("L_CEIL"), Some("R_CEIL"), Some("UL_CORNER"), 
		Some("UR_CORNER"), Some("FUNC_LIM"), Some("LIM_APPROACH_SYM"), Some("FUNC_INT"), 
		Some("FUNC_SUM"), Some("FUNC_PROD"), Some("FUNC_LOG"), Some("FUNC_LN"), 
		Some("FUNC_EXP"), Some("FUNC_SIN"), Some("FUNC_COS"), Some("FUNC_TAN"), 
		Some("FUNC_CSC"), Some("FUNC_SEC"), Some("FUNC_COT"), Some("FUNC_GAMMA"), 
		Some("FUNC_ARCSIN"), Some("FUNC_ARCCOS"), Some("FUNC_ARCTAN"), Some("FUNC_ARCCSC"), 
		Some("FUNC_ARCSEC"), Some("FUNC_ARCCOT"), Some("FUNC_SINH"), Some("FUNC_COSH"), 
		Some("FUNC_TANH"), Some("FUNC_ARSINH"), Some("FUNC_ARCOSH"), Some("FUNC_ARTANH"), 
		Some("FUNC_ARCSINH"), Some("FUNC_ARCCOSH"), Some("FUNC_ARCTANH"), Some("FUNC_ARSINH_NAME"), 
		Some("FUNC_ARCSINH_NAME"), Some("FUNC_ARCOSH_NAME"), Some("FUNC_ARCCOSH_NAME"), 
		Some("FUNC_ARTANH_NAME"), Some("FUNC_ARCTANH_NAME"), Some("FUNC_GCD_NAME"), 
		Some("FUNC_LCM_NAME"), Some("FUNC_FLOOR_NAME"), Some("FUNC_CEIL_NAME"), 
		Some("FUNC_SQRT"), Some("FUNC_GCD"), Some("FUNC_LCM"), Some("FUNC_FLOOR"), 
		Some("FUNC_CEIL"), Some("FUNC_MAX"), Some("FUNC_MIN"), Some("FUNC_DET"), 
		Some("FUNC_EYE_NAME"), Some("FUNC_ZEROS_NAME"), Some("FUNC_ONES_NAME"), 
		Some("FUNC_COLS_NAME"), Some("FUNC_ROWS_NAME"), Some("FUNC_DIAG_NAME"), 
		Some("FUNC_NORM_NAME"), Some("FUNC_RANK_NAME"), Some("FUNC_TRACE_NAME"), 
		Some("FUNC_RREF_NAME"), Some("FUNC_HSTACK_NAME"), Some("FUNC_VSTACK_NAME"), 
		Some("FUNC_ORTHOGONALIZE_NAME"), Some("FUNC_NULLSPACE_NAME"), Some("FUNC_DIAGONALIZE_NAME"), 
		Some("FUNC_EIGENVALS_NAME"), Some("FUNC_EIGENVECTORS_NAME"), Some("FUNC_SVD_NAME"), 
		Some("CMD_TIMES"), Some("CMD_CDOT"), Some("CMD_DIV"), Some("CMD_FRAC"), 
		Some("CMD_BINOM"), Some("CMD_CHOOSE"), Some("CMD_MOD"), Some("CMD_MATHIT"), 
		Some("CMD_OPERATORNAME"), Some("MATRIX_TYPE_MATRIX"), Some("MATRIX_TYPE_PMATRIX"), 
		Some("MATRIX_TYPE_BMATRIX"), Some("MATRIX_TYPE_DET"), Some("MATRIX_TYPES"), 
		Some("CMD_MATRIX_START"), Some("CMD_MATRIX_END"), Some("CMD_ARRAY_START"), 
		Some("CMD_ARRAY_END"), Some("CMD_DET_START"), Some("CMD_DET_END"), Some("MATRIX_DEL_COL"), 
		Some("MATRIX_DEL_ROW"), Some("UNDERSCORE"), Some("CARET"), Some("COLON"), 
		Some("SEMICOLON"), Some("COMMA"), Some("PERIOD"), Some("DIFFERENTIAL"), 
		Some("EXP_E"), Some("E_NOTATION_E"), Some("LETTER_NO_E"), Some("MATRIX_XRIGHTARROW"), 
		Some("TRANSFORM_EXCHANGE"), Some("NUMBER"), Some("E_NOTATION"), Some("IN"), 
		Some("ASSIGNMENT"), Some("EQUAL"), Some("APPROX"), Some("LT"), Some("LTE"), 
		Some("GT"), Some("GTE"), Some("UNEQUAL"), Some("BANG"), Some("PERCENT_NUMBER"), 
		Some("GREEK_CMD"), Some("OTHER_SYMBOL_CMD"), Some("SYMBOL"), Some("VARIABLE"), 
		Some("SET_NATURALS"), Some("SET_INTEGERS"), Some("SET_RATIONALS"), Some("SET_REALS"), 
		Some("SET_COMPLEX"), Some("SET_PRIMES"), Some("ACCENT"), Some("UNION"), 
		Some("INTERSECTION"), Some("SET_MINUS"), Some("PLUS_MINUS"), Some("SET_EMPTY"), 
		Some("SUPSET"), Some("SUBSET"), Some("NOTIN")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,PSParserExt<'input>, I, PSParserContextType , dyn PSListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type PSTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, PSParserContextType , dyn PSListener<'input> + 'a>;

/// Parser for PS grammar
pub struct PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Rc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		let interpreter = Rc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Rc::clone(&interpreter),
				PSParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for PSParser
pub trait PSParserContext<'input>:
	for<'x> Listenable<dyn PSListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=PSParserContextType>
{}

antlr4rust::coerce_from!{ 'input : PSParserContext<'input> }

impl<'input> PSParserContext<'input> for TerminalNode<'input,PSParserContextType> {}
impl<'input> PSParserContext<'input> for ErrorNode<'input,PSParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn PSParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn PSListener<'input> + 'input }

pub struct PSParserContextType;
antlr4rust::tid!{PSParserContextType}

impl<'input> ParserNodeType<'input> for PSParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn PSParserContext<'input> + 'input;
}

impl<'input, I> Deref for PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct PSParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> PSParserExt<'input>{
}
antlr4rust::tid! { PSParserExt<'a> }

impl<'input> TokenAware<'input> for PSParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for PSParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for PSParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "PS.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn PSParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					12 => PSParser::<'input,I>::relation_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					15 => PSParser::<'input,I>::additive_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					16 => PSParser::<'input,I>::mp_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					17 => PSParser::<'input,I>::mp_nofunc_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					26 => PSParser::<'input,I>::exp_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					27 => PSParser::<'input,I>::exp_nofunc_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					62 => PSParser::<'input,I>::set_relation_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					63 => PSParser::<'input,I>::minus_expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					64 => PSParser::<'input,I>::union_expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					65 => PSParser::<'input,I>::intersection_expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn relation_sempred(_localctx: Option<&RelationContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn additive_sempred(_localctx: Option<&AdditiveContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				1=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn mp_sempred(_localctx: Option<&MpContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				2=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn mp_nofunc_sempred(_localctx: Option<&Mp_nofuncContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				3=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn exp_sempred(_localctx: Option<&ExpContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				4=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn exp_nofunc_sempred(_localctx: Option<&Exp_nofuncContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				5=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn set_relation_sempred(_localctx: Option<&Set_relationContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				6=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
	fn minus_expr_sempred(_localctx: Option<&Minus_exprContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				7=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn union_expr_sempred(_localctx: Option<&Union_exprContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				8=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn intersection_expr_sempred(_localctx: Option<&Intersection_exprContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				9=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
}
//------------------- math ----------------
pub type MathContextAll<'input> = MathContext<'input>;


pub type MathContext<'input> = BaseParserRuleContext<'input,MathContextExt<'input>>;

#[derive(Clone)]
pub struct MathContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for MathContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for MathContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_math(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_math(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for MathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_math }
	//fn type_rule_index() -> usize where Self: Sized { RULE_math }
}
antlr4rust::tid!{MathContextExt<'a>}

impl<'input> MathContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MathContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MathContextAttrs<'input>: PSParserContext<'input> + BorrowMut<MathContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_EOF, 0)
}
fn set_elements_relation(&self) -> Option<Rc<Set_elements_relationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn set_elements(&self) -> Option<Rc<Set_elementsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn set_relation(&self) -> Option<Rc<Set_relationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MathContextAttrs<'input> for MathContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn math(&mut self,)
	-> Result<Rc<MathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_math);
        let mut _localctx: Rc<MathContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(165);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule set_elements_relation*/
					recog.base.set_state(162);
					recog.set_elements_relation()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule set_elements*/
					recog.base.set_state(163);
					recog.set_elements()?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule set_relation*/
					recog.base.set_state(164);
					recog.set_relation_rec(0)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(167);
			recog.base.match_token(PS_EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- transpose ----------------
pub type TransposeContextAll<'input> = TransposeContext<'input>;


pub type TransposeContext<'input> = BaseParserRuleContext<'input,TransposeContextExt<'input>>;

#[derive(Clone)]
pub struct TransposeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for TransposeContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for TransposeContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_transpose(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_transpose(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TransposeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transpose }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transpose }
}
antlr4rust::tid!{TransposeContextExt<'a>}

impl<'input> TransposeContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TransposeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransposeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TransposeContextAttrs<'input>: PSParserContext<'input> + BorrowMut<TransposeContextExt<'input>>{


}

impl<'input> TransposeContextAttrs<'input> for TransposeContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn transpose(&mut self,)
	-> Result<Rc<TransposeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransposeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_transpose);
        let mut _localctx: Rc<TransposeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(169);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 30) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- degree ----------------
pub type DegreeContextAll<'input> = DegreeContext<'input>;


pub type DegreeContext<'input> = BaseParserRuleContext<'input,DegreeContextExt<'input>>;

#[derive(Clone)]
pub struct DegreeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for DegreeContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for DegreeContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_degree(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_degree(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DegreeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_degree }
	//fn type_rule_index() -> usize where Self: Sized { RULE_degree }
}
antlr4rust::tid!{DegreeContextExt<'a>}

impl<'input> DegreeContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DegreeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DegreeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DegreeContextAttrs<'input>: PSParserContext<'input> + BorrowMut<DegreeContextExt<'input>>{


}

impl<'input> DegreeContextAttrs<'input> for DegreeContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn degree(&mut self,)
	-> Result<Rc<DegreeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DegreeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_degree);
        let mut _localctx: Rc<DegreeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(171);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 8160) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- transform_atom ----------------
pub type Transform_atomContextAll<'input> = Transform_atomContext<'input>;


pub type Transform_atomContext<'input> = BaseParserRuleContext<'input,Transform_atomContextExt<'input>>;

#[derive(Clone)]
pub struct Transform_atomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Transform_atomContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Transform_atomContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_transform_atom(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_transform_atom(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Transform_atomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transform_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transform_atom }
}
antlr4rust::tid!{Transform_atomContextExt<'a>}

impl<'input> Transform_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Transform_atomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Transform_atomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Transform_atomContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Transform_atomContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LETTER_NO_E
/// Returns `None` if there is no child corresponding to token LETTER_NO_E
fn LETTER_NO_E(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_LETTER_NO_E, 0)
}
/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UNDERSCORE, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}

}

impl<'input> Transform_atomContextAttrs<'input> for Transform_atomContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn transform_atom(&mut self,)
	-> Result<Rc<Transform_atomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Transform_atomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_transform_atom);
        let mut _localctx: Rc<Transform_atomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(173);
			recog.base.match_token(PS_LETTER_NO_E,&mut recog.err_handler)?;

			recog.base.set_state(174);
			recog.base.match_token(PS_UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(179);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_NUMBER 
				=> {
					{
					recog.base.set_state(175);
					recog.base.match_token(PS_NUMBER,&mut recog.err_handler)?;

					}
				}

			PS_L_BRACE 
				=> {
					{
					recog.base.set_state(176);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					recog.base.set_state(177);
					recog.base.match_token(PS_NUMBER,&mut recog.err_handler)?;

					recog.base.set_state(178);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- transform_scale ----------------
pub type Transform_scaleContextAll<'input> = Transform_scaleContext<'input>;


pub type Transform_scaleContext<'input> = BaseParserRuleContext<'input,Transform_scaleContextExt<'input>>;

#[derive(Clone)]
pub struct Transform_scaleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Transform_scaleContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Transform_scaleContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_transform_scale(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_transform_scale(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Transform_scaleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transform_scale }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transform_scale }
}
antlr4rust::tid!{Transform_scaleContextExt<'a>}

impl<'input> Transform_scaleContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Transform_scaleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Transform_scaleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Transform_scaleContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Transform_scaleContextExt<'input>>{

fn transform_atom(&self) -> Option<Rc<Transform_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn group(&self) -> Option<Rc<GroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SUB, 0)
}

}

impl<'input> Transform_scaleContextAttrs<'input> for Transform_scaleContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn transform_scale(&mut self,)
	-> Result<Rc<Transform_scaleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Transform_scaleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_transform_scale);
        let mut _localctx: Rc<Transform_scaleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(185);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(2,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule expr*/
					recog.base.set_state(181);
					recog.expr()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule group*/
					recog.base.set_state(182);
					recog.group()?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(183);
					recog.base.match_token(PS_ADD,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					recog.base.set_state(184);
					recog.base.match_token(PS_SUB,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule transform_atom*/
			recog.base.set_state(187);
			recog.transform_atom()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- transform_swap ----------------
pub type Transform_swapContextAll<'input> = Transform_swapContext<'input>;


pub type Transform_swapContext<'input> = BaseParserRuleContext<'input,Transform_swapContextExt<'input>>;

#[derive(Clone)]
pub struct Transform_swapContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Transform_swapContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Transform_swapContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_transform_swap(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_transform_swap(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Transform_swapContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transform_swap }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transform_swap }
}
antlr4rust::tid!{Transform_swapContextExt<'a>}

impl<'input> Transform_swapContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Transform_swapContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Transform_swapContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Transform_swapContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Transform_swapContextExt<'input>>{

fn transform_atom_all(&self) ->  Vec<Rc<Transform_atomContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn transform_atom(&self, i: usize) -> Option<Rc<Transform_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token TRANSFORM_EXCHANGE
/// Returns `None` if there is no child corresponding to token TRANSFORM_EXCHANGE
fn TRANSFORM_EXCHANGE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_TRANSFORM_EXCHANGE, 0)
}

}

impl<'input> Transform_swapContextAttrs<'input> for Transform_swapContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn transform_swap(&mut self,)
	-> Result<Rc<Transform_swapContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Transform_swapContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_transform_swap);
        let mut _localctx: Rc<Transform_swapContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule transform_atom*/
			recog.base.set_state(189);
			recog.transform_atom()?;

			recog.base.set_state(190);
			recog.base.match_token(PS_TRANSFORM_EXCHANGE,&mut recog.err_handler)?;

			/*InvokeRule transform_atom*/
			recog.base.set_state(191);
			recog.transform_atom()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- transform_assignment ----------------
pub type Transform_assignmentContextAll<'input> = Transform_assignmentContext<'input>;


pub type Transform_assignmentContext<'input> = BaseParserRuleContext<'input,Transform_assignmentContextExt<'input>>;

#[derive(Clone)]
pub struct Transform_assignmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Transform_assignmentContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Transform_assignmentContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_transform_assignment(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_transform_assignment(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Transform_assignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transform_assignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transform_assignment }
}
antlr4rust::tid!{Transform_assignmentContextExt<'a>}

impl<'input> Transform_assignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Transform_assignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Transform_assignmentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Transform_assignmentContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Transform_assignmentContextExt<'input>>{

fn transform_atom(&self) -> Option<Rc<Transform_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transform_scale(&self) -> Option<Rc<Transform_scaleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Transform_assignmentContextAttrs<'input> for Transform_assignmentContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn transform_assignment(&mut self,)
	-> Result<Rc<Transform_assignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Transform_assignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_transform_assignment);
        let mut _localctx: Rc<Transform_assignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule transform_atom*/
			recog.base.set_state(193);
			recog.transform_atom()?;

			/*InvokeRule transform_scale*/
			recog.base.set_state(194);
			recog.transform_scale()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- elementary_transform ----------------
pub type Elementary_transformContextAll<'input> = Elementary_transformContext<'input>;


pub type Elementary_transformContext<'input> = BaseParserRuleContext<'input,Elementary_transformContextExt<'input>>;

#[derive(Clone)]
pub struct Elementary_transformContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Elementary_transformContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Elementary_transformContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_elementary_transform(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_elementary_transform(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Elementary_transformContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elementary_transform }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elementary_transform }
}
antlr4rust::tid!{Elementary_transformContextExt<'a>}

impl<'input> Elementary_transformContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Elementary_transformContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Elementary_transformContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Elementary_transformContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Elementary_transformContextExt<'input>>{

fn transform_assignment(&self) -> Option<Rc<Transform_assignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transform_scale(&self) -> Option<Rc<Transform_scaleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transform_swap(&self) -> Option<Rc<Transform_swapContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Elementary_transformContextAttrs<'input> for Elementary_transformContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn elementary_transform(&mut self,)
	-> Result<Rc<Elementary_transformContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Elementary_transformContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_elementary_transform);
        let mut _localctx: Rc<Elementary_transformContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(199);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule transform_assignment*/
					recog.base.set_state(196);
					recog.transform_assignment()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule transform_scale*/
					recog.base.set_state(197);
					recog.transform_scale()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule transform_swap*/
					recog.base.set_state(198);
					recog.transform_swap()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- elementary_transforms ----------------
pub type Elementary_transformsContextAll<'input> = Elementary_transformsContext<'input>;


pub type Elementary_transformsContext<'input> = BaseParserRuleContext<'input,Elementary_transformsContextExt<'input>>;

#[derive(Clone)]
pub struct Elementary_transformsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Elementary_transformsContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Elementary_transformsContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_elementary_transforms(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_elementary_transforms(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Elementary_transformsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elementary_transforms }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elementary_transforms }
}
antlr4rust::tid!{Elementary_transformsContextExt<'a>}

impl<'input> Elementary_transformsContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Elementary_transformsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Elementary_transformsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Elementary_transformsContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Elementary_transformsContextExt<'input>>{

fn elementary_transform_all(&self) ->  Vec<Rc<Elementary_transformContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn elementary_transform(&self, i: usize) -> Option<Rc<Elementary_transformContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COMMA, i)
}

}

impl<'input> Elementary_transformsContextAttrs<'input> for Elementary_transformsContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn elementary_transforms(&mut self,)
	-> Result<Rc<Elementary_transformsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Elementary_transformsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_elementary_transforms);
        let mut _localctx: Rc<Elementary_transformsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule elementary_transform*/
			recog.base.set_state(201);
			recog.elementary_transform()?;

			recog.base.set_state(206);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==PS_COMMA {
				{
				{
				recog.base.set_state(202);
				recog.base.match_token(PS_COMMA,&mut recog.err_handler)?;

				/*InvokeRule elementary_transform*/
				recog.base.set_state(203);
				recog.elementary_transform()?;

				}
				}
				recog.base.set_state(208);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- matrix ----------------
pub type MatrixContextAll<'input> = MatrixContext<'input>;


pub type MatrixContext<'input> = BaseParserRuleContext<'input,MatrixContextExt<'input>>;

#[derive(Clone)]
pub struct MatrixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for MatrixContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for MatrixContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_matrix(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_matrix(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for MatrixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_matrix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_matrix }
}
antlr4rust::tid!{MatrixContextExt<'a>}

impl<'input> MatrixContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MatrixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MatrixContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MatrixContextAttrs<'input>: PSParserContext<'input> + BorrowMut<MatrixContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CMD_MATRIX_START
/// Returns `None` if there is no child corresponding to token CMD_MATRIX_START
fn CMD_MATRIX_START(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_MATRIX_START, 0)
}
fn matrix_row_all(&self) ->  Vec<Rc<Matrix_rowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn matrix_row(&self, i: usize) -> Option<Rc<Matrix_rowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token CMD_MATRIX_END
/// Returns `None` if there is no child corresponding to token CMD_MATRIX_END
fn CMD_MATRIX_END(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_MATRIX_END, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_ARRAY_START
/// Returns `None` if there is no child corresponding to token CMD_ARRAY_START
fn CMD_ARRAY_START(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_ARRAY_START, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_ARRAY_END
/// Returns `None` if there is no child corresponding to token CMD_ARRAY_END
fn CMD_ARRAY_END(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_ARRAY_END, 0)
}
/// Retrieves first TerminalNode corresponding to token MATRIX_XRIGHTARROW
/// Returns `None` if there is no child corresponding to token MATRIX_XRIGHTARROW
fn MATRIX_XRIGHTARROW(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_MATRIX_XRIGHTARROW, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
fn elementary_transforms_all(&self) ->  Vec<Rc<Elementary_transformsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn elementary_transforms(&self, i: usize) -> Option<Rc<Elementary_transformsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token MATRIX_DEL_ROW in current rule
fn MATRIX_DEL_ROW_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MATRIX_DEL_ROW, starting from 0.
/// Returns `None` if number of children corresponding to token MATRIX_DEL_ROW is less or equal than `i`.
fn MATRIX_DEL_ROW(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_MATRIX_DEL_ROW, i)
}
/// Retrieves first TerminalNode corresponding to token L_BRACKET
/// Returns `None` if there is no child corresponding to token L_BRACKET
fn L_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACKET
/// Returns `None` if there is no child corresponding to token R_BRACKET
fn R_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACKET, 0)
}

}

impl<'input> MatrixContextAttrs<'input> for MatrixContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn matrix(&mut self,)
	-> Result<Rc<MatrixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MatrixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_matrix);
        let mut _localctx: Rc<MatrixContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(237);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_CMD_MATRIX_START 
				=> {
					{
					recog.base.set_state(209);
					recog.base.match_token(PS_CMD_MATRIX_START,&mut recog.err_handler)?;

					/*InvokeRule matrix_row*/
					recog.base.set_state(210);
					recog.matrix_row()?;

					recog.base.set_state(215);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(211);
							recog.base.match_token(PS_MATRIX_DEL_ROW,&mut recog.err_handler)?;

							/*InvokeRule matrix_row*/
							recog.base.set_state(212);
							recog.matrix_row()?;

							}
							} 
						}
						recog.base.set_state(217);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
					}
					recog.base.set_state(219);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PS_MATRIX_DEL_ROW {
						{
						recog.base.set_state(218);
						recog.base.match_token(PS_MATRIX_DEL_ROW,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(221);
					recog.base.match_token(PS_CMD_MATRIX_END,&mut recog.err_handler)?;

					}
				}

			PS_CMD_ARRAY_START 
				=> {
					{
					recog.base.set_state(223);
					recog.base.match_token(PS_CMD_ARRAY_START,&mut recog.err_handler)?;

					/*InvokeRule matrix_row*/
					recog.base.set_state(224);
					recog.matrix_row()?;

					recog.base.set_state(229);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(225);
							recog.base.match_token(PS_MATRIX_DEL_ROW,&mut recog.err_handler)?;

							/*InvokeRule matrix_row*/
							recog.base.set_state(226);
							recog.matrix_row()?;

							}
							} 
						}
						recog.base.set_state(231);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
					}
					recog.base.set_state(233);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PS_MATRIX_DEL_ROW {
						{
						recog.base.set_state(232);
						recog.base.match_token(PS_MATRIX_DEL_ROW,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(235);
					recog.base.match_token(PS_CMD_ARRAY_END,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(250);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(239);
					recog.base.match_token(PS_MATRIX_XRIGHTARROW,&mut recog.err_handler)?;

					recog.base.set_state(244);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PS_L_BRACKET {
						{
						recog.base.set_state(240);
						recog.base.match_token(PS_L_BRACKET,&mut recog.err_handler)?;

						/*InvokeRule elementary_transforms*/
						recog.base.set_state(241);
						recog.elementary_transforms()?;

						recog.base.set_state(242);
						recog.base.match_token(PS_R_BRACKET,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(246);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule elementary_transforms*/
					recog.base.set_state(247);
					recog.elementary_transforms()?;

					recog.base.set_state(248);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- det ----------------
pub type DetContextAll<'input> = DetContext<'input>;


pub type DetContext<'input> = BaseParserRuleContext<'input,DetContextExt<'input>>;

#[derive(Clone)]
pub struct DetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for DetContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for DetContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_det(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_det(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_det }
	//fn type_rule_index() -> usize where Self: Sized { RULE_det }
}
antlr4rust::tid!{DetContextExt<'a>}

impl<'input> DetContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DetContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DetContextAttrs<'input>: PSParserContext<'input> + BorrowMut<DetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CMD_DET_START
/// Returns `None` if there is no child corresponding to token CMD_DET_START
fn CMD_DET_START(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_DET_START, 0)
}
fn matrix_row_all(&self) ->  Vec<Rc<Matrix_rowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn matrix_row(&self, i: usize) -> Option<Rc<Matrix_rowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token CMD_DET_END
/// Returns `None` if there is no child corresponding to token CMD_DET_END
fn CMD_DET_END(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_DET_END, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token MATRIX_DEL_ROW in current rule
fn MATRIX_DEL_ROW_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MATRIX_DEL_ROW, starting from 0.
/// Returns `None` if number of children corresponding to token MATRIX_DEL_ROW is less or equal than `i`.
fn MATRIX_DEL_ROW(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_MATRIX_DEL_ROW, i)
}

}

impl<'input> DetContextAttrs<'input> for DetContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn det(&mut self,)
	-> Result<Rc<DetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_det);
        let mut _localctx: Rc<DetContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(252);
			recog.base.match_token(PS_CMD_DET_START,&mut recog.err_handler)?;

			/*InvokeRule matrix_row*/
			recog.base.set_state(253);
			recog.matrix_row()?;

			recog.base.set_state(258);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(254);
					recog.base.match_token(PS_MATRIX_DEL_ROW,&mut recog.err_handler)?;

					/*InvokeRule matrix_row*/
					recog.base.set_state(255);
					recog.matrix_row()?;

					}
					} 
				}
				recog.base.set_state(260);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			}
			recog.base.set_state(262);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==PS_MATRIX_DEL_ROW {
				{
				recog.base.set_state(261);
				recog.base.match_token(PS_MATRIX_DEL_ROW,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(264);
			recog.base.match_token(PS_CMD_DET_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- matrix_row ----------------
pub type Matrix_rowContextAll<'input> = Matrix_rowContext<'input>;


pub type Matrix_rowContext<'input> = BaseParserRuleContext<'input,Matrix_rowContextExt<'input>>;

#[derive(Clone)]
pub struct Matrix_rowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Matrix_rowContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Matrix_rowContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_matrix_row(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_matrix_row(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Matrix_rowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_matrix_row }
	//fn type_rule_index() -> usize where Self: Sized { RULE_matrix_row }
}
antlr4rust::tid!{Matrix_rowContextExt<'a>}

impl<'input> Matrix_rowContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Matrix_rowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Matrix_rowContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Matrix_rowContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Matrix_rowContextExt<'input>>{

fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token MATRIX_DEL_COL in current rule
fn MATRIX_DEL_COL_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MATRIX_DEL_COL, starting from 0.
/// Returns `None` if number of children corresponding to token MATRIX_DEL_COL is less or equal than `i`.
fn MATRIX_DEL_COL(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_MATRIX_DEL_COL, i)
}

}

impl<'input> Matrix_rowContextAttrs<'input> for Matrix_rowContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn matrix_row(&mut self,)
	-> Result<Rc<Matrix_rowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Matrix_rowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_matrix_row);
        let mut _localctx: Rc<Matrix_rowContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule expr*/
			recog.base.set_state(266);
			recog.expr()?;

			recog.base.set_state(271);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==PS_MATRIX_DEL_COL {
				{
				{
				recog.base.set_state(267);
				recog.base.match_token(PS_MATRIX_DEL_COL,&mut recog.err_handler)?;

				/*InvokeRule expr*/
				recog.base.set_state(268);
				recog.expr()?;

				}
				}
				recog.base.set_state(273);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- relation ----------------
pub type RelationContextAll<'input> = RelationContext<'input>;


pub type RelationContext<'input> = BaseParserRuleContext<'input,RelationContextExt<'input>>;

#[derive(Clone)]
pub struct RelationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for RelationContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for RelationContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_relation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_relation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for RelationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation }
}
antlr4rust::tid!{RelationContextExt<'a>}

impl<'input> RelationContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RelationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RelationContextAttrs<'input>: PSParserContext<'input> + BorrowMut<RelationContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn relation_all(&self) ->  Vec<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relation(&self, i: usize) -> Option<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_IN, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGNMENT
/// Returns `None` if there is no child corresponding to token ASSIGNMENT
fn ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ASSIGNMENT, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_EQUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token APPROX
/// Returns `None` if there is no child corresponding to token APPROX
fn APPROX(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_APPROX, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_LT, 0)
}
/// Retrieves first TerminalNode corresponding to token LTE
/// Returns `None` if there is no child corresponding to token LTE
fn LTE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_LTE, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_GT, 0)
}
/// Retrieves first TerminalNode corresponding to token GTE
/// Returns `None` if there is no child corresponding to token GTE
fn GTE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_GTE, 0)
}
/// Retrieves first TerminalNode corresponding to token UNEQUAL
/// Returns `None` if there is no child corresponding to token UNEQUAL
fn UNEQUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UNEQUAL, 0)
}

}

impl<'input> RelationContextAttrs<'input> for RelationContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  relation(&mut self,)
	-> Result<Rc<RelationContextAll<'input>>,ANTLRError> {
		self.relation_rec(0)
	}

	fn relation_rec(&mut self, _p: i32)
	-> Result<Rc<RelationContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = RelationContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 24, RULE_relation, _p);
	    let mut _localctx: Rc<RelationContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 24;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule expr*/
			recog.base.set_state(275);
			recog.expr()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(282);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = RelationContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_relation)?;
					_localctx = tmp;
					recog.base.set_state(277);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(278);
					_la = recog.base.input.la(1);
					if { !(((((_la - 163)) & !0x3f) == 0 && ((1usize << (_la - 163)) & 511) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule relation*/
					recog.base.set_state(279);
					recog.relation_rec(3)?;

					}
					} 
				}
				recog.base.set_state(284);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- equality ----------------
pub type EqualityContextAll<'input> = EqualityContext<'input>;


pub type EqualityContext<'input> = BaseParserRuleContext<'input,EqualityContextExt<'input>>;

#[derive(Clone)]
pub struct EqualityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for EqualityContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for EqualityContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_equality(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_equality(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for EqualityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equality }
}
antlr4rust::tid!{EqualityContextExt<'a>}

impl<'input> EqualityContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EqualityContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqualityContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EqualityContextAttrs<'input>: PSParserContext<'input> + BorrowMut<EqualityContextExt<'input>>{

fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_EQUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGNMENT
/// Returns `None` if there is no child corresponding to token ASSIGNMENT
fn ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ASSIGNMENT, 0)
}

}

impl<'input> EqualityContextAttrs<'input> for EqualityContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn equality(&mut self,)
	-> Result<Rc<EqualityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqualityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_equality);
        let mut _localctx: Rc<EqualityContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule expr*/
			recog.base.set_state(285);
			recog.expr()?;

			recog.base.set_state(286);
			_la = recog.base.input.la(1);
			if { !(_la==PS_ASSIGNMENT || _la==PS_EQUAL) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule expr*/
			recog.base.set_state(287);
			recog.expr()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;


pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_expr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_expr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr4rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ExprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<ExprContextExt<'input>>{

fn additive(&self) -> Option<Rc<AdditiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule additive*/
			recog.base.set_state(289);
			recog.additive_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- additive ----------------
pub type AdditiveContextAll<'input> = AdditiveContext<'input>;


pub type AdditiveContext<'input> = BaseParserRuleContext<'input,AdditiveContextExt<'input>>;

#[derive(Clone)]
pub struct AdditiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for AdditiveContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for AdditiveContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_additive(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_additive(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AdditiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additive }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additive }
}
antlr4rust::tid!{AdditiveContextExt<'a>}

impl<'input> AdditiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AdditiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AdditiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AdditiveContextAttrs<'input>: PSParserContext<'input> + BorrowMut<AdditiveContextExt<'input>>{

fn mp(&self) -> Option<Rc<MpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn additive_all(&self) ->  Vec<Rc<AdditiveContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn additive(&self, i: usize) -> Option<Rc<AdditiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SUB, 0)
}

}

impl<'input> AdditiveContextAttrs<'input> for AdditiveContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  additive(&mut self,)
	-> Result<Rc<AdditiveContextAll<'input>>,ANTLRError> {
		self.additive_rec(0)
	}

	fn additive_rec(&mut self, _p: i32)
	-> Result<Rc<AdditiveContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = AdditiveContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 30, RULE_additive, _p);
	    let mut _localctx: Rc<AdditiveContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 30;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule mp*/
			recog.base.set_state(292);
			recog.mp_rec(0)?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(299);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = AdditiveContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_additive)?;
					_localctx = tmp;
					recog.base.set_state(294);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(295);
					_la = recog.base.input.la(1);
					if { !(_la==PS_ADD || _la==PS_SUB) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule additive*/
					recog.base.set_state(296);
					recog.additive_rec(3)?;

					}
					} 
				}
				recog.base.set_state(301);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- mp ----------------
pub type MpContextAll<'input> = MpContext<'input>;


pub type MpContext<'input> = BaseParserRuleContext<'input,MpContextExt<'input>>;

#[derive(Clone)]
pub struct MpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for MpContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for MpContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_mp(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_mp(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for MpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mp }
}
antlr4rust::tid!{MpContextExt<'a>}

impl<'input> MpContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MpContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MpContextAttrs<'input>: PSParserContext<'input> + BorrowMut<MpContextExt<'input>>{

fn unary(&self) -> Option<Rc<UnaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mp_all(&self) ->  Vec<Rc<MpContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mp(&self, i: usize) -> Option<Rc<MpContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token MUL
/// Returns `None` if there is no child corresponding to token MUL
fn MUL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_MUL, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_TIMES
/// Returns `None` if there is no child corresponding to token CMD_TIMES
fn CMD_TIMES(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_TIMES, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_CDOT
/// Returns `None` if there is no child corresponding to token CMD_CDOT
fn CMD_CDOT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_CDOT, 0)
}
/// Retrieves first TerminalNode corresponding to token DIV
/// Returns `None` if there is no child corresponding to token DIV
fn DIV(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_DIV, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_DIV
/// Returns `None` if there is no child corresponding to token CMD_DIV
fn CMD_DIV(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_DIV, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_MOD
/// Returns `None` if there is no child corresponding to token CMD_MOD
fn CMD_MOD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_MOD, 0)
}

}

impl<'input> MpContextAttrs<'input> for MpContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  mp(&mut self,)
	-> Result<Rc<MpContextAll<'input>>,ANTLRError> {
		self.mp_rec(0)
	}

	fn mp_rec(&mut self, _p: i32)
	-> Result<Rc<MpContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = MpContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 32, RULE_mp, _p);
	    let mut _localctx: Rc<MpContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 32;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule unary*/
			recog.base.set_state(303);
			recog.unary()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(310);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = MpContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_mp)?;
					_localctx = tmp;
					recog.base.set_state(305);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(306);
					_la = recog.base.input.la(1);
					if { !(_la==PS_MUL || _la==PS_DIV || ((((_la - 127)) & !0x3f) == 0 && ((1usize << (_la - 127)) & 16777287) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule mp*/
					recog.base.set_state(307);
					recog.mp_rec(3)?;

					}
					} 
				}
				recog.base.set_state(312);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- mp_nofunc ----------------
pub type Mp_nofuncContextAll<'input> = Mp_nofuncContext<'input>;


pub type Mp_nofuncContext<'input> = BaseParserRuleContext<'input,Mp_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Mp_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Mp_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Mp_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_mp_nofunc(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_mp_nofunc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Mp_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mp_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mp_nofunc }
}
antlr4rust::tid!{Mp_nofuncContextExt<'a>}

impl<'input> Mp_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Mp_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mp_nofuncContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Mp_nofuncContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Mp_nofuncContextExt<'input>>{

fn unary_nofunc(&self) -> Option<Rc<Unary_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mp_nofunc_all(&self) ->  Vec<Rc<Mp_nofuncContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mp_nofunc(&self, i: usize) -> Option<Rc<Mp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token MUL
/// Returns `None` if there is no child corresponding to token MUL
fn MUL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_MUL, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_TIMES
/// Returns `None` if there is no child corresponding to token CMD_TIMES
fn CMD_TIMES(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_TIMES, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_CDOT
/// Returns `None` if there is no child corresponding to token CMD_CDOT
fn CMD_CDOT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_CDOT, 0)
}
/// Retrieves first TerminalNode corresponding to token DIV
/// Returns `None` if there is no child corresponding to token DIV
fn DIV(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_DIV, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_DIV
/// Returns `None` if there is no child corresponding to token CMD_DIV
fn CMD_DIV(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_DIV, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_MOD
/// Returns `None` if there is no child corresponding to token CMD_MOD
fn CMD_MOD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_MOD, 0)
}

}

impl<'input> Mp_nofuncContextAttrs<'input> for Mp_nofuncContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  mp_nofunc(&mut self,)
	-> Result<Rc<Mp_nofuncContextAll<'input>>,ANTLRError> {
		self.mp_nofunc_rec(0)
	}

	fn mp_nofunc_rec(&mut self, _p: i32)
	-> Result<Rc<Mp_nofuncContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Mp_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 34, RULE_mp_nofunc, _p);
	    let mut _localctx: Rc<Mp_nofuncContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 34;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule unary_nofunc*/
			recog.base.set_state(314);
			recog.unary_nofunc()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(321);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Mp_nofuncContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_mp_nofunc)?;
					_localctx = tmp;
					recog.base.set_state(316);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(317);
					_la = recog.base.input.la(1);
					if { !(_la==PS_MUL || _la==PS_DIV || ((((_la - 127)) & !0x3f) == 0 && ((1usize << (_la - 127)) & 16777287) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule mp_nofunc*/
					recog.base.set_state(318);
					recog.mp_nofunc_rec(3)?;

					}
					} 
				}
				recog.base.set_state(323);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- unary ----------------
pub type UnaryContextAll<'input> = UnaryContext<'input>;


pub type UnaryContext<'input> = BaseParserRuleContext<'input,UnaryContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for UnaryContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for UnaryContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_unary(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_unary(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for UnaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary }
}
antlr4rust::tid!{UnaryContextExt<'a>}

impl<'input> UnaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<UnaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryContextAttrs<'input>: PSParserContext<'input> + BorrowMut<UnaryContextExt<'input>>{

fn unary(&self) -> Option<Rc<UnaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SUB, 0)
}
fn postfix_all(&self) ->  Vec<Rc<PostfixContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn postfix(&self, i: usize) -> Option<Rc<PostfixContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> UnaryContextAttrs<'input> for UnaryContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn unary(&mut self,)
	-> Result<Rc<UnaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_unary);
        let mut _localctx: Rc<UnaryContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			recog.base.set_state(331);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_ADD |PS_SUB 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(324);
					_la = recog.base.input.la(1);
					if { !(_la==PS_ADD || _la==PS_SUB) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule unary*/
					recog.base.set_state(325);
					recog.unary()?;

					}
				}

			PS_L_PAREN |PS_L_GROUP |PS_L_BRACE |PS_L_BRACE_VISUAL |PS_L_BRACE_CMD |
			PS_L_BRACKET |PS_L_BRACK |PS_PHANTOM_CMD |PS_BOXED_CMD |PS_BAR |PS_L_VERT |
			PS_VERT |PS_NORM |PS_L_ANGLE |PS_L_FLOOR |PS_LL_CORNER |PS_L_CEIL |PS_UL_CORNER |
			PS_FUNC_LIM |PS_FUNC_INT |PS_FUNC_SUM |PS_FUNC_PROD |PS_FUNC_LOG |PS_FUNC_LN |
			PS_FUNC_EXP |PS_FUNC_SIN |PS_FUNC_COS |PS_FUNC_TAN |PS_FUNC_CSC |PS_FUNC_SEC |
			PS_FUNC_COT |PS_FUNC_GAMMA |PS_FUNC_ARCSIN |PS_FUNC_ARCCOS |PS_FUNC_ARCTAN |
			PS_FUNC_ARCCSC |PS_FUNC_ARCSEC |PS_FUNC_ARCCOT |PS_FUNC_SINH |PS_FUNC_COSH |
			PS_FUNC_TANH |PS_FUNC_ARSINH |PS_FUNC_ARCOSH |PS_FUNC_ARTANH |PS_FUNC_ARCSINH |
			PS_FUNC_ARCCOSH |PS_FUNC_ARCTANH |PS_FUNC_SQRT |PS_FUNC_GCD |PS_FUNC_LCM |
			PS_FUNC_FLOOR |PS_FUNC_CEIL |PS_FUNC_MAX |PS_FUNC_MIN |PS_FUNC_DET |PS_CMD_FRAC |
			PS_CMD_BINOM |PS_CMD_OPERATORNAME |PS_CMD_MATRIX_START |PS_CMD_ARRAY_START |
			PS_CMD_DET_START |PS_DIFFERENTIAL |PS_EXP_E |PS_E_NOTATION_E |PS_LETTER_NO_E |
			PS_NUMBER |PS_E_NOTATION |PS_PERCENT_NUMBER |PS_GREEK_CMD |PS_OTHER_SYMBOL_CMD |
			PS_SYMBOL |PS_VARIABLE |PS_ACCENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(327); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							/*InvokeRule postfix*/
							recog.base.set_state(326);
							recog.postfix()?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(329); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- unary_nofunc ----------------
pub type Unary_nofuncContextAll<'input> = Unary_nofuncContext<'input>;


pub type Unary_nofuncContext<'input> = BaseParserRuleContext<'input,Unary_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Unary_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Unary_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Unary_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_unary_nofunc(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_unary_nofunc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Unary_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_nofunc }
}
antlr4rust::tid!{Unary_nofuncContextExt<'a>}

impl<'input> Unary_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Unary_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Unary_nofuncContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Unary_nofuncContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Unary_nofuncContextExt<'input>>{

fn unary_nofunc(&self) -> Option<Rc<Unary_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SUB, 0)
}
fn postfix(&self) -> Option<Rc<PostfixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postfix_nofunc_all(&self) ->  Vec<Rc<Postfix_nofuncContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn postfix_nofunc(&self, i: usize) -> Option<Rc<Postfix_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Unary_nofuncContextAttrs<'input> for Unary_nofuncContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn unary_nofunc(&mut self,)
	-> Result<Rc<Unary_nofuncContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Unary_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_unary_nofunc);
        let mut _localctx: Rc<Unary_nofuncContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			recog.base.set_state(342);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_ADD |PS_SUB 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(333);
					_la = recog.base.input.la(1);
					if { !(_la==PS_ADD || _la==PS_SUB) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule unary_nofunc*/
					recog.base.set_state(334);
					recog.unary_nofunc()?;

					}
				}

			PS_L_PAREN |PS_L_GROUP |PS_L_BRACE |PS_L_BRACE_VISUAL |PS_L_BRACE_CMD |
			PS_L_BRACKET |PS_L_BRACK |PS_PHANTOM_CMD |PS_BOXED_CMD |PS_BAR |PS_L_VERT |
			PS_VERT |PS_NORM |PS_L_ANGLE |PS_L_FLOOR |PS_LL_CORNER |PS_L_CEIL |PS_UL_CORNER |
			PS_FUNC_LIM |PS_FUNC_INT |PS_FUNC_SUM |PS_FUNC_PROD |PS_FUNC_LOG |PS_FUNC_LN |
			PS_FUNC_EXP |PS_FUNC_SIN |PS_FUNC_COS |PS_FUNC_TAN |PS_FUNC_CSC |PS_FUNC_SEC |
			PS_FUNC_COT |PS_FUNC_GAMMA |PS_FUNC_ARCSIN |PS_FUNC_ARCCOS |PS_FUNC_ARCTAN |
			PS_FUNC_ARCCSC |PS_FUNC_ARCSEC |PS_FUNC_ARCCOT |PS_FUNC_SINH |PS_FUNC_COSH |
			PS_FUNC_TANH |PS_FUNC_ARSINH |PS_FUNC_ARCOSH |PS_FUNC_ARTANH |PS_FUNC_ARCSINH |
			PS_FUNC_ARCCOSH |PS_FUNC_ARCTANH |PS_FUNC_SQRT |PS_FUNC_GCD |PS_FUNC_LCM |
			PS_FUNC_FLOOR |PS_FUNC_CEIL |PS_FUNC_MAX |PS_FUNC_MIN |PS_FUNC_DET |PS_CMD_FRAC |
			PS_CMD_BINOM |PS_CMD_OPERATORNAME |PS_CMD_MATRIX_START |PS_CMD_ARRAY_START |
			PS_CMD_DET_START |PS_DIFFERENTIAL |PS_EXP_E |PS_E_NOTATION_E |PS_LETTER_NO_E |
			PS_NUMBER |PS_E_NOTATION |PS_PERCENT_NUMBER |PS_GREEK_CMD |PS_OTHER_SYMBOL_CMD |
			PS_SYMBOL |PS_VARIABLE |PS_ACCENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule postfix*/
					recog.base.set_state(335);
					recog.postfix()?;

					recog.base.set_state(339);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule postfix_nofunc*/
							recog.base.set_state(336);
							recog.postfix_nofunc()?;

							}
							} 
						}
						recog.base.set_state(341);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- postfix ----------------
pub type PostfixContextAll<'input> = PostfixContext<'input>;


pub type PostfixContext<'input> = BaseParserRuleContext<'input,PostfixContextExt<'input>>;

#[derive(Clone)]
pub struct PostfixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for PostfixContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for PostfixContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_postfix(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_postfix(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PostfixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfix }
}
antlr4rust::tid!{PostfixContextExt<'a>}

impl<'input> PostfixContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PostfixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PostfixContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PostfixContextAttrs<'input>: PSParserContext<'input> + BorrowMut<PostfixContextExt<'input>>{

fn exp(&self) -> Option<Rc<ExpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postfix_op_all(&self) ->  Vec<Rc<Postfix_opContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn postfix_op(&self, i: usize) -> Option<Rc<Postfix_opContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PostfixContextAttrs<'input> for PostfixContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn postfix(&mut self,)
	-> Result<Rc<PostfixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PostfixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_postfix);
        let mut _localctx: Rc<PostfixContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule exp*/
			recog.base.set_state(344);
			recog.exp_rec(0)?;

			recog.base.set_state(348);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(23,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule postfix_op*/
					recog.base.set_state(345);
					recog.postfix_op()?;

					}
					} 
				}
				recog.base.set_state(350);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(23,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- postfix_nofunc ----------------
pub type Postfix_nofuncContextAll<'input> = Postfix_nofuncContext<'input>;


pub type Postfix_nofuncContext<'input> = BaseParserRuleContext<'input,Postfix_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Postfix_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Postfix_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Postfix_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_postfix_nofunc(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_postfix_nofunc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Postfix_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfix_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfix_nofunc }
}
antlr4rust::tid!{Postfix_nofuncContextExt<'a>}

impl<'input> Postfix_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Postfix_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Postfix_nofuncContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Postfix_nofuncContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Postfix_nofuncContextExt<'input>>{

fn exp_nofunc(&self) -> Option<Rc<Exp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postfix_op_all(&self) ->  Vec<Rc<Postfix_opContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn postfix_op(&self, i: usize) -> Option<Rc<Postfix_opContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Postfix_nofuncContextAttrs<'input> for Postfix_nofuncContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn postfix_nofunc(&mut self,)
	-> Result<Rc<Postfix_nofuncContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Postfix_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_postfix_nofunc);
        let mut _localctx: Rc<Postfix_nofuncContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule exp_nofunc*/
			recog.base.set_state(351);
			recog.exp_nofunc_rec(0)?;

			recog.base.set_state(355);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(24,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule postfix_op*/
					recog.base.set_state(352);
					recog.postfix_op()?;

					}
					} 
				}
				recog.base.set_state(357);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(24,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- postfix_op ----------------
pub type Postfix_opContextAll<'input> = Postfix_opContext<'input>;


pub type Postfix_opContext<'input> = BaseParserRuleContext<'input,Postfix_opContextExt<'input>>;

#[derive(Clone)]
pub struct Postfix_opContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Postfix_opContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Postfix_opContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_postfix_op(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_postfix_op(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Postfix_opContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfix_op }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfix_op }
}
antlr4rust::tid!{Postfix_opContextExt<'a>}

impl<'input> Postfix_opContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Postfix_opContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Postfix_opContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Postfix_opContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Postfix_opContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BANG
/// Returns `None` if there is no child corresponding to token BANG
fn BANG(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_BANG, 0)
}
fn eval_at(&self) -> Option<Rc<Eval_atContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transpose(&self) -> Option<Rc<TransposeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn degree(&self) -> Option<Rc<DegreeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Postfix_opContextAttrs<'input> for Postfix_opContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn postfix_op(&mut self,)
	-> Result<Rc<Postfix_opContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Postfix_opContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_postfix_op);
        let mut _localctx: Rc<Postfix_opContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(362);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_BANG 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(358);
					recog.base.match_token(PS_BANG,&mut recog.err_handler)?;

					}
				}

			PS_BAR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule eval_at*/
					recog.base.set_state(359);
					recog.eval_at()?;

					}
				}

			PS_T__0 |PS_T__1 |PS_T__2 |PS_T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule transpose*/
					recog.base.set_state(360);
					recog.transpose()?;

					}
				}

			PS_T__4 |PS_T__5 |PS_T__6 |PS_T__7 |PS_T__8 |PS_T__9 |PS_T__10 |PS_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule degree*/
					recog.base.set_state(361);
					recog.degree()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- eval_at ----------------
pub type Eval_atContextAll<'input> = Eval_atContext<'input>;


pub type Eval_atContext<'input> = BaseParserRuleContext<'input,Eval_atContextExt<'input>>;

#[derive(Clone)]
pub struct Eval_atContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Eval_atContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Eval_atContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_eval_at(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_eval_at(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Eval_atContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eval_at }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eval_at }
}
antlr4rust::tid!{Eval_atContextExt<'a>}

impl<'input> Eval_atContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Eval_atContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eval_atContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Eval_atContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Eval_atContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BAR
/// Returns `None` if there is no child corresponding to token BAR
fn BAR(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_BAR, 0)
}
fn eval_at_sup(&self) -> Option<Rc<Eval_at_supContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eval_at_sub(&self) -> Option<Rc<Eval_at_subContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Eval_atContextAttrs<'input> for Eval_atContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn eval_at(&mut self,)
	-> Result<Rc<Eval_atContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eval_atContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_eval_at);
        let mut _localctx: Rc<Eval_atContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(364);
			recog.base.match_token(PS_BAR,&mut recog.err_handler)?;

			recog.base.set_state(370);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule eval_at_sup*/
					recog.base.set_state(365);
					recog.eval_at_sup()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule eval_at_sub*/
					recog.base.set_state(366);
					recog.eval_at_sub()?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule eval_at_sup*/
					recog.base.set_state(367);
					recog.eval_at_sup()?;

					/*InvokeRule eval_at_sub*/
					recog.base.set_state(368);
					recog.eval_at_sub()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- eval_at_sub ----------------
pub type Eval_at_subContextAll<'input> = Eval_at_subContext<'input>;


pub type Eval_at_subContext<'input> = BaseParserRuleContext<'input,Eval_at_subContextExt<'input>>;

#[derive(Clone)]
pub struct Eval_at_subContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Eval_at_subContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Eval_at_subContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_eval_at_sub(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_eval_at_sub(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Eval_at_subContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eval_at_sub }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eval_at_sub }
}
antlr4rust::tid!{Eval_at_subContextExt<'a>}

impl<'input> Eval_at_subContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Eval_at_subContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eval_at_subContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Eval_at_subContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Eval_at_subContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UNDERSCORE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn equality(&self) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Eval_at_subContextAttrs<'input> for Eval_at_subContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn eval_at_sub(&mut self,)
	-> Result<Rc<Eval_at_subContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eval_at_subContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_eval_at_sub);
        let mut _localctx: Rc<Eval_at_subContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(372);
			recog.base.match_token(PS_UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(373);
			recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(376);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule expr*/
					recog.base.set_state(374);
					recog.expr()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule equality*/
					recog.base.set_state(375);
					recog.equality()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(378);
			recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- eval_at_sup ----------------
pub type Eval_at_supContextAll<'input> = Eval_at_supContext<'input>;


pub type Eval_at_supContext<'input> = BaseParserRuleContext<'input,Eval_at_supContextExt<'input>>;

#[derive(Clone)]
pub struct Eval_at_supContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Eval_at_supContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Eval_at_supContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_eval_at_sup(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_eval_at_sup(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Eval_at_supContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eval_at_sup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eval_at_sup }
}
antlr4rust::tid!{Eval_at_supContextExt<'a>}

impl<'input> Eval_at_supContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Eval_at_supContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eval_at_supContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Eval_at_supContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Eval_at_supContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CARET, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn equality(&self) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Eval_at_supContextAttrs<'input> for Eval_at_supContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn eval_at_sup(&mut self,)
	-> Result<Rc<Eval_at_supContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eval_at_supContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_eval_at_sup);
        let mut _localctx: Rc<Eval_at_supContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(380);
			recog.base.match_token(PS_CARET,&mut recog.err_handler)?;

			recog.base.set_state(381);
			recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(384);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule expr*/
					recog.base.set_state(382);
					recog.expr()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule equality*/
					recog.base.set_state(383);
					recog.equality()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(386);
			recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- exp ----------------
pub type ExpContextAll<'input> = ExpContext<'input>;


pub type ExpContext<'input> = BaseParserRuleContext<'input,ExpContextExt<'input>>;

#[derive(Clone)]
pub struct ExpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for ExpContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for ExpContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_exp(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_exp(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ExpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exp }
}
antlr4rust::tid!{ExpContextExt<'a>}

impl<'input> ExpContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ExpContextAttrs<'input>: PSParserContext<'input> + BorrowMut<ExpContextExt<'input>>{

fn comp(&self) -> Option<Rc<CompContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn exp(&self) -> Option<Rc<ExpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CARET, 0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpContextAttrs<'input> for ExpContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  exp(&mut self,)
	-> Result<Rc<ExpContextAll<'input>>,ANTLRError> {
		self.exp_rec(0)
	}

	fn exp_rec(&mut self, _p: i32)
	-> Result<Rc<ExpContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 52, RULE_exp, _p);
	    let mut _localctx: Rc<ExpContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 52;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule comp*/
			recog.base.set_state(389);
			recog.comp()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(405);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(31,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ExpContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_exp)?;
					_localctx = tmp;
					recog.base.set_state(391);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(392);
					recog.base.match_token(PS_CARET,&mut recog.err_handler)?;

					recog.base.set_state(398);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					PS_FUNC_GAMMA |PS_DIFFERENTIAL |PS_E_NOTATION_E |PS_LETTER_NO_E |PS_NUMBER |
					PS_E_NOTATION |PS_PERCENT_NUMBER |PS_GREEK_CMD |PS_OTHER_SYMBOL_CMD |
					PS_SYMBOL |PS_VARIABLE |PS_ACCENT 
						=> {
							{
							/*InvokeRule atom*/
							recog.base.set_state(393);
							recog.atom()?;

							}
						}

					PS_L_BRACE 
						=> {
							{
							recog.base.set_state(394);
							recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(395);
							recog.expr()?;

							recog.base.set_state(396);
							recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(401);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(30,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule subexpr*/
							recog.base.set_state(400);
							recog.subexpr()?;

							}
						}

						_ => {}
					}
					}
					} 
				}
				recog.base.set_state(407);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(31,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- exp_nofunc ----------------
pub type Exp_nofuncContextAll<'input> = Exp_nofuncContext<'input>;


pub type Exp_nofuncContext<'input> = BaseParserRuleContext<'input,Exp_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Exp_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Exp_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Exp_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_exp_nofunc(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_exp_nofunc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Exp_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exp_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exp_nofunc }
}
antlr4rust::tid!{Exp_nofuncContextExt<'a>}

impl<'input> Exp_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Exp_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Exp_nofuncContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Exp_nofuncContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Exp_nofuncContextExt<'input>>{

fn comp_nofunc(&self) -> Option<Rc<Comp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn exp_nofunc(&self) -> Option<Rc<Exp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CARET, 0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Exp_nofuncContextAttrs<'input> for Exp_nofuncContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  exp_nofunc(&mut self,)
	-> Result<Rc<Exp_nofuncContextAll<'input>>,ANTLRError> {
		self.exp_nofunc_rec(0)
	}

	fn exp_nofunc_rec(&mut self, _p: i32)
	-> Result<Rc<Exp_nofuncContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Exp_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 54, RULE_exp_nofunc, _p);
	    let mut _localctx: Rc<Exp_nofuncContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 54;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule comp_nofunc*/
			recog.base.set_state(409);
			recog.comp_nofunc()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(425);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Exp_nofuncContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_exp_nofunc)?;
					_localctx = tmp;
					recog.base.set_state(411);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(412);
					recog.base.match_token(PS_CARET,&mut recog.err_handler)?;

					recog.base.set_state(418);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					PS_FUNC_GAMMA |PS_DIFFERENTIAL |PS_E_NOTATION_E |PS_LETTER_NO_E |PS_NUMBER |
					PS_E_NOTATION |PS_PERCENT_NUMBER |PS_GREEK_CMD |PS_OTHER_SYMBOL_CMD |
					PS_SYMBOL |PS_VARIABLE |PS_ACCENT 
						=> {
							{
							/*InvokeRule atom*/
							recog.base.set_state(413);
							recog.atom()?;

							}
						}

					PS_L_BRACE 
						=> {
							{
							recog.base.set_state(414);
							recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(415);
							recog.expr()?;

							recog.base.set_state(416);
							recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(421);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule subexpr*/
							recog.base.set_state(420);
							recog.subexpr()?;

							}
						}

						_ => {}
					}
					}
					} 
				}
				recog.base.set_state(427);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- comp ----------------
pub type CompContextAll<'input> = CompContext<'input>;


pub type CompContext<'input> = BaseParserRuleContext<'input,CompContextExt<'input>>;

#[derive(Clone)]
pub struct CompContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for CompContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for CompContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_comp(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_comp(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for CompContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comp }
}
antlr4rust::tid!{CompContextExt<'a>}

impl<'input> CompContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CompContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CompContextAttrs<'input>: PSParserContext<'input> + BorrowMut<CompContextExt<'input>>{

fn group(&self) -> Option<Rc<GroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formatting_group(&self) -> Option<Rc<Formatting_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn norm_group(&self) -> Option<Rc<Norm_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn abs_group(&self) -> Option<Rc<Abs_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dot_product(&self) -> Option<Rc<Dot_productContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn floor_group(&self) -> Option<Rc<Floor_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ceil_group(&self) -> Option<Rc<Ceil_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn func(&self) -> Option<Rc<FuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn frac(&self) -> Option<Rc<FracContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn binom(&self) -> Option<Rc<BinomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn matrix(&self) -> Option<Rc<MatrixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn det(&self) -> Option<Rc<DetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CompContextAttrs<'input> for CompContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comp(&mut self,)
	-> Result<Rc<CompContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_comp);
        let mut _localctx: Rc<CompContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(441);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(35,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule group*/
					recog.base.set_state(428);
					recog.group()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule formatting_group*/
					recog.base.set_state(429);
					recog.formatting_group()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule norm_group*/
					recog.base.set_state(430);
					recog.norm_group()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule abs_group*/
					recog.base.set_state(431);
					recog.abs_group()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule dot_product*/
					recog.base.set_state(432);
					recog.dot_product()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule floor_group*/
					recog.base.set_state(433);
					recog.floor_group()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					/*InvokeRule ceil_group*/
					recog.base.set_state(434);
					recog.ceil_group()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					/*InvokeRule func*/
					recog.base.set_state(435);
					recog.func()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					/*InvokeRule atom*/
					recog.base.set_state(436);
					recog.atom()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10)?;
					recog.base.enter_outer_alt(None, 10)?;
					{
					/*InvokeRule frac*/
					recog.base.set_state(437);
					recog.frac()?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11)?;
					recog.base.enter_outer_alt(None, 11)?;
					{
					/*InvokeRule binom*/
					recog.base.set_state(438);
					recog.binom()?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12)?;
					recog.base.enter_outer_alt(None, 12)?;
					{
					/*InvokeRule matrix*/
					recog.base.set_state(439);
					recog.matrix()?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13)?;
					recog.base.enter_outer_alt(None, 13)?;
					{
					/*InvokeRule det*/
					recog.base.set_state(440);
					recog.det()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- comp_nofunc ----------------
pub type Comp_nofuncContextAll<'input> = Comp_nofuncContext<'input>;


pub type Comp_nofuncContext<'input> = BaseParserRuleContext<'input,Comp_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Comp_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Comp_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Comp_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_comp_nofunc(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_comp_nofunc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Comp_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comp_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comp_nofunc }
}
antlr4rust::tid!{Comp_nofuncContextExt<'a>}

impl<'input> Comp_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Comp_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Comp_nofuncContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Comp_nofuncContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Comp_nofuncContextExt<'input>>{

fn group(&self) -> Option<Rc<GroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formatting_group(&self) -> Option<Rc<Formatting_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn norm_group(&self) -> Option<Rc<Norm_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn abs_group(&self) -> Option<Rc<Abs_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dot_product(&self) -> Option<Rc<Dot_productContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn floor_group(&self) -> Option<Rc<Floor_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ceil_group(&self) -> Option<Rc<Ceil_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn frac(&self) -> Option<Rc<FracContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn binom(&self) -> Option<Rc<BinomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn matrix(&self) -> Option<Rc<MatrixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn det(&self) -> Option<Rc<DetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Comp_nofuncContextAttrs<'input> for Comp_nofuncContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comp_nofunc(&mut self,)
	-> Result<Rc<Comp_nofuncContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Comp_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_comp_nofunc);
        let mut _localctx: Rc<Comp_nofuncContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(455);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(36,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule group*/
					recog.base.set_state(443);
					recog.group()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule formatting_group*/
					recog.base.set_state(444);
					recog.formatting_group()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule norm_group*/
					recog.base.set_state(445);
					recog.norm_group()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule abs_group*/
					recog.base.set_state(446);
					recog.abs_group()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule dot_product*/
					recog.base.set_state(447);
					recog.dot_product()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule floor_group*/
					recog.base.set_state(448);
					recog.floor_group()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					/*InvokeRule ceil_group*/
					recog.base.set_state(449);
					recog.ceil_group()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					/*InvokeRule atom*/
					recog.base.set_state(450);
					recog.atom()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					/*InvokeRule frac*/
					recog.base.set_state(451);
					recog.frac()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10)?;
					recog.base.enter_outer_alt(None, 10)?;
					{
					/*InvokeRule binom*/
					recog.base.set_state(452);
					recog.binom()?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11)?;
					recog.base.enter_outer_alt(None, 11)?;
					{
					/*InvokeRule matrix*/
					recog.base.set_state(453);
					recog.matrix()?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12)?;
					recog.base.enter_outer_alt(None, 12)?;
					{
					/*InvokeRule det*/
					recog.base.set_state(454);
					recog.det()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- group ----------------
pub type GroupContextAll<'input> = GroupContext<'input>;


pub type GroupContext<'input> = BaseParserRuleContext<'input,GroupContextExt<'input>>;

#[derive(Clone)]
pub struct GroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for GroupContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for GroupContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_group(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_group(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for GroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_group }
}
antlr4rust::tid!{GroupContextExt<'a>}

impl<'input> GroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GroupContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GroupContextAttrs<'input>: PSParserContext<'input> + BorrowMut<GroupContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_PAREN
/// Returns `None` if there is no child corresponding to token L_PAREN
fn L_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_PAREN, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_PAREN
/// Returns `None` if there is no child corresponding to token R_PAREN
fn R_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_PAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token L_GROUP
/// Returns `None` if there is no child corresponding to token L_GROUP
fn L_GROUP(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_GROUP, 0)
}
/// Retrieves first TerminalNode corresponding to token R_GROUP
/// Returns `None` if there is no child corresponding to token R_GROUP
fn R_GROUP(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_GROUP, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACKET
/// Returns `None` if there is no child corresponding to token L_BRACKET
fn L_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACKET
/// Returns `None` if there is no child corresponding to token R_BRACKET
fn R_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE_VISUAL
/// Returns `None` if there is no child corresponding to token L_BRACE_VISUAL
fn L_BRACE_VISUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE_VISUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE_VISUAL
/// Returns `None` if there is no child corresponding to token R_BRACE_VISUAL
fn R_BRACE_VISUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE_VISUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE_CMD
/// Returns `None` if there is no child corresponding to token L_BRACE_CMD
fn L_BRACE_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE_CMD, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE_CMD
/// Returns `None` if there is no child corresponding to token R_BRACE_CMD
fn R_BRACE_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE_CMD, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACK
/// Returns `None` if there is no child corresponding to token L_BRACK
fn L_BRACK(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACK, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACK
/// Returns `None` if there is no child corresponding to token R_BRACK
fn R_BRACK(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACK, 0)
}

}

impl<'input> GroupContextAttrs<'input> for GroupContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn group(&mut self,)
	-> Result<Rc<GroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_group);
        let mut _localctx: Rc<GroupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(485);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_L_PAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(457);
					recog.base.match_token(PS_L_PAREN,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(458);
					recog.expr()?;

					recog.base.set_state(459);
					recog.base.match_token(PS_R_PAREN,&mut recog.err_handler)?;

					}
				}

			PS_L_GROUP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(461);
					recog.base.match_token(PS_L_GROUP,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(462);
					recog.expr()?;

					recog.base.set_state(463);
					recog.base.match_token(PS_R_GROUP,&mut recog.err_handler)?;

					}
				}

			PS_L_BRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(465);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(466);
					recog.expr()?;

					recog.base.set_state(467);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

			PS_L_BRACKET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(469);
					recog.base.match_token(PS_L_BRACKET,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(470);
					recog.expr()?;

					recog.base.set_state(471);
					recog.base.match_token(PS_R_BRACKET,&mut recog.err_handler)?;

					}
				}

			PS_L_BRACE_VISUAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(473);
					recog.base.match_token(PS_L_BRACE_VISUAL,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(474);
					recog.expr()?;

					recog.base.set_state(475);
					recog.base.match_token(PS_R_BRACE_VISUAL,&mut recog.err_handler)?;

					}
				}

			PS_L_BRACE_CMD 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(477);
					recog.base.match_token(PS_L_BRACE_CMD,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(478);
					recog.expr()?;

					recog.base.set_state(479);
					recog.base.match_token(PS_R_BRACE_CMD,&mut recog.err_handler)?;

					}
				}

			PS_L_BRACK 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					recog.base.set_state(481);
					recog.base.match_token(PS_L_BRACK,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(482);
					recog.expr()?;

					recog.base.set_state(483);
					recog.base.match_token(PS_R_BRACK,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- formatting_group ----------------
pub type Formatting_groupContextAll<'input> = Formatting_groupContext<'input>;


pub type Formatting_groupContext<'input> = BaseParserRuleContext<'input,Formatting_groupContextExt<'input>>;

#[derive(Clone)]
pub struct Formatting_groupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Formatting_groupContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Formatting_groupContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_formatting_group(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_formatting_group(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Formatting_groupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formatting_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formatting_group }
}
antlr4rust::tid!{Formatting_groupContextExt<'a>}

impl<'input> Formatting_groupContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Formatting_groupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Formatting_groupContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Formatting_groupContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Formatting_groupContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PHANTOM_CMD
/// Returns `None` if there is no child corresponding to token PHANTOM_CMD
fn PHANTOM_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_PHANTOM_CMD, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token BOXED_CMD
/// Returns `None` if there is no child corresponding to token BOXED_CMD
fn BOXED_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_BOXED_CMD, 0)
}

}

impl<'input> Formatting_groupContextAttrs<'input> for Formatting_groupContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn formatting_group(&mut self,)
	-> Result<Rc<Formatting_groupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Formatting_groupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_formatting_group);
        let mut _localctx: Rc<Formatting_groupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(497);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_PHANTOM_CMD 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(487);
					recog.base.match_token(PS_PHANTOM_CMD,&mut recog.err_handler)?;

					recog.base.set_state(488);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(489);
					recog.expr()?;

					recog.base.set_state(490);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

			PS_BOXED_CMD 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(492);
					recog.base.match_token(PS_BOXED_CMD,&mut recog.err_handler)?;

					recog.base.set_state(493);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(494);
					recog.expr()?;

					recog.base.set_state(495);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- norm_group ----------------
pub type Norm_groupContextAll<'input> = Norm_groupContext<'input>;


pub type Norm_groupContext<'input> = BaseParserRuleContext<'input,Norm_groupContextExt<'input>>;

#[derive(Clone)]
pub struct Norm_groupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Norm_groupContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Norm_groupContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_norm_group(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_norm_group(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Norm_groupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_norm_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_norm_group }
}
antlr4rust::tid!{Norm_groupContextExt<'a>}

impl<'input> Norm_groupContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Norm_groupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Norm_groupContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Norm_groupContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Norm_groupContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token NORM in current rule
fn NORM_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NORM, starting from 0.
/// Returns `None` if number of children corresponding to token NORM is less or equal than `i`.
fn NORM(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_NORM, i)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Norm_groupContextAttrs<'input> for Norm_groupContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn norm_group(&mut self,)
	-> Result<Rc<Norm_groupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Norm_groupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_norm_group);
        let mut _localctx: Rc<Norm_groupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(499);
			recog.base.match_token(PS_NORM,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(500);
			recog.expr()?;

			recog.base.set_state(501);
			recog.base.match_token(PS_NORM,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- abs_group ----------------
pub type Abs_groupContextAll<'input> = Abs_groupContext<'input>;


pub type Abs_groupContext<'input> = BaseParserRuleContext<'input,Abs_groupContextExt<'input>>;

#[derive(Clone)]
pub struct Abs_groupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Abs_groupContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Abs_groupContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_abs_group(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_abs_group(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Abs_groupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_abs_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_abs_group }
}
antlr4rust::tid!{Abs_groupContextExt<'a>}

impl<'input> Abs_groupContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Abs_groupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Abs_groupContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Abs_groupContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Abs_groupContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token BAR in current rule
fn BAR_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BAR, starting from 0.
/// Returns `None` if number of children corresponding to token BAR is less or equal than `i`.
fn BAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_BAR, i)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_VERT
/// Returns `None` if there is no child corresponding to token L_VERT
fn L_VERT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_VERT, 0)
}
/// Retrieves first TerminalNode corresponding to token R_VERT
/// Returns `None` if there is no child corresponding to token R_VERT
fn R_VERT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_VERT, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token VERT in current rule
fn VERT_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token VERT, starting from 0.
/// Returns `None` if number of children corresponding to token VERT is less or equal than `i`.
fn VERT(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_VERT, i)
}

}

impl<'input> Abs_groupContextAttrs<'input> for Abs_groupContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn abs_group(&mut self,)
	-> Result<Rc<Abs_groupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Abs_groupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_abs_group);
        let mut _localctx: Rc<Abs_groupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(515);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_BAR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(503);
					recog.base.match_token(PS_BAR,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(504);
					recog.expr()?;

					recog.base.set_state(505);
					recog.base.match_token(PS_BAR,&mut recog.err_handler)?;

					}
				}

			PS_L_VERT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(507);
					recog.base.match_token(PS_L_VERT,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(508);
					recog.expr()?;

					recog.base.set_state(509);
					recog.base.match_token(PS_R_VERT,&mut recog.err_handler)?;

					}
				}

			PS_VERT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(511);
					recog.base.match_token(PS_VERT,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(512);
					recog.expr()?;

					recog.base.set_state(513);
					recog.base.match_token(PS_VERT,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- dot_product ----------------
pub type Dot_productContextAll<'input> = Dot_productContext<'input>;


pub type Dot_productContext<'input> = BaseParserRuleContext<'input,Dot_productContextExt<'input>>;

#[derive(Clone)]
pub struct Dot_productContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Dot_productContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Dot_productContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_dot_product(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_dot_product(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Dot_productContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dot_product }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dot_product }
}
antlr4rust::tid!{Dot_productContextExt<'a>}

impl<'input> Dot_productContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Dot_productContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Dot_productContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Dot_productContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Dot_productContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_ANGLE
/// Returns `None` if there is no child corresponding to token L_ANGLE
fn L_ANGLE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_ANGLE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_ANGLE
/// Returns `None` if there is no child corresponding to token R_ANGLE
fn R_ANGLE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_ANGLE, 0)
}

}

impl<'input> Dot_productContextAttrs<'input> for Dot_productContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn dot_product(&mut self,)
	-> Result<Rc<Dot_productContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Dot_productContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_dot_product);
        let mut _localctx: Rc<Dot_productContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(517);
			recog.base.match_token(PS_L_ANGLE,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(518);
			recog.expr()?;

			recog.base.set_state(519);
			recog.base.match_token(PS_R_ANGLE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- floor_group ----------------
pub type Floor_groupContextAll<'input> = Floor_groupContext<'input>;


pub type Floor_groupContext<'input> = BaseParserRuleContext<'input,Floor_groupContextExt<'input>>;

#[derive(Clone)]
pub struct Floor_groupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Floor_groupContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Floor_groupContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_floor_group(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_floor_group(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Floor_groupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_floor_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_floor_group }
}
antlr4rust::tid!{Floor_groupContextExt<'a>}

impl<'input> Floor_groupContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Floor_groupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Floor_groupContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Floor_groupContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Floor_groupContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_FLOOR
/// Returns `None` if there is no child corresponding to token L_FLOOR
fn L_FLOOR(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_FLOOR, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_FLOOR
/// Returns `None` if there is no child corresponding to token R_FLOOR
fn R_FLOOR(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_FLOOR, 0)
}
/// Retrieves first TerminalNode corresponding to token LL_CORNER
/// Returns `None` if there is no child corresponding to token LL_CORNER
fn LL_CORNER(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_LL_CORNER, 0)
}
/// Retrieves first TerminalNode corresponding to token LR_CORNER
/// Returns `None` if there is no child corresponding to token LR_CORNER
fn LR_CORNER(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_LR_CORNER, 0)
}

}

impl<'input> Floor_groupContextAttrs<'input> for Floor_groupContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn floor_group(&mut self,)
	-> Result<Rc<Floor_groupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Floor_groupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_floor_group);
        let mut _localctx: Rc<Floor_groupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(529);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_L_FLOOR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(521);
					recog.base.match_token(PS_L_FLOOR,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(522);
					recog.expr()?;

					recog.base.set_state(523);
					recog.base.match_token(PS_R_FLOOR,&mut recog.err_handler)?;

					}
				}

			PS_LL_CORNER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(525);
					recog.base.match_token(PS_LL_CORNER,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(526);
					recog.expr()?;

					recog.base.set_state(527);
					recog.base.match_token(PS_LR_CORNER,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- ceil_group ----------------
pub type Ceil_groupContextAll<'input> = Ceil_groupContext<'input>;


pub type Ceil_groupContext<'input> = BaseParserRuleContext<'input,Ceil_groupContextExt<'input>>;

#[derive(Clone)]
pub struct Ceil_groupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Ceil_groupContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Ceil_groupContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_ceil_group(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_ceil_group(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Ceil_groupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ceil_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ceil_group }
}
antlr4rust::tid!{Ceil_groupContextExt<'a>}

impl<'input> Ceil_groupContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Ceil_groupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Ceil_groupContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Ceil_groupContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Ceil_groupContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_CEIL
/// Returns `None` if there is no child corresponding to token L_CEIL
fn L_CEIL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_CEIL, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_CEIL
/// Returns `None` if there is no child corresponding to token R_CEIL
fn R_CEIL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_CEIL, 0)
}
/// Retrieves first TerminalNode corresponding to token UL_CORNER
/// Returns `None` if there is no child corresponding to token UL_CORNER
fn UL_CORNER(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UL_CORNER, 0)
}
/// Retrieves first TerminalNode corresponding to token UR_CORNER
/// Returns `None` if there is no child corresponding to token UR_CORNER
fn UR_CORNER(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UR_CORNER, 0)
}

}

impl<'input> Ceil_groupContextAttrs<'input> for Ceil_groupContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn ceil_group(&mut self,)
	-> Result<Rc<Ceil_groupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Ceil_groupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_ceil_group);
        let mut _localctx: Rc<Ceil_groupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(539);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_L_CEIL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(531);
					recog.base.match_token(PS_L_CEIL,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(532);
					recog.expr()?;

					recog.base.set_state(533);
					recog.base.match_token(PS_R_CEIL,&mut recog.err_handler)?;

					}
				}

			PS_UL_CORNER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(535);
					recog.base.match_token(PS_UL_CORNER,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(536);
					recog.expr()?;

					recog.base.set_state(537);
					recog.base.match_token(PS_UR_CORNER,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- atom_expr_no_supexpr ----------------
pub type Atom_expr_no_supexprContextAll<'input> = Atom_expr_no_supexprContext<'input>;


pub type Atom_expr_no_supexprContext<'input> = BaseParserRuleContext<'input,Atom_expr_no_supexprContextExt<'input>>;

#[derive(Clone)]
pub struct Atom_expr_no_supexprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Atom_expr_no_supexprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Atom_expr_no_supexprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atom_expr_no_supexpr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atom_expr_no_supexpr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Atom_expr_no_supexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom_expr_no_supexpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom_expr_no_supexpr }
}
antlr4rust::tid!{Atom_expr_no_supexprContextExt<'a>}

impl<'input> Atom_expr_no_supexprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atom_expr_no_supexprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atom_expr_no_supexprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atom_expr_no_supexprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Atom_expr_no_supexprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LETTER_NO_E
/// Returns `None` if there is no child corresponding to token LETTER_NO_E
fn LETTER_NO_E(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_LETTER_NO_E, 0)
}
/// Retrieves first TerminalNode corresponding to token GREEK_CMD
/// Returns `None` if there is no child corresponding to token GREEK_CMD
fn GREEK_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_GREEK_CMD, 0)
}
/// Retrieves first TerminalNode corresponding to token OTHER_SYMBOL_CMD
/// Returns `None` if there is no child corresponding to token OTHER_SYMBOL_CMD
fn OTHER_SYMBOL_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_OTHER_SYMBOL_CMD, 0)
}
/// Retrieves first TerminalNode corresponding to token ACCENT
/// Returns `None` if there is no child corresponding to token ACCENT
fn ACCENT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ACCENT, 0)
}
fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Atom_expr_no_supexprContextAttrs<'input> for Atom_expr_no_supexprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atom_expr_no_supexpr(&mut self,)
	-> Result<Rc<Atom_expr_no_supexprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atom_expr_no_supexprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_atom_expr_no_supexpr);
        let mut _localctx: Rc<Atom_expr_no_supexprContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(541);
			_la = recog.base.input.la(1);
			if { !(((((_la - 158)) & !0x3f) == 0 && ((1usize << (_la - 158)) & 67305473) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(543);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==PS_UNDERSCORE {
				{
				/*InvokeRule subexpr*/
				recog.base.set_state(542);
				recog.subexpr()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- atom_expr ----------------
pub type Atom_exprContextAll<'input> = Atom_exprContext<'input>;


pub type Atom_exprContext<'input> = BaseParserRuleContext<'input,Atom_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Atom_exprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Atom_exprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Atom_exprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atom_expr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atom_expr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Atom_exprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom_expr }
}
antlr4rust::tid!{Atom_exprContextExt<'a>}

impl<'input> Atom_exprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atom_exprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atom_exprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atom_exprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Atom_exprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LETTER_NO_E
/// Returns `None` if there is no child corresponding to token LETTER_NO_E
fn LETTER_NO_E(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_LETTER_NO_E, 0)
}
/// Retrieves first TerminalNode corresponding to token GREEK_CMD
/// Returns `None` if there is no child corresponding to token GREEK_CMD
fn GREEK_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_GREEK_CMD, 0)
}
/// Retrieves first TerminalNode corresponding to token OTHER_SYMBOL_CMD
/// Returns `None` if there is no child corresponding to token OTHER_SYMBOL_CMD
fn OTHER_SYMBOL_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_OTHER_SYMBOL_CMD, 0)
}
/// Retrieves first TerminalNode corresponding to token ACCENT
/// Returns `None` if there is no child corresponding to token ACCENT
fn ACCENT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ACCENT, 0)
}
fn supexpr(&self) -> Option<Rc<SupexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Atom_exprContextAttrs<'input> for Atom_exprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atom_expr(&mut self,)
	-> Result<Rc<Atom_exprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atom_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_atom_expr);
        let mut _localctx: Rc<Atom_exprContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(545);
			_la = recog.base.input.la(1);
			if { !(((((_la - 158)) & !0x3f) == 0 && ((1usize << (_la - 158)) & 67305473) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(554);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(43,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule supexpr*/
					recog.base.set_state(546);
					recog.supexpr()?;

					/*InvokeRule subexpr*/
					recog.base.set_state(547);
					recog.subexpr()?;

					}
				}

				x if x == 2=>{
					{
					/*InvokeRule subexpr*/
					recog.base.set_state(549);
					recog.subexpr()?;

					/*InvokeRule supexpr*/
					recog.base.set_state(550);
					recog.supexpr()?;

					}
				}

				x if x == 3=>{
					{
					/*InvokeRule subexpr*/
					recog.base.set_state(552);
					recog.subexpr()?;

					}
				}

				x if x == 4=>{
					{
					/*InvokeRule supexpr*/
					recog.base.set_state(553);
					recog.supexpr()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- atom_expr_list ----------------
pub type Atom_expr_listContextAll<'input> = Atom_expr_listContext<'input>;


pub type Atom_expr_listContext<'input> = BaseParserRuleContext<'input,Atom_expr_listContextExt<'input>>;

#[derive(Clone)]
pub struct Atom_expr_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Atom_expr_listContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Atom_expr_listContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atom_expr_list(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atom_expr_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Atom_expr_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom_expr_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom_expr_list }
}
antlr4rust::tid!{Atom_expr_listContextExt<'a>}

impl<'input> Atom_expr_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atom_expr_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atom_expr_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atom_expr_listContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Atom_expr_listContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_PAREN
/// Returns `None` if there is no child corresponding to token L_PAREN
fn L_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_PAREN, 0)
}
fn atom_expr_all(&self) ->  Vec<Rc<Atom_exprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn atom_expr(&self, i: usize) -> Option<Rc<Atom_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token R_PAREN
/// Returns `None` if there is no child corresponding to token R_PAREN
fn R_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_PAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COMMA, i)
}

}

impl<'input> Atom_expr_listContextAttrs<'input> for Atom_expr_listContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atom_expr_list(&mut self,)
	-> Result<Rc<Atom_expr_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atom_expr_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_atom_expr_list);
        let mut _localctx: Rc<Atom_expr_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(568);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_L_PAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(556);
					recog.base.match_token(PS_L_PAREN,&mut recog.err_handler)?;

					/*InvokeRule atom_expr*/
					recog.base.set_state(557);
					recog.atom_expr()?;

					recog.base.set_state(562);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==PS_COMMA {
						{
						{
						recog.base.set_state(558);
						recog.base.match_token(PS_COMMA,&mut recog.err_handler)?;

						/*InvokeRule atom_expr*/
						recog.base.set_state(559);
						recog.atom_expr()?;

						}
						}
						recog.base.set_state(564);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(565);
					recog.base.match_token(PS_R_PAREN,&mut recog.err_handler)?;

					}
					}
				}

			PS_LETTER_NO_E |PS_GREEK_CMD |PS_OTHER_SYMBOL_CMD |PS_ACCENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule atom_expr*/
					recog.base.set_state(567);
					recog.atom_expr()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- number_subexpr ----------------
pub type Number_subexprContextAll<'input> = Number_subexprContext<'input>;


pub type Number_subexprContext<'input> = BaseParserRuleContext<'input,Number_subexprContextExt<'input>>;

#[derive(Clone)]
pub struct Number_subexprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Number_subexprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Number_subexprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_number_subexpr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_number_subexpr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Number_subexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_number_subexpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_number_subexpr }
}
antlr4rust::tid!{Number_subexprContextExt<'a>}

impl<'input> Number_subexprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Number_subexprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Number_subexprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Number_subexprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Number_subexprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_NUMBER, 0)
}
fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Number_subexprContextAttrs<'input> for Number_subexprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn number_subexpr(&mut self,)
	-> Result<Rc<Number_subexprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Number_subexprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_number_subexpr);
        let mut _localctx: Rc<Number_subexprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			recog.base.set_state(570);
			recog.base.match_token(PS_NUMBER,&mut recog.err_handler)?;

			}
			recog.base.set_state(572);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(46,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule subexpr*/
					recog.base.set_state(571);
					recog.subexpr()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- atom ----------------
pub type AtomContextAll<'input> = AtomContext<'input>;


pub type AtomContext<'input> = BaseParserRuleContext<'input,AtomContextExt<'input>>;

#[derive(Clone)]
pub struct AtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for AtomContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for AtomContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atom(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atom(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}
antlr4rust::tid!{AtomContextExt<'a>}

impl<'input> AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AtomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AtomContextAttrs<'input>: PSParserContext<'input> + BorrowMut<AtomContextExt<'input>>{

fn atom_expr(&self) -> Option<Rc<Atom_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SYMBOL
/// Returns `None` if there is no child corresponding to token SYMBOL
fn SYMBOL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SYMBOL, 0)
}
fn number_subexpr(&self) -> Option<Rc<Number_subexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PERCENT_NUMBER
/// Returns `None` if there is no child corresponding to token PERCENT_NUMBER
fn PERCENT_NUMBER(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_PERCENT_NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token E_NOTATION
/// Returns `None` if there is no child corresponding to token E_NOTATION
fn E_NOTATION(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_E_NOTATION, 0)
}
/// Retrieves first TerminalNode corresponding to token E_NOTATION_E
/// Returns `None` if there is no child corresponding to token E_NOTATION_E
fn E_NOTATION_E(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_E_NOTATION_E, 0)
}
/// Retrieves first TerminalNode corresponding to token DIFFERENTIAL
/// Returns `None` if there is no child corresponding to token DIFFERENTIAL
fn DIFFERENTIAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_DIFFERENTIAL, 0)
}
/// Retrieves first TerminalNode corresponding to token VARIABLE
/// Returns `None` if there is no child corresponding to token VARIABLE
fn VARIABLE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_VARIABLE, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_GAMMA
/// Returns `None` if there is no child corresponding to token FUNC_GAMMA
fn FUNC_GAMMA(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_GAMMA, 0)
}

}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atom(&mut self,)
	-> Result<Rc<AtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_atom);
        let mut _localctx: Rc<AtomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(583);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_LETTER_NO_E |PS_GREEK_CMD |PS_OTHER_SYMBOL_CMD |PS_ACCENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule atom_expr*/
					recog.base.set_state(574);
					recog.atom_expr()?;

					}
				}

			PS_SYMBOL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(575);
					recog.base.match_token(PS_SYMBOL,&mut recog.err_handler)?;

					}
				}

			PS_NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule number_subexpr*/
					recog.base.set_state(576);
					recog.number_subexpr()?;

					}
				}

			PS_PERCENT_NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(577);
					recog.base.match_token(PS_PERCENT_NUMBER,&mut recog.err_handler)?;

					}
				}

			PS_E_NOTATION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(578);
					recog.base.match_token(PS_E_NOTATION,&mut recog.err_handler)?;

					}
				}

			PS_E_NOTATION_E 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(579);
					recog.base.match_token(PS_E_NOTATION_E,&mut recog.err_handler)?;

					}
				}

			PS_DIFFERENTIAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					recog.base.set_state(580);
					recog.base.match_token(PS_DIFFERENTIAL,&mut recog.err_handler)?;

					}
				}

			PS_VARIABLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					recog.base.set_state(581);
					recog.base.match_token(PS_VARIABLE,&mut recog.err_handler)?;

					}
				}

			PS_FUNC_GAMMA 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					recog.base.set_state(582);
					recog.base.match_token(PS_FUNC_GAMMA,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- frac ----------------
pub type FracContextAll<'input> = FracContext<'input>;


pub type FracContext<'input> = BaseParserRuleContext<'input,FracContextExt<'input>>;

#[derive(Clone)]
pub struct FracContextExt<'input>{
	pub upper: Option<Rc<ExprContextAll<'input>>>,
	pub lower: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for FracContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for FracContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_frac(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_frac(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FracContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_frac }
	//fn type_rule_index() -> usize where Self: Sized { RULE_frac }
}
antlr4rust::tid!{FracContextExt<'a>}

impl<'input> FracContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FracContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FracContextExt{
				upper: None, lower: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait FracContextAttrs<'input>: PSParserContext<'input> + BorrowMut<FracContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CMD_FRAC
/// Returns `None` if there is no child corresponding to token CMD_FRAC
fn CMD_FRAC(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_FRAC, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token L_BRACE in current rule
fn L_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token L_BRACE is less or equal than `i`.
fn L_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token R_BRACE in current rule
fn R_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token R_BRACE is less or equal than `i`.
fn R_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, i)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FracContextAttrs<'input> for FracContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn frac(&mut self,)
	-> Result<Rc<FracContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FracContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_frac);
        let mut _localctx: Rc<FracContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(585);
			recog.base.match_token(PS_CMD_FRAC,&mut recog.err_handler)?;

			recog.base.set_state(586);
			recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(587);
			let tmp = recog.expr()?;
			// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
			(unsafe { cast_mut::<FracContext >(&mut _localctx) }).upper = Some(tmp.clone());
			  

			recog.base.set_state(588);
			recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(589);
			recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(590);
			let tmp = recog.expr()?;
			// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
			(unsafe { cast_mut::<FracContext >(&mut _localctx) }).lower = Some(tmp.clone());
			  

			recog.base.set_state(591);
			recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- binom ----------------
pub type BinomContextAll<'input> = BinomContext<'input>;


pub type BinomContext<'input> = BaseParserRuleContext<'input,BinomContextExt<'input>>;

#[derive(Clone)]
pub struct BinomContextExt<'input>{
	pub upper: Option<Rc<ExprContextAll<'input>>>,
	pub lower: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for BinomContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for BinomContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_binom(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_binom(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for BinomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binom }
}
antlr4rust::tid!{BinomContextExt<'a>}

impl<'input> BinomContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BinomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BinomContextExt{
				upper: None, lower: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait BinomContextAttrs<'input>: PSParserContext<'input> + BorrowMut<BinomContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token L_BRACE in current rule
fn L_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token L_BRACE is less or equal than `i`.
fn L_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, i)
}
/// Retrieves first TerminalNode corresponding to token CMD_CHOOSE
/// Returns `None` if there is no child corresponding to token CMD_CHOOSE
fn CMD_CHOOSE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_CHOOSE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token R_BRACE in current rule
fn R_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token R_BRACE is less or equal than `i`.
fn R_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, i)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token CMD_BINOM
/// Returns `None` if there is no child corresponding to token CMD_BINOM
fn CMD_BINOM(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_BINOM, 0)
}

}

impl<'input> BinomContextAttrs<'input> for BinomContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn binom(&mut self,)
	-> Result<Rc<BinomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BinomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_binom);
        let mut _localctx: Rc<BinomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(607);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_L_BRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(593);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(594);
					let tmp = recog.expr()?;
					// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
					(unsafe { cast_mut::<BinomContext >(&mut _localctx) }).upper = Some(tmp.clone());
					  

					recog.base.set_state(595);
					recog.base.match_token(PS_CMD_CHOOSE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(596);
					let tmp = recog.expr()?;
					// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
					(unsafe { cast_mut::<BinomContext >(&mut _localctx) }).lower = Some(tmp.clone());
					  

					recog.base.set_state(597);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

			PS_CMD_BINOM 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(599);
					recog.base.match_token(PS_CMD_BINOM,&mut recog.err_handler)?;

					recog.base.set_state(600);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(601);
					let tmp = recog.expr()?;
					// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
					(unsafe { cast_mut::<BinomContext >(&mut _localctx) }).upper = Some(tmp.clone());
					  

					recog.base.set_state(602);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					recog.base.set_state(603);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(604);
					let tmp = recog.expr()?;
					// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
					(unsafe { cast_mut::<BinomContext >(&mut _localctx) }).lower = Some(tmp.clone());
					  

					recog.base.set_state(605);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_normal_functions_single_arg ----------------
pub type Func_normal_functions_single_argContextAll<'input> = Func_normal_functions_single_argContext<'input>;


pub type Func_normal_functions_single_argContext<'input> = BaseParserRuleContext<'input,Func_normal_functions_single_argContextExt<'input>>;

#[derive(Clone)]
pub struct Func_normal_functions_single_argContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_normal_functions_single_argContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_normal_functions_single_argContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_normal_functions_single_arg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_normal_functions_single_arg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_normal_functions_single_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_normal_functions_single_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_normal_functions_single_arg }
}
antlr4rust::tid!{Func_normal_functions_single_argContextExt<'a>}

impl<'input> Func_normal_functions_single_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_normal_functions_single_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_normal_functions_single_argContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_normal_functions_single_argContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_normal_functions_single_argContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FUNC_LOG
/// Returns `None` if there is no child corresponding to token FUNC_LOG
fn FUNC_LOG(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_LOG, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_LN
/// Returns `None` if there is no child corresponding to token FUNC_LN
fn FUNC_LN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_LN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_EXP
/// Returns `None` if there is no child corresponding to token FUNC_EXP
fn FUNC_EXP(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_EXP, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_SIN
/// Returns `None` if there is no child corresponding to token FUNC_SIN
fn FUNC_SIN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_SIN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_COS
/// Returns `None` if there is no child corresponding to token FUNC_COS
fn FUNC_COS(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_COS, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_TAN
/// Returns `None` if there is no child corresponding to token FUNC_TAN
fn FUNC_TAN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_TAN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_CSC
/// Returns `None` if there is no child corresponding to token FUNC_CSC
fn FUNC_CSC(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_CSC, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_SEC
/// Returns `None` if there is no child corresponding to token FUNC_SEC
fn FUNC_SEC(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_SEC, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_COT
/// Returns `None` if there is no child corresponding to token FUNC_COT
fn FUNC_COT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_COT, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCSIN
/// Returns `None` if there is no child corresponding to token FUNC_ARCSIN
fn FUNC_ARCSIN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCSIN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCCOS
/// Returns `None` if there is no child corresponding to token FUNC_ARCCOS
fn FUNC_ARCCOS(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCCOS, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCTAN
/// Returns `None` if there is no child corresponding to token FUNC_ARCTAN
fn FUNC_ARCTAN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCTAN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCCSC
/// Returns `None` if there is no child corresponding to token FUNC_ARCCSC
fn FUNC_ARCCSC(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCCSC, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCSEC
/// Returns `None` if there is no child corresponding to token FUNC_ARCSEC
fn FUNC_ARCSEC(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCSEC, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCCOT
/// Returns `None` if there is no child corresponding to token FUNC_ARCCOT
fn FUNC_ARCCOT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCCOT, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_SINH
/// Returns `None` if there is no child corresponding to token FUNC_SINH
fn FUNC_SINH(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_SINH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_COSH
/// Returns `None` if there is no child corresponding to token FUNC_COSH
fn FUNC_COSH(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_COSH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_TANH
/// Returns `None` if there is no child corresponding to token FUNC_TANH
fn FUNC_TANH(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_TANH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARSINH
/// Returns `None` if there is no child corresponding to token FUNC_ARSINH
fn FUNC_ARSINH(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARSINH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCOSH
/// Returns `None` if there is no child corresponding to token FUNC_ARCOSH
fn FUNC_ARCOSH(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCOSH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARTANH
/// Returns `None` if there is no child corresponding to token FUNC_ARTANH
fn FUNC_ARTANH(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARTANH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCSINH
/// Returns `None` if there is no child corresponding to token FUNC_ARCSINH
fn FUNC_ARCSINH(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCSINH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCCOSH
/// Returns `None` if there is no child corresponding to token FUNC_ARCCOSH
fn FUNC_ARCCOSH(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCCOSH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCTANH
/// Returns `None` if there is no child corresponding to token FUNC_ARCTANH
fn FUNC_ARCTANH(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCTANH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_FLOOR
/// Returns `None` if there is no child corresponding to token FUNC_FLOOR
fn FUNC_FLOOR(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_FLOOR, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_CEIL
/// Returns `None` if there is no child corresponding to token FUNC_CEIL
fn FUNC_CEIL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_CEIL, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_DET
/// Returns `None` if there is no child corresponding to token FUNC_DET
fn FUNC_DET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_DET, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_GAMMA
/// Returns `None` if there is no child corresponding to token FUNC_GAMMA
fn FUNC_GAMMA(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_GAMMA, 0)
}

}

impl<'input> Func_normal_functions_single_argContextAttrs<'input> for Func_normal_functions_single_argContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_normal_functions_single_arg(&mut self,)
	-> Result<Rc<Func_normal_functions_single_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_normal_functions_single_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_func_normal_functions_single_arg);
        let mut _localctx: Rc<Func_normal_functions_single_argContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(609);
			_la = recog.base.input.la(1);
			if { !(((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & 33554431) != 0) || ((((_la - 104)) & !0x3f) == 0 && ((1usize << (_la - 104)) & 19) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_normal_functions_multi_arg ----------------
pub type Func_normal_functions_multi_argContextAll<'input> = Func_normal_functions_multi_argContext<'input>;


pub type Func_normal_functions_multi_argContext<'input> = BaseParserRuleContext<'input,Func_normal_functions_multi_argContextExt<'input>>;

#[derive(Clone)]
pub struct Func_normal_functions_multi_argContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_normal_functions_multi_argContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_normal_functions_multi_argContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_normal_functions_multi_arg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_normal_functions_multi_arg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_normal_functions_multi_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_normal_functions_multi_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_normal_functions_multi_arg }
}
antlr4rust::tid!{Func_normal_functions_multi_argContextExt<'a>}

impl<'input> Func_normal_functions_multi_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_normal_functions_multi_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_normal_functions_multi_argContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_normal_functions_multi_argContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_normal_functions_multi_argContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FUNC_GCD
/// Returns `None` if there is no child corresponding to token FUNC_GCD
fn FUNC_GCD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_GCD, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_LCM
/// Returns `None` if there is no child corresponding to token FUNC_LCM
fn FUNC_LCM(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_LCM, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_MAX
/// Returns `None` if there is no child corresponding to token FUNC_MAX
fn FUNC_MAX(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_MAX, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_MIN
/// Returns `None` if there is no child corresponding to token FUNC_MIN
fn FUNC_MIN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_MIN, 0)
}

}

impl<'input> Func_normal_functions_multi_argContextAttrs<'input> for Func_normal_functions_multi_argContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_normal_functions_multi_arg(&mut self,)
	-> Result<Rc<Func_normal_functions_multi_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_normal_functions_multi_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_func_normal_functions_multi_arg);
        let mut _localctx: Rc<Func_normal_functions_multi_argContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(611);
			_la = recog.base.input.la(1);
			if { !(((((_la - 102)) & !0x3f) == 0 && ((1usize << (_la - 102)) & 51) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_operator_names_single_arg ----------------
pub type Func_operator_names_single_argContextAll<'input> = Func_operator_names_single_argContext<'input>;


pub type Func_operator_names_single_argContext<'input> = BaseParserRuleContext<'input,Func_operator_names_single_argContextExt<'input>>;

#[derive(Clone)]
pub struct Func_operator_names_single_argContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_operator_names_single_argContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_operator_names_single_argContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_operator_names_single_arg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_operator_names_single_arg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_operator_names_single_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_operator_names_single_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_operator_names_single_arg }
}
antlr4rust::tid!{Func_operator_names_single_argContextExt<'a>}

impl<'input> Func_operator_names_single_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_operator_names_single_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_operator_names_single_argContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_operator_names_single_argContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_operator_names_single_argContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FUNC_ARSINH_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ARSINH_NAME
fn FUNC_ARSINH_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARSINH_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCOSH_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ARCOSH_NAME
fn FUNC_ARCOSH_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCOSH_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARTANH_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ARTANH_NAME
fn FUNC_ARTANH_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARTANH_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCSINH_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ARCSINH_NAME
fn FUNC_ARCSINH_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCSINH_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCCOSH_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ARCCOSH_NAME
fn FUNC_ARCCOSH_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCCOSH_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCTANH_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ARCTANH_NAME
fn FUNC_ARCTANH_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ARCTANH_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_FLOOR_NAME
/// Returns `None` if there is no child corresponding to token FUNC_FLOOR_NAME
fn FUNC_FLOOR_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_FLOOR_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_CEIL_NAME
/// Returns `None` if there is no child corresponding to token FUNC_CEIL_NAME
fn FUNC_CEIL_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_CEIL_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_EYE_NAME
/// Returns `None` if there is no child corresponding to token FUNC_EYE_NAME
fn FUNC_EYE_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_EYE_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_RANK_NAME
/// Returns `None` if there is no child corresponding to token FUNC_RANK_NAME
fn FUNC_RANK_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_RANK_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_TRACE_NAME
/// Returns `None` if there is no child corresponding to token FUNC_TRACE_NAME
fn FUNC_TRACE_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_TRACE_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_RREF_NAME
/// Returns `None` if there is no child corresponding to token FUNC_RREF_NAME
fn FUNC_RREF_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_RREF_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_NULLSPACE_NAME
/// Returns `None` if there is no child corresponding to token FUNC_NULLSPACE_NAME
fn FUNC_NULLSPACE_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_NULLSPACE_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_DIAGONALIZE_NAME
/// Returns `None` if there is no child corresponding to token FUNC_DIAGONALIZE_NAME
fn FUNC_DIAGONALIZE_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_DIAGONALIZE_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_NORM_NAME
/// Returns `None` if there is no child corresponding to token FUNC_NORM_NAME
fn FUNC_NORM_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_NORM_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_EIGENVALS_NAME
/// Returns `None` if there is no child corresponding to token FUNC_EIGENVALS_NAME
fn FUNC_EIGENVALS_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_EIGENVALS_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_EIGENVECTORS_NAME
/// Returns `None` if there is no child corresponding to token FUNC_EIGENVECTORS_NAME
fn FUNC_EIGENVECTORS_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_EIGENVECTORS_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_SVD_NAME
/// Returns `None` if there is no child corresponding to token FUNC_SVD_NAME
fn FUNC_SVD_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_SVD_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_COLS_NAME
/// Returns `None` if there is no child corresponding to token FUNC_COLS_NAME
fn FUNC_COLS_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_COLS_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ROWS_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ROWS_NAME
fn FUNC_ROWS_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ROWS_NAME, 0)
}

}

impl<'input> Func_operator_names_single_argContextAttrs<'input> for Func_operator_names_single_argContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_operator_names_single_arg(&mut self,)
	-> Result<Rc<Func_operator_names_single_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_operator_names_single_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_func_operator_names_single_arg);
        let mut _localctx: Rc<Func_operator_names_single_argContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(613);
			_la = recog.base.input.la(1);
			if { !(((((_la - 91)) & !0x3f) == 0 && ((1usize << (_la - 91)) & 2405696319) != 0) || ((((_la - 123)) & !0x3f) == 0 && ((1usize << (_la - 123)) & 15) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_operator_names_multi_arg ----------------
pub type Func_operator_names_multi_argContextAll<'input> = Func_operator_names_multi_argContext<'input>;


pub type Func_operator_names_multi_argContext<'input> = BaseParserRuleContext<'input,Func_operator_names_multi_argContextExt<'input>>;

#[derive(Clone)]
pub struct Func_operator_names_multi_argContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_operator_names_multi_argContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_operator_names_multi_argContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_operator_names_multi_arg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_operator_names_multi_arg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_operator_names_multi_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_operator_names_multi_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_operator_names_multi_arg }
}
antlr4rust::tid!{Func_operator_names_multi_argContextExt<'a>}

impl<'input> Func_operator_names_multi_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_operator_names_multi_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_operator_names_multi_argContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_operator_names_multi_argContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_operator_names_multi_argContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FUNC_GCD_NAME
/// Returns `None` if there is no child corresponding to token FUNC_GCD_NAME
fn FUNC_GCD_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_GCD_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_LCM_NAME
/// Returns `None` if there is no child corresponding to token FUNC_LCM_NAME
fn FUNC_LCM_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_LCM_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ZEROS_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ZEROS_NAME
fn FUNC_ZEROS_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ZEROS_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ORTHOGONALIZE_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ORTHOGONALIZE_NAME
fn FUNC_ORTHOGONALIZE_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ORTHOGONALIZE_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ONES_NAME
/// Returns `None` if there is no child corresponding to token FUNC_ONES_NAME
fn FUNC_ONES_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_ONES_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_DIAG_NAME
/// Returns `None` if there is no child corresponding to token FUNC_DIAG_NAME
fn FUNC_DIAG_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_DIAG_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_HSTACK_NAME
/// Returns `None` if there is no child corresponding to token FUNC_HSTACK_NAME
fn FUNC_HSTACK_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_HSTACK_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_VSTACK_NAME
/// Returns `None` if there is no child corresponding to token FUNC_VSTACK_NAME
fn FUNC_VSTACK_NAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_VSTACK_NAME, 0)
}

}

impl<'input> Func_operator_names_multi_argContextAttrs<'input> for Func_operator_names_multi_argContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_operator_names_multi_arg(&mut self,)
	-> Result<Rc<Func_operator_names_multi_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_operator_names_multi_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_func_operator_names_multi_arg);
        let mut _localctx: Rc<Func_operator_names_multi_argContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(615);
			_la = recog.base.input.la(1);
			if { !(((((_la - 97)) & !0x3f) == 0 && ((1usize << (_la - 97)) & 29515779) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_normal_single_arg ----------------
pub type Func_normal_single_argContextAll<'input> = Func_normal_single_argContext<'input>;


pub type Func_normal_single_argContext<'input> = BaseParserRuleContext<'input,Func_normal_single_argContextExt<'input>>;

#[derive(Clone)]
pub struct Func_normal_single_argContextExt<'input>{
	pub func_operator_name: Option<Rc<Func_operator_names_single_argContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_normal_single_argContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_normal_single_argContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_normal_single_arg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_normal_single_arg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_normal_single_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_normal_single_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_normal_single_arg }
}
antlr4rust::tid!{Func_normal_single_argContextExt<'a>}

impl<'input> Func_normal_single_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_normal_single_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_normal_single_argContextExt{
				func_operator_name: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_normal_single_argContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_normal_single_argContextExt<'input>>{

fn func_normal_functions_single_arg(&self) -> Option<Rc<Func_normal_functions_single_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CMD_OPERATORNAME
/// Returns `None` if there is no child corresponding to token CMD_OPERATORNAME
fn CMD_OPERATORNAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_OPERATORNAME, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
fn func_operator_names_single_arg(&self) -> Option<Rc<Func_operator_names_single_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_normal_single_argContextAttrs<'input> for Func_normal_single_argContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_normal_single_arg(&mut self,)
	-> Result<Rc<Func_normal_single_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_normal_single_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_func_normal_single_arg);
        let mut _localctx: Rc<Func_normal_single_argContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(623);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_FUNC_LOG |PS_FUNC_LN |PS_FUNC_EXP |PS_FUNC_SIN |PS_FUNC_COS |PS_FUNC_TAN |
			PS_FUNC_CSC |PS_FUNC_SEC |PS_FUNC_COT |PS_FUNC_GAMMA |PS_FUNC_ARCSIN |
			PS_FUNC_ARCCOS |PS_FUNC_ARCTAN |PS_FUNC_ARCCSC |PS_FUNC_ARCSEC |PS_FUNC_ARCCOT |
			PS_FUNC_SINH |PS_FUNC_COSH |PS_FUNC_TANH |PS_FUNC_ARSINH |PS_FUNC_ARCOSH |
			PS_FUNC_ARTANH |PS_FUNC_ARCSINH |PS_FUNC_ARCCOSH |PS_FUNC_ARCTANH |PS_FUNC_FLOOR |
			PS_FUNC_CEIL |PS_FUNC_DET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule func_normal_functions_single_arg*/
					recog.base.set_state(617);
					recog.func_normal_functions_single_arg()?;

					}
					}
				}

			PS_CMD_OPERATORNAME 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(618);
					recog.base.match_token(PS_CMD_OPERATORNAME,&mut recog.err_handler)?;

					recog.base.set_state(619);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule func_operator_names_single_arg*/
					recog.base.set_state(620);
					let tmp = recog.func_operator_names_single_arg()?;
					// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
					(unsafe { cast_mut::<Func_normal_single_argContext >(&mut _localctx) }).func_operator_name = Some(tmp.clone());
					  

					recog.base.set_state(621);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_normal_multi_arg ----------------
pub type Func_normal_multi_argContextAll<'input> = Func_normal_multi_argContext<'input>;


pub type Func_normal_multi_argContext<'input> = BaseParserRuleContext<'input,Func_normal_multi_argContextExt<'input>>;

#[derive(Clone)]
pub struct Func_normal_multi_argContextExt<'input>{
	pub func_operator_name: Option<Rc<Func_operator_names_multi_argContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_normal_multi_argContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_normal_multi_argContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_normal_multi_arg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_normal_multi_arg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_normal_multi_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_normal_multi_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_normal_multi_arg }
}
antlr4rust::tid!{Func_normal_multi_argContextExt<'a>}

impl<'input> Func_normal_multi_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_normal_multi_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_normal_multi_argContextExt{
				func_operator_name: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_normal_multi_argContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_normal_multi_argContextExt<'input>>{

fn func_normal_functions_multi_arg(&self) -> Option<Rc<Func_normal_functions_multi_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CMD_OPERATORNAME
/// Returns `None` if there is no child corresponding to token CMD_OPERATORNAME
fn CMD_OPERATORNAME(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CMD_OPERATORNAME, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
fn func_operator_names_multi_arg(&self) -> Option<Rc<Func_operator_names_multi_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_normal_multi_argContextAttrs<'input> for Func_normal_multi_argContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_normal_multi_arg(&mut self,)
	-> Result<Rc<Func_normal_multi_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_normal_multi_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_func_normal_multi_arg);
        let mut _localctx: Rc<Func_normal_multi_argContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(631);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_FUNC_GCD |PS_FUNC_LCM |PS_FUNC_MAX |PS_FUNC_MIN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule func_normal_functions_multi_arg*/
					recog.base.set_state(625);
					recog.func_normal_functions_multi_arg()?;

					}
					}
				}

			PS_CMD_OPERATORNAME 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(626);
					recog.base.match_token(PS_CMD_OPERATORNAME,&mut recog.err_handler)?;

					recog.base.set_state(627);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule func_operator_names_multi_arg*/
					recog.base.set_state(628);
					let tmp = recog.func_operator_names_multi_arg()?;
					// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
					(unsafe { cast_mut::<Func_normal_multi_argContext >(&mut _localctx) }).func_operator_name = Some(tmp.clone());
					  

					recog.base.set_state(629);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func ----------------
pub type FuncContextAll<'input> = FuncContext<'input>;


pub type FuncContext<'input> = BaseParserRuleContext<'input,FuncContextExt<'input>>;

#[derive(Clone)]
pub struct FuncContextExt<'input>{
	pub root: Option<Rc<ExprContextAll<'input>>>,
	pub base: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for FuncContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for FuncContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}
antlr4rust::tid!{FuncContextExt<'a>}

impl<'input> FuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncContextExt{
				root: None, base: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait FuncContextAttrs<'input>: PSParserContext<'input> + BorrowMut<FuncContextExt<'input>>{

fn func_normal_single_arg(&self) -> Option<Rc<Func_normal_single_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_PAREN
/// Returns `None` if there is no child corresponding to token L_PAREN
fn L_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_PAREN, 0)
}
fn func_single_arg(&self) -> Option<Rc<Func_single_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_PAREN
/// Returns `None` if there is no child corresponding to token R_PAREN
fn R_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_PAREN, 0)
}
fn func_single_arg_noparens(&self) -> Option<Rc<Func_single_arg_noparensContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn supexpr(&self) -> Option<Rc<SupexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn func_normal_multi_arg(&self) -> Option<Rc<Func_normal_multi_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn func_multi_arg(&self) -> Option<Rc<Func_multi_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn func_multi_arg_noparens(&self) -> Option<Rc<Func_multi_arg_noparensContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom_expr_no_supexpr(&self) -> Option<Rc<Atom_expr_no_supexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn func_common_args(&self) -> Option<Rc<Func_common_argsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACKET
/// Returns `None` if there is no child corresponding to token L_BRACKET
fn L_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACKET
/// Returns `None` if there is no child corresponding to token R_BRACKET
fn R_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token L_BRACE in current rule
fn L_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token L_BRACE is less or equal than `i`.
fn L_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token R_BRACE in current rule
fn R_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token R_BRACE is less or equal than `i`.
fn R_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, i)
}
/// Retrieves first TerminalNode corresponding to token FUNC_INT
/// Returns `None` if there is no child corresponding to token FUNC_INT
fn FUNC_INT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_INT, 0)
}
/// Retrieves first TerminalNode corresponding to token DIFFERENTIAL
/// Returns `None` if there is no child corresponding to token DIFFERENTIAL
fn DIFFERENTIAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_DIFFERENTIAL, 0)
}
fn frac(&self) -> Option<Rc<FracContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn additive(&self) -> Option<Rc<AdditiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UNDERSCORE, 0)
}
/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CARET, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_SQRT
/// Returns `None` if there is no child corresponding to token FUNC_SQRT
fn FUNC_SQRT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_SQRT, 0)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn mp(&self) -> Option<Rc<MpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_SUM
/// Returns `None` if there is no child corresponding to token FUNC_SUM
fn FUNC_SUM(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_SUM, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_PROD
/// Returns `None` if there is no child corresponding to token FUNC_PROD
fn FUNC_PROD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_PROD, 0)
}
fn subeq(&self) -> Option<Rc<SubeqContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_LIM
/// Returns `None` if there is no child corresponding to token FUNC_LIM
fn FUNC_LIM(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_FUNC_LIM, 0)
}
fn limit_sub(&self) -> Option<Rc<Limit_subContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EXP_E
/// Returns `None` if there is no child corresponding to token EXP_E
fn EXP_E(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_EXP_E, 0)
}

}

impl<'input> FuncContextAttrs<'input> for FuncContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func(&mut self,)
	-> Result<Rc<FuncContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_func);
        let mut _localctx: Rc<FuncContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(764);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(71,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule func_normal_single_arg*/
					recog.base.set_state(633);
					recog.func_normal_single_arg()?;

					recog.base.set_state(646);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(55,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(635);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==PS_UNDERSCORE {
								{
								/*InvokeRule subexpr*/
								recog.base.set_state(634);
								recog.subexpr()?;

								}
							}

							recog.base.set_state(638);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==PS_CARET {
								{
								/*InvokeRule supexpr*/
								recog.base.set_state(637);
								recog.supexpr()?;

								}
							}

							}
						}
					,
						2 =>{
							{
							recog.base.set_state(641);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==PS_CARET {
								{
								/*InvokeRule supexpr*/
								recog.base.set_state(640);
								recog.supexpr()?;

								}
							}

							recog.base.set_state(644);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==PS_UNDERSCORE {
								{
								/*InvokeRule subexpr*/
								recog.base.set_state(643);
								recog.subexpr()?;

								}
							}

							}
						}

						_ => {}
					}
					recog.base.set_state(653);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(56,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(648);
							recog.base.match_token(PS_L_PAREN,&mut recog.err_handler)?;

							/*InvokeRule func_single_arg*/
							recog.base.set_state(649);
							recog.func_single_arg()?;

							recog.base.set_state(650);
							recog.base.match_token(PS_R_PAREN,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule func_single_arg_noparens*/
							recog.base.set_state(652);
							recog.func_single_arg_noparens()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule func_normal_multi_arg*/
					recog.base.set_state(655);
					recog.func_normal_multi_arg()?;

					recog.base.set_state(668);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(61,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(657);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==PS_UNDERSCORE {
								{
								/*InvokeRule subexpr*/
								recog.base.set_state(656);
								recog.subexpr()?;

								}
							}

							recog.base.set_state(660);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==PS_CARET {
								{
								/*InvokeRule supexpr*/
								recog.base.set_state(659);
								recog.supexpr()?;

								}
							}

							}
						}
					,
						2 =>{
							{
							recog.base.set_state(663);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==PS_CARET {
								{
								/*InvokeRule supexpr*/
								recog.base.set_state(662);
								recog.supexpr()?;

								}
							}

							recog.base.set_state(666);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==PS_UNDERSCORE {
								{
								/*InvokeRule subexpr*/
								recog.base.set_state(665);
								recog.subexpr()?;

								}
							}

							}
						}

						_ => {}
					}
					recog.base.set_state(675);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(62,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(670);
							recog.base.match_token(PS_L_PAREN,&mut recog.err_handler)?;

							/*InvokeRule func_multi_arg*/
							recog.base.set_state(671);
							recog.func_multi_arg()?;

							recog.base.set_state(672);
							recog.base.match_token(PS_R_PAREN,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule func_multi_arg_noparens*/
							recog.base.set_state(674);
							recog.func_multi_arg_noparens()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule atom_expr_no_supexpr*/
					recog.base.set_state(677);
					recog.atom_expr_no_supexpr()?;

					recog.base.set_state(679);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PS_CARET {
						{
						/*InvokeRule supexpr*/
						recog.base.set_state(678);
						recog.supexpr()?;

						}
					}

					recog.base.set_state(701);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(64,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(681);
							recog.base.match_token(PS_L_PAREN,&mut recog.err_handler)?;

							/*InvokeRule func_common_args*/
							recog.base.set_state(682);
							recog.func_common_args()?;

							recog.base.set_state(683);
							recog.base.match_token(PS_R_PAREN,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							recog.base.set_state(685);
							recog.base.match_token(PS_L_BRACKET,&mut recog.err_handler)?;

							/*InvokeRule func_common_args*/
							recog.base.set_state(686);
							recog.func_common_args()?;

							recog.base.set_state(687);
							recog.base.match_token(PS_R_BRACKET,&mut recog.err_handler)?;

							}
						}
					,
						3 =>{
							{
							recog.base.set_state(689);
							recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

							recog.base.set_state(690);
							recog.base.match_token(PS_L_PAREN,&mut recog.err_handler)?;

							/*InvokeRule func_common_args*/
							recog.base.set_state(691);
							recog.func_common_args()?;

							recog.base.set_state(692);
							recog.base.match_token(PS_R_PAREN,&mut recog.err_handler)?;

							recog.base.set_state(693);
							recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

							}
						}
					,
						4 =>{
							{
							recog.base.set_state(695);
							recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

							recog.base.set_state(696);
							recog.base.match_token(PS_L_BRACKET,&mut recog.err_handler)?;

							/*InvokeRule func_common_args*/
							recog.base.set_state(697);
							recog.func_common_args()?;

							recog.base.set_state(698);
							recog.base.match_token(PS_R_BRACKET,&mut recog.err_handler)?;

							recog.base.set_state(699);
							recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(703);
					recog.base.match_token(PS_FUNC_INT,&mut recog.err_handler)?;

					recog.base.set_state(724);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(65,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule subexpr*/
							recog.base.set_state(704);
							recog.subexpr()?;

							/*InvokeRule supexpr*/
							recog.base.set_state(705);
							recog.supexpr()?;

							}
						}

						x if x == 2=>{
							{
							/*InvokeRule supexpr*/
							recog.base.set_state(707);
							recog.supexpr()?;

							/*InvokeRule subexpr*/
							recog.base.set_state(708);
							recog.subexpr()?;

							}
						}

						x if x == 3=>{
							{
							{
							recog.base.set_state(710);
							recog.base.match_token(PS_UNDERSCORE,&mut recog.err_handler)?;

							recog.base.set_state(711);
							recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

							recog.base.set_state(712);
							recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

							}
							{
							recog.base.set_state(714);
							recog.base.match_token(PS_CARET,&mut recog.err_handler)?;

							recog.base.set_state(715);
							recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

							recog.base.set_state(716);
							recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

							}
							}
						}

						x if x == 4=>{
							{
							{
							recog.base.set_state(717);
							recog.base.match_token(PS_CARET,&mut recog.err_handler)?;

							recog.base.set_state(718);
							recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

							recog.base.set_state(719);
							recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

							}
							{
							recog.base.set_state(721);
							recog.base.match_token(PS_UNDERSCORE,&mut recog.err_handler)?;

							recog.base.set_state(722);
							recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

							recog.base.set_state(723);
							recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

							}
							}
						}

						_ => {}
					}
					recog.base.set_state(732);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(67,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(727);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(66,&mut recog.base)? {
								x if x == 1=>{
									{
									/*InvokeRule additive*/
									recog.base.set_state(726);
									recog.additive_rec(0)?;

									}
								}

								_ => {}
							}
							recog.base.set_state(729);
							recog.base.match_token(PS_DIFFERENTIAL,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule frac*/
							recog.base.set_state(730);
							recog.frac()?;

							}
						}
					,
						3 =>{
							{
							/*InvokeRule additive*/
							recog.base.set_state(731);
							recog.additive_rec(0)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(734);
					recog.base.match_token(PS_FUNC_SQRT,&mut recog.err_handler)?;

					recog.base.set_state(739);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PS_L_BRACKET {
						{
						recog.base.set_state(735);
						recog.base.match_token(PS_L_BRACKET,&mut recog.err_handler)?;

						/*InvokeRule expr*/
						recog.base.set_state(736);
						let tmp = recog.expr()?;
						// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
						(unsafe { cast_mut::<FuncContext >(&mut _localctx) }).root = Some(tmp.clone());
						  

						recog.base.set_state(737);
						recog.base.match_token(PS_R_BRACKET,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(741);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(742);
					let tmp = recog.expr()?;
					// SAFETY: the borrow only covers this one field access, during which no other handle to the context is dereferenced.
					(unsafe { cast_mut::<FuncContext >(&mut _localctx) }).base = Some(tmp.clone());
					  

					recog.base.set_state(743);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(745);
					_la = recog.base.input.la(1);
					if { !(_la==PS_FUNC_SUM || _la==PS_FUNC_PROD) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(752);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					PS_UNDERSCORE 
						=> {
							{
							/*InvokeRule subeq*/
							recog.base.set_state(746);
							recog.subeq()?;

							/*InvokeRule supexpr*/
							recog.base.set_state(747);
							recog.supexpr()?;

							}
						}

					PS_CARET 
						=> {
							{
							/*InvokeRule supexpr*/
							recog.base.set_state(749);
							recog.supexpr()?;

							/*InvokeRule subeq*/
							recog.base.set_state(750);
							recog.subeq()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					/*InvokeRule mp*/
					recog.base.set_state(754);
					recog.mp_rec(0)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					recog.base.set_state(756);
					recog.base.match_token(PS_FUNC_LIM,&mut recog.err_handler)?;

					/*InvokeRule limit_sub*/
					recog.base.set_state(757);
					recog.limit_sub()?;

					/*InvokeRule mp*/
					recog.base.set_state(758);
					recog.mp_rec(0)?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					recog.base.set_state(760);
					recog.base.match_token(PS_EXP_E,&mut recog.err_handler)?;

					recog.base.set_state(762);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(70,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule supexpr*/
							recog.base.set_state(761);
							recog.supexpr()?;

							}
						}

						_ => {}
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- args ----------------
pub type ArgsContextAll<'input> = ArgsContext<'input>;


pub type ArgsContext<'input> = BaseParserRuleContext<'input,ArgsContextExt<'input>>;

#[derive(Clone)]
pub struct ArgsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for ArgsContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for ArgsContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_args(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_args(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ArgsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_args }
	//fn type_rule_index() -> usize where Self: Sized { RULE_args }
}
antlr4rust::tid!{ArgsContextExt<'a>}

impl<'input> ArgsContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ArgsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ArgsContextAttrs<'input>: PSParserContext<'input> + BorrowMut<ArgsContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COMMA, 0)
}
fn args(&self) -> Option<Rc<ArgsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ArgsContextAttrs<'input> for ArgsContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn args(&mut self,)
	-> Result<Rc<ArgsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_args);
        let mut _localctx: Rc<ArgsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(771);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(72,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule expr*/
					recog.base.set_state(766);
					recog.expr()?;

					recog.base.set_state(767);
					recog.base.match_token(PS_COMMA,&mut recog.err_handler)?;

					/*InvokeRule args*/
					recog.base.set_state(768);
					recog.args()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule expr*/
					recog.base.set_state(770);
					recog.expr()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_common_args ----------------
pub type Func_common_argsContextAll<'input> = Func_common_argsContext<'input>;


pub type Func_common_argsContext<'input> = BaseParserRuleContext<'input,Func_common_argsContextExt<'input>>;

#[derive(Clone)]
pub struct Func_common_argsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_common_argsContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_common_argsContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_common_args(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_common_args(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_common_argsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_common_args }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_common_args }
}
antlr4rust::tid!{Func_common_argsContextExt<'a>}

impl<'input> Func_common_argsContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_common_argsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_common_argsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_common_argsContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_common_argsContextExt<'input>>{

fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COMMA, 0)
}
fn args(&self) -> Option<Rc<ArgsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_common_argsContextAttrs<'input> for Func_common_argsContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_common_args(&mut self,)
	-> Result<Rc<Func_common_argsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_common_argsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_func_common_args);
        let mut _localctx: Rc<Func_common_argsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(781);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(73,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule atom*/
					recog.base.set_state(773);
					recog.atom()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					/*InvokeRule expr*/
					recog.base.set_state(774);
					recog.expr()?;

					recog.base.set_state(775);
					recog.base.match_token(PS_COMMA,&mut recog.err_handler)?;

					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					{
					/*InvokeRule expr*/
					recog.base.set_state(777);
					recog.expr()?;

					recog.base.set_state(778);
					recog.base.match_token(PS_COMMA,&mut recog.err_handler)?;

					/*InvokeRule args*/
					recog.base.set_state(779);
					recog.args()?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- limit_sub ----------------
pub type Limit_subContextAll<'input> = Limit_subContext<'input>;


pub type Limit_subContext<'input> = BaseParserRuleContext<'input,Limit_subContextExt<'input>>;

#[derive(Clone)]
pub struct Limit_subContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Limit_subContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Limit_subContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_limit_sub(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_limit_sub(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Limit_subContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_limit_sub }
	//fn type_rule_index() -> usize where Self: Sized { RULE_limit_sub }
}
antlr4rust::tid!{Limit_subContextExt<'a>}

impl<'input> Limit_subContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Limit_subContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Limit_subContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Limit_subContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Limit_subContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UNDERSCORE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token L_BRACE in current rule
fn L_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token L_BRACE is less or equal than `i`.
fn L_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, i)
}
/// Retrieves first TerminalNode corresponding to token LIM_APPROACH_SYM
/// Returns `None` if there is no child corresponding to token LIM_APPROACH_SYM
fn LIM_APPROACH_SYM(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_LIM_APPROACH_SYM, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token R_BRACE in current rule
fn R_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token R_BRACE is less or equal than `i`.
fn R_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, i)
}
/// Retrieves first TerminalNode corresponding to token LETTER_NO_E
/// Returns `None` if there is no child corresponding to token LETTER_NO_E
fn LETTER_NO_E(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_LETTER_NO_E, 0)
}
/// Retrieves first TerminalNode corresponding to token GREEK_CMD
/// Returns `None` if there is no child corresponding to token GREEK_CMD
fn GREEK_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_GREEK_CMD, 0)
}
/// Retrieves first TerminalNode corresponding to token OTHER_SYMBOL_CMD
/// Returns `None` if there is no child corresponding to token OTHER_SYMBOL_CMD
fn OTHER_SYMBOL_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_OTHER_SYMBOL_CMD, 0)
}
/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CARET, 0)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SUB, 0)
}

}

impl<'input> Limit_subContextAttrs<'input> for Limit_subContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn limit_sub(&mut self,)
	-> Result<Rc<Limit_subContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Limit_subContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_limit_sub);
        let mut _localctx: Rc<Limit_subContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(783);
			recog.base.match_token(PS_UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(784);
			recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(785);
			_la = recog.base.input.la(1);
			if { !(((((_la - 158)) & !0x3f) == 0 && ((1usize << (_la - 158)) & 196609) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(786);
			recog.base.match_token(PS_LIM_APPROACH_SYM,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(787);
			recog.expr()?;

			recog.base.set_state(792);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==PS_CARET {
				{
				recog.base.set_state(788);
				recog.base.match_token(PS_CARET,&mut recog.err_handler)?;

				recog.base.set_state(789);
				recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

				recog.base.set_state(790);
				_la = recog.base.input.la(1);
				if { !(_la==PS_ADD || _la==PS_SUB) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				recog.base.set_state(791);
				recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(794);
			recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_single_arg ----------------
pub type Func_single_argContextAll<'input> = Func_single_argContext<'input>;


pub type Func_single_argContext<'input> = BaseParserRuleContext<'input,Func_single_argContextExt<'input>>;

#[derive(Clone)]
pub struct Func_single_argContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_single_argContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_single_argContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_single_arg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_single_arg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_single_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_single_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_single_arg }
}
antlr4rust::tid!{Func_single_argContextExt<'a>}

impl<'input> Func_single_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_single_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_single_argContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_single_argContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_single_argContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_single_argContextAttrs<'input> for Func_single_argContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_single_arg(&mut self,)
	-> Result<Rc<Func_single_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_single_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_func_single_arg);
        let mut _localctx: Rc<Func_single_argContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule expr*/
			recog.base.set_state(796);
			recog.expr()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_single_arg_noparens ----------------
pub type Func_single_arg_noparensContextAll<'input> = Func_single_arg_noparensContext<'input>;


pub type Func_single_arg_noparensContext<'input> = BaseParserRuleContext<'input,Func_single_arg_noparensContextExt<'input>>;

#[derive(Clone)]
pub struct Func_single_arg_noparensContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_single_arg_noparensContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_single_arg_noparensContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_single_arg_noparens(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_single_arg_noparens(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_single_arg_noparensContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_single_arg_noparens }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_single_arg_noparens }
}
antlr4rust::tid!{Func_single_arg_noparensContextExt<'a>}

impl<'input> Func_single_arg_noparensContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_single_arg_noparensContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_single_arg_noparensContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_single_arg_noparensContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_single_arg_noparensContextExt<'input>>{

fn mp_nofunc(&self) -> Option<Rc<Mp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_single_arg_noparensContextAttrs<'input> for Func_single_arg_noparensContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_single_arg_noparens(&mut self,)
	-> Result<Rc<Func_single_arg_noparensContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_single_arg_noparensContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_func_single_arg_noparens);
        let mut _localctx: Rc<Func_single_arg_noparensContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule mp_nofunc*/
			recog.base.set_state(798);
			recog.mp_nofunc_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_multi_arg ----------------
pub type Func_multi_argContextAll<'input> = Func_multi_argContext<'input>;


pub type Func_multi_argContext<'input> = BaseParserRuleContext<'input,Func_multi_argContextExt<'input>>;

#[derive(Clone)]
pub struct Func_multi_argContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_multi_argContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_multi_argContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_multi_arg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_multi_arg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_multi_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_multi_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_multi_arg }
}
antlr4rust::tid!{Func_multi_argContextExt<'a>}

impl<'input> Func_multi_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_multi_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_multi_argContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_multi_argContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_multi_argContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COMMA, 0)
}
fn func_multi_arg(&self) -> Option<Rc<Func_multi_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_multi_argContextAttrs<'input> for Func_multi_argContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_multi_arg(&mut self,)
	-> Result<Rc<Func_multi_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_multi_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_func_multi_arg);
        let mut _localctx: Rc<Func_multi_argContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(805);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(75,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule expr*/
					recog.base.set_state(800);
					recog.expr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					/*InvokeRule expr*/
					recog.base.set_state(801);
					recog.expr()?;

					recog.base.set_state(802);
					recog.base.match_token(PS_COMMA,&mut recog.err_handler)?;

					/*InvokeRule func_multi_arg*/
					recog.base.set_state(803);
					recog.func_multi_arg()?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- func_multi_arg_noparens ----------------
pub type Func_multi_arg_noparensContextAll<'input> = Func_multi_arg_noparensContext<'input>;


pub type Func_multi_arg_noparensContext<'input> = BaseParserRuleContext<'input,Func_multi_arg_noparensContextExt<'input>>;

#[derive(Clone)]
pub struct Func_multi_arg_noparensContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Func_multi_arg_noparensContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Func_multi_arg_noparensContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_func_multi_arg_noparens(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_func_multi_arg_noparens(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Func_multi_arg_noparensContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_multi_arg_noparens }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_multi_arg_noparens }
}
antlr4rust::tid!{Func_multi_arg_noparensContextExt<'a>}

impl<'input> Func_multi_arg_noparensContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Func_multi_arg_noparensContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_multi_arg_noparensContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Func_multi_arg_noparensContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Func_multi_arg_noparensContextExt<'input>>{

fn mp_nofunc(&self) -> Option<Rc<Mp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_multi_arg_noparensContextAttrs<'input> for Func_multi_arg_noparensContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn func_multi_arg_noparens(&mut self,)
	-> Result<Rc<Func_multi_arg_noparensContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_multi_arg_noparensContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_func_multi_arg_noparens);
        let mut _localctx: Rc<Func_multi_arg_noparensContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule mp_nofunc*/
			recog.base.set_state(807);
			recog.mp_nofunc_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- subexpr ----------------
pub type SubexprContextAll<'input> = SubexprContext<'input>;


pub type SubexprContext<'input> = BaseParserRuleContext<'input,SubexprContextExt<'input>>;

#[derive(Clone)]
pub struct SubexprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for SubexprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for SubexprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_subexpr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_subexpr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for SubexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subexpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subexpr }
}
antlr4rust::tid!{SubexprContextExt<'a>}

impl<'input> SubexprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SubexprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubexprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SubexprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<SubexprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UNDERSCORE, 0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn args(&self) -> Option<Rc<ArgsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SubexprContextAttrs<'input> for SubexprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn subexpr(&mut self,)
	-> Result<Rc<SubexprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubexprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_subexpr);
        let mut _localctx: Rc<SubexprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(809);
			recog.base.match_token(PS_UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(818);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_FUNC_GAMMA |PS_DIFFERENTIAL |PS_E_NOTATION_E |PS_LETTER_NO_E |PS_NUMBER |
			PS_E_NOTATION |PS_PERCENT_NUMBER |PS_GREEK_CMD |PS_OTHER_SYMBOL_CMD |
			PS_SYMBOL |PS_VARIABLE |PS_ACCENT 
				=> {
					{
					/*InvokeRule atom*/
					recog.base.set_state(810);
					recog.atom()?;

					}
				}

			PS_L_BRACE 
				=> {
					{
					recog.base.set_state(811);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					recog.base.set_state(814);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(76,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule expr*/
							recog.base.set_state(812);
							recog.expr()?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule args*/
							recog.base.set_state(813);
							recog.args()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(816);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- supexpr ----------------
pub type SupexprContextAll<'input> = SupexprContext<'input>;


pub type SupexprContext<'input> = BaseParserRuleContext<'input,SupexprContextExt<'input>>;

#[derive(Clone)]
pub struct SupexprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for SupexprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for SupexprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_supexpr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_supexpr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for SupexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_supexpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_supexpr }
}
antlr4rust::tid!{SupexprContextExt<'a>}

impl<'input> SupexprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SupexprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SupexprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SupexprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<SupexprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_CARET, 0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}

}

impl<'input> SupexprContextAttrs<'input> for SupexprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn supexpr(&mut self,)
	-> Result<Rc<SupexprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SupexprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_supexpr);
        let mut _localctx: Rc<SupexprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(820);
			recog.base.match_token(PS_CARET,&mut recog.err_handler)?;

			recog.base.set_state(826);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_FUNC_GAMMA |PS_DIFFERENTIAL |PS_E_NOTATION_E |PS_LETTER_NO_E |PS_NUMBER |
			PS_E_NOTATION |PS_PERCENT_NUMBER |PS_GREEK_CMD |PS_OTHER_SYMBOL_CMD |
			PS_SYMBOL |PS_VARIABLE |PS_ACCENT 
				=> {
					{
					/*InvokeRule atom*/
					recog.base.set_state(821);
					recog.atom()?;

					}
				}

			PS_L_BRACE 
				=> {
					{
					recog.base.set_state(822);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(823);
					recog.expr()?;

					recog.base.set_state(824);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- subeq ----------------
pub type SubeqContextAll<'input> = SubeqContext<'input>;


pub type SubeqContext<'input> = BaseParserRuleContext<'input,SubeqContextExt<'input>>;

#[derive(Clone)]
pub struct SubeqContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for SubeqContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for SubeqContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_subeq(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_subeq(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for SubeqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subeq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subeq }
}
antlr4rust::tid!{SubeqContextExt<'a>}

impl<'input> SubeqContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SubeqContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubeqContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SubeqContextAttrs<'input>: PSParserContext<'input> + BorrowMut<SubeqContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UNDERSCORE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
fn equality(&self) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}

}

impl<'input> SubeqContextAttrs<'input> for SubeqContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn subeq(&mut self,)
	-> Result<Rc<SubeqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubeqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_subeq);
        let mut _localctx: Rc<SubeqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(828);
			recog.base.match_token(PS_UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(829);
			recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule equality*/
			recog.base.set_state(830);
			recog.equality()?;

			recog.base.set_state(831);
			recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- supeq ----------------
pub type SupeqContextAll<'input> = SupeqContext<'input>;


pub type SupeqContext<'input> = BaseParserRuleContext<'input,SupeqContextExt<'input>>;

#[derive(Clone)]
pub struct SupeqContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for SupeqContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for SupeqContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_supeq(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_supeq(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for SupeqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_supeq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_supeq }
}
antlr4rust::tid!{SupeqContextExt<'a>}

impl<'input> SupeqContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SupeqContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SupeqContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SupeqContextAttrs<'input>: PSParserContext<'input> + BorrowMut<SupeqContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UNDERSCORE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
fn equality(&self) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}

}

impl<'input> SupeqContextAttrs<'input> for SupeqContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn supeq(&mut self,)
	-> Result<Rc<SupeqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SupeqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_supeq);
        let mut _localctx: Rc<SupeqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(833);
			recog.base.match_token(PS_UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(834);
			recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule equality*/
			recog.base.set_state(835);
			recog.equality()?;

			recog.base.set_state(836);
			recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- set_relation ----------------
pub type Set_relationContextAll<'input> = Set_relationContext<'input>;


pub type Set_relationContext<'input> = BaseParserRuleContext<'input,Set_relationContextExt<'input>>;

#[derive(Clone)]
pub struct Set_relationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Set_relationContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Set_relationContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_set_relation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_set_relation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Set_relationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_set_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_set_relation }
}
antlr4rust::tid!{Set_relationContextExt<'a>}

impl<'input> Set_relationContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Set_relationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Set_relationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Set_relationContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Set_relationContextExt<'input>>{

fn atom_expr_list(&self) -> Option<Rc<Atom_expr_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn set_relation_all(&self) ->  Vec<Rc<Set_relationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn set_relation(&self, i: usize) -> Option<Rc<Set_relationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_IN, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTIN
/// Returns `None` if there is no child corresponding to token NOTIN
fn NOTIN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_NOTIN, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGNMENT
/// Returns `None` if there is no child corresponding to token ASSIGNMENT
fn ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ASSIGNMENT, 0)
}
fn minus_expr(&self) -> Option<Rc<Minus_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SUBSET
/// Returns `None` if there is no child corresponding to token SUBSET
fn SUBSET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SUBSET, 0)
}
/// Retrieves first TerminalNode corresponding to token SUPSET
/// Returns `None` if there is no child corresponding to token SUPSET
fn SUPSET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SUPSET, 0)
}

}

impl<'input> Set_relationContextAttrs<'input> for Set_relationContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  set_relation(&mut self,)
	-> Result<Rc<Set_relationContextAll<'input>>,ANTLRError> {
		self.set_relation_rec(0)
	}

	fn set_relation_rec(&mut self, _p: i32)
	-> Result<Rc<Set_relationContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Set_relationContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 124, RULE_set_relation, _p);
	    let mut _localctx: Rc<Set_relationContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 124;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(844);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(79,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule atom_expr_list*/
					recog.base.set_state(839);
					recog.atom_expr_list()?;

					recog.base.set_state(840);
					_la = recog.base.input.la(1);
					if { !(((((_la - 163)) & !0x3f) == 0 && ((1usize << (_la - 163)) & 536870915) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule set_relation*/
					recog.base.set_state(841);
					recog.set_relation_rec(2)?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule minus_expr*/
					recog.base.set_state(843);
					recog.minus_expr_rec(0)?;

					}
				}

				_ => {}
			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(851);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(80,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Set_relationContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_set_relation)?;
					_localctx = tmp;
					recog.base.set_state(846);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 3)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
					}
					recog.base.set_state(847);
					_la = recog.base.input.la(1);
					if { !(_la==PS_SUPSET || _la==PS_SUBSET) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule set_relation*/
					recog.base.set_state(848);
					recog.set_relation_rec(4)?;

					}
					} 
				}
				recog.base.set_state(853);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(80,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- minus_expr ----------------
pub type Minus_exprContextAll<'input> = Minus_exprContext<'input>;


pub type Minus_exprContext<'input> = BaseParserRuleContext<'input,Minus_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Minus_exprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Minus_exprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Minus_exprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_minus_expr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_minus_expr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Minus_exprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_minus_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_minus_expr }
}
antlr4rust::tid!{Minus_exprContextExt<'a>}

impl<'input> Minus_exprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Minus_exprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Minus_exprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Minus_exprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Minus_exprContextExt<'input>>{

fn union_expr(&self) -> Option<Rc<Union_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn minus_expr_all(&self) ->  Vec<Rc<Minus_exprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn minus_expr(&self, i: usize) -> Option<Rc<Minus_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SET_MINUS
/// Returns `None` if there is no child corresponding to token SET_MINUS
fn SET_MINUS(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SET_MINUS, 0)
}

}

impl<'input> Minus_exprContextAttrs<'input> for Minus_exprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  minus_expr(&mut self,)
	-> Result<Rc<Minus_exprContextAll<'input>>,ANTLRError> {
		self.minus_expr_rec(0)
	}

	fn minus_expr_rec(&mut self, _p: i32)
	-> Result<Rc<Minus_exprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Minus_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 126, RULE_minus_expr, _p);
	    let mut _localctx: Rc<Minus_exprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 126;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule union_expr*/
			recog.base.set_state(855);
			recog.union_expr_rec(0)?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(862);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(81,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Minus_exprContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_minus_expr)?;
					_localctx = tmp;
					recog.base.set_state(857);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(858);
					recog.base.match_token(PS_SET_MINUS,&mut recog.err_handler)?;

					/*InvokeRule minus_expr*/
					recog.base.set_state(859);
					recog.minus_expr_rec(3)?;

					}
					} 
				}
				recog.base.set_state(864);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(81,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- union_expr ----------------
pub type Union_exprContextAll<'input> = Union_exprContext<'input>;


pub type Union_exprContext<'input> = BaseParserRuleContext<'input,Union_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Union_exprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Union_exprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Union_exprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_union_expr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_union_expr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Union_exprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_union_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_union_expr }
}
antlr4rust::tid!{Union_exprContextExt<'a>}

impl<'input> Union_exprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Union_exprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Union_exprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Union_exprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Union_exprContextExt<'input>>{

fn intersection_expr(&self) -> Option<Rc<Intersection_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn union_expr_all(&self) ->  Vec<Rc<Union_exprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn union_expr(&self, i: usize) -> Option<Rc<Union_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token UNION
/// Returns `None` if there is no child corresponding to token UNION
fn UNION(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_UNION, 0)
}

}

impl<'input> Union_exprContextAttrs<'input> for Union_exprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  union_expr(&mut self,)
	-> Result<Rc<Union_exprContextAll<'input>>,ANTLRError> {
		self.union_expr_rec(0)
	}

	fn union_expr_rec(&mut self, _p: i32)
	-> Result<Rc<Union_exprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Union_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 128, RULE_union_expr, _p);
	    let mut _localctx: Rc<Union_exprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 128;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule intersection_expr*/
			recog.base.set_state(866);
			recog.intersection_expr_rec(0)?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(873);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(82,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Union_exprContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_union_expr)?;
					_localctx = tmp;
					recog.base.set_state(868);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(869);
					recog.base.match_token(PS_UNION,&mut recog.err_handler)?;

					/*InvokeRule union_expr*/
					recog.base.set_state(870);
					recog.union_expr_rec(3)?;

					}
					} 
				}
				recog.base.set_state(875);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(82,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- intersection_expr ----------------
pub type Intersection_exprContextAll<'input> = Intersection_exprContext<'input>;


pub type Intersection_exprContext<'input> = BaseParserRuleContext<'input,Intersection_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Intersection_exprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Intersection_exprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Intersection_exprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intersection_expr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intersection_expr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Intersection_exprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intersection_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intersection_expr }
}
antlr4rust::tid!{Intersection_exprContextExt<'a>}

impl<'input> Intersection_exprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Intersection_exprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Intersection_exprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Intersection_exprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Intersection_exprContextExt<'input>>{

fn set_group(&self) -> Option<Rc<Set_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn intersection_expr_all(&self) ->  Vec<Rc<Intersection_exprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn intersection_expr(&self, i: usize) -> Option<Rc<Intersection_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token INTERSECTION
/// Returns `None` if there is no child corresponding to token INTERSECTION
fn INTERSECTION(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_INTERSECTION, 0)
}

}

impl<'input> Intersection_exprContextAttrs<'input> for Intersection_exprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  intersection_expr(&mut self,)
	-> Result<Rc<Intersection_exprContextAll<'input>>,ANTLRError> {
		self.intersection_expr_rec(0)
	}

	fn intersection_expr_rec(&mut self, _p: i32)
	-> Result<Rc<Intersection_exprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Intersection_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 130, RULE_intersection_expr, _p);
	    let mut _localctx: Rc<Intersection_exprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 130;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule set_group*/
			recog.base.set_state(877);
			recog.set_group()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(884);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Intersection_exprContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_intersection_expr)?;
					_localctx = tmp;
					recog.base.set_state(879);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(880);
					recog.base.match_token(PS_INTERSECTION,&mut recog.err_handler)?;

					/*InvokeRule intersection_expr*/
					recog.base.set_state(881);
					recog.intersection_expr_rec(3)?;

					}
					} 
				}
				recog.base.set_state(886);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- set_group ----------------
pub type Set_groupContextAll<'input> = Set_groupContext<'input>;


pub type Set_groupContext<'input> = BaseParserRuleContext<'input,Set_groupContextExt<'input>>;

#[derive(Clone)]
pub struct Set_groupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Set_groupContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Set_groupContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_set_group(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_set_group(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Set_groupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_set_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_set_group }
}
antlr4rust::tid!{Set_groupContextExt<'a>}

impl<'input> Set_groupContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Set_groupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Set_groupContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Set_groupContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Set_groupContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_PAREN
/// Returns `None` if there is no child corresponding to token L_PAREN
fn L_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_PAREN, 0)
}
fn minus_expr(&self) -> Option<Rc<Minus_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_PAREN
/// Returns `None` if there is no child corresponding to token R_PAREN
fn R_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_PAREN, 0)
}
fn set_atom(&self) -> Option<Rc<Set_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Set_groupContextAttrs<'input> for Set_groupContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn set_group(&mut self,)
	-> Result<Rc<Set_groupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Set_groupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_set_group);
        let mut _localctx: Rc<Set_groupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(892);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(84,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(887);
					recog.base.match_token(PS_L_PAREN,&mut recog.err_handler)?;

					/*InvokeRule minus_expr*/
					recog.base.set_state(888);
					recog.minus_expr_rec(0)?;

					recog.base.set_state(889);
					recog.base.match_token(PS_R_PAREN,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule set_atom*/
					recog.base.set_state(891);
					recog.set_atom()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- set_atom ----------------
pub type Set_atomContextAll<'input> = Set_atomContext<'input>;


pub type Set_atomContext<'input> = BaseParserRuleContext<'input,Set_atomContextExt<'input>>;

#[derive(Clone)]
pub struct Set_atomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Set_atomContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Set_atomContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_set_atom(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_set_atom(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Set_atomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_set_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_set_atom }
}
antlr4rust::tid!{Set_atomContextExt<'a>}

impl<'input> Set_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Set_atomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Set_atomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Set_atomContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Set_atomContextExt<'input>>{

fn interval(&self) -> Option<Rc<IntervalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literal_set(&self) -> Option<Rc<Literal_setContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ordered_tuple(&self) -> Option<Rc<Ordered_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn finite_set(&self) -> Option<Rc<Finite_setContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Set_atomContextAttrs<'input> for Set_atomContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn set_atom(&mut self,)
	-> Result<Rc<Set_atomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Set_atomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_set_atom);
        let mut _localctx: Rc<Set_atomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(898);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(85,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule interval*/
					recog.base.set_state(894);
					recog.interval()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule literal_set*/
					recog.base.set_state(895);
					recog.literal_set()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule ordered_tuple*/
					recog.base.set_state(896);
					recog.ordered_tuple()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule finite_set*/
					recog.base.set_state(897);
					recog.finite_set()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- interval ----------------
pub type IntervalContextAll<'input> = IntervalContext<'input>;


pub type IntervalContext<'input> = BaseParserRuleContext<'input,IntervalContextExt<'input>>;

#[derive(Clone)]
pub struct IntervalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for IntervalContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for IntervalContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_interval(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_interval(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntervalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interval }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interval }
}
antlr4rust::tid!{IntervalContextExt<'a>}

impl<'input> IntervalContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntervalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntervalContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IntervalContextAttrs<'input>: PSParserContext<'input> + BorrowMut<IntervalContextExt<'input>>{

fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACKET
/// Returns `None` if there is no child corresponding to token L_BRACKET
fn L_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token L_PAREN
/// Returns `None` if there is no child corresponding to token L_PAREN
fn L_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_PAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token L_PAREN_VISUAL
/// Returns `None` if there is no child corresponding to token L_PAREN_VISUAL
fn L_PAREN_VISUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_PAREN_VISUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACK
/// Returns `None` if there is no child corresponding to token L_BRACK
fn L_BRACK(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACK, 0)
}
/// Retrieves first TerminalNode corresponding to token L_GROUP
/// Returns `None` if there is no child corresponding to token L_GROUP
fn L_GROUP(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_GROUP, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACKET
/// Returns `None` if there is no child corresponding to token R_BRACKET
fn R_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token R_PAREN
/// Returns `None` if there is no child corresponding to token R_PAREN
fn R_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_PAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token R_PAREN_VISUAL
/// Returns `None` if there is no child corresponding to token R_PAREN_VISUAL
fn R_PAREN_VISUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_PAREN_VISUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACK
/// Returns `None` if there is no child corresponding to token R_BRACK
fn R_BRACK(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACK, 0)
}
/// Retrieves first TerminalNode corresponding to token R_GROUP
/// Returns `None` if there is no child corresponding to token R_GROUP
fn R_GROUP(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_GROUP, 0)
}

}

impl<'input> IntervalContextAttrs<'input> for IntervalContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn interval(&mut self,)
	-> Result<Rc<IntervalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntervalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_interval);
        let mut _localctx: Rc<IntervalContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(900);
			_la = recog.base.input.la(1);
			if { !(((((_la - 28)) & !0x3f) == 0 && ((1usize << (_la - 28)) & 20501) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule expr*/
			recog.base.set_state(901);
			recog.expr()?;

			recog.base.set_state(902);
			recog.base.match_token(PS_COMMA,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(903);
			recog.expr()?;

			recog.base.set_state(904);
			_la = recog.base.input.la(1);
			if { !(((((_la - 29)) & !0x3f) == 0 && ((1usize << (_la - 29)) & 20501) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- ordered_tuple ----------------
pub type Ordered_tupleContextAll<'input> = Ordered_tupleContext<'input>;


pub type Ordered_tupleContext<'input> = BaseParserRuleContext<'input,Ordered_tupleContextExt<'input>>;

#[derive(Clone)]
pub struct Ordered_tupleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Ordered_tupleContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Ordered_tupleContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_ordered_tuple(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_ordered_tuple(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Ordered_tupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ordered_tuple }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ordered_tuple }
}
antlr4rust::tid!{Ordered_tupleContextExt<'a>}

impl<'input> Ordered_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Ordered_tupleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Ordered_tupleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Ordered_tupleContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Ordered_tupleContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_PAREN
/// Returns `None` if there is no child corresponding to token L_PAREN
fn L_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_PAREN, 0)
}
fn semicolon_elements(&self) -> Option<Rc<Semicolon_elementsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_PAREN
/// Returns `None` if there is no child corresponding to token R_PAREN
fn R_PAREN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_PAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token L_PAREN_VISUAL
/// Returns `None` if there is no child corresponding to token L_PAREN_VISUAL
fn L_PAREN_VISUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_PAREN_VISUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token R_PAREN_VISUAL
/// Returns `None` if there is no child corresponding to token R_PAREN_VISUAL
fn R_PAREN_VISUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_PAREN_VISUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACKET
/// Returns `None` if there is no child corresponding to token L_BRACKET
fn L_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACKET
/// Returns `None` if there is no child corresponding to token R_BRACKET
fn R_BRACKET(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACKET, 0)
}

}

impl<'input> Ordered_tupleContextAttrs<'input> for Ordered_tupleContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn ordered_tuple(&mut self,)
	-> Result<Rc<Ordered_tupleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Ordered_tupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_ordered_tuple);
        let mut _localctx: Rc<Ordered_tupleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(918);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_L_PAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(906);
					recog.base.match_token(PS_L_PAREN,&mut recog.err_handler)?;

					/*InvokeRule semicolon_elements*/
					recog.base.set_state(907);
					recog.semicolon_elements()?;

					recog.base.set_state(908);
					recog.base.match_token(PS_R_PAREN,&mut recog.err_handler)?;

					}
					}
				}

			PS_L_PAREN_VISUAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(910);
					recog.base.match_token(PS_L_PAREN_VISUAL,&mut recog.err_handler)?;

					/*InvokeRule semicolon_elements*/
					recog.base.set_state(911);
					recog.semicolon_elements()?;

					recog.base.set_state(912);
					recog.base.match_token(PS_R_PAREN_VISUAL,&mut recog.err_handler)?;

					}
					}
				}

			PS_L_BRACKET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					{
					recog.base.set_state(914);
					recog.base.match_token(PS_L_BRACKET,&mut recog.err_handler)?;

					/*InvokeRule semicolon_elements*/
					recog.base.set_state(915);
					recog.semicolon_elements()?;

					recog.base.set_state(916);
					recog.base.match_token(PS_R_BRACKET,&mut recog.err_handler)?;

					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- finite_set ----------------
pub type Finite_setContextAll<'input> = Finite_setContext<'input>;


pub type Finite_setContext<'input> = BaseParserRuleContext<'input,Finite_setContextExt<'input>>;

#[derive(Clone)]
pub struct Finite_setContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Finite_setContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Finite_setContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_finite_set(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_finite_set(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Finite_setContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_finite_set }
	//fn type_rule_index() -> usize where Self: Sized { RULE_finite_set }
}
antlr4rust::tid!{Finite_setContextExt<'a>}

impl<'input> Finite_setContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Finite_setContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Finite_setContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Finite_setContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Finite_setContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
fn semicolon_elements(&self) -> Option<Rc<Semicolon_elementsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE_VISUAL
/// Returns `None` if there is no child corresponding to token L_BRACE_VISUAL
fn L_BRACE_VISUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE_VISUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE_VISUAL
/// Returns `None` if there is no child corresponding to token R_BRACE_VISUAL
fn R_BRACE_VISUAL(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE_VISUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token BOXED_CMD
/// Returns `None` if there is no child corresponding to token BOXED_CMD
fn BOXED_CMD(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_BOXED_CMD, 0)
}

}

impl<'input> Finite_setContextAttrs<'input> for Finite_setContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn finite_set(&mut self,)
	-> Result<Rc<Finite_setContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Finite_setContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_finite_set);
        let mut _localctx: Rc<Finite_setContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(933);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_L_BRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(920);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule semicolon_elements*/
					recog.base.set_state(921);
					recog.semicolon_elements()?;

					recog.base.set_state(922);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
					}
				}

			PS_L_BRACE_VISUAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(924);
					recog.base.match_token(PS_L_BRACE_VISUAL,&mut recog.err_handler)?;

					/*InvokeRule semicolon_elements*/
					recog.base.set_state(925);
					recog.semicolon_elements()?;

					recog.base.set_state(926);
					recog.base.match_token(PS_R_BRACE_VISUAL,&mut recog.err_handler)?;

					}
					}
				}

			PS_BOXED_CMD 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					{
					recog.base.set_state(928);
					recog.base.match_token(PS_BOXED_CMD,&mut recog.err_handler)?;

					recog.base.set_state(929);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule semicolon_elements*/
					recog.base.set_state(930);
					recog.semicolon_elements()?;

					recog.base.set_state(931);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- set_elements_relation ----------------
pub type Set_elements_relationContextAll<'input> = Set_elements_relationContext<'input>;


pub type Set_elements_relationContext<'input> = BaseParserRuleContext<'input,Set_elements_relationContextExt<'input>>;

#[derive(Clone)]
pub struct Set_elements_relationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Set_elements_relationContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Set_elements_relationContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_set_elements_relation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_set_elements_relation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Set_elements_relationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_set_elements_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_set_elements_relation }
}
antlr4rust::tid!{Set_elements_relationContextExt<'a>}

impl<'input> Set_elements_relationContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Set_elements_relationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Set_elements_relationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Set_elements_relationContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Set_elements_relationContextExt<'input>>{

fn atom_expr_list(&self) -> Option<Rc<Atom_expr_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn semicolon_elements_no_relation(&self) -> Option<Rc<Semicolon_elements_no_relationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_IN, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGNMENT
/// Returns `None` if there is no child corresponding to token ASSIGNMENT
fn ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_ASSIGNMENT, 0)
}

}

impl<'input> Set_elements_relationContextAttrs<'input> for Set_elements_relationContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn set_elements_relation(&mut self,)
	-> Result<Rc<Set_elements_relationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Set_elements_relationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_set_elements_relation);
        let mut _localctx: Rc<Set_elements_relationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atom_expr_list*/
			recog.base.set_state(935);
			recog.atom_expr_list()?;

			recog.base.set_state(936);
			_la = recog.base.input.la(1);
			if { !(_la==PS_IN || _la==PS_ASSIGNMENT) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule semicolon_elements_no_relation*/
			recog.base.set_state(937);
			recog.semicolon_elements_no_relation()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- set_elements ----------------
pub type Set_elementsContextAll<'input> = Set_elementsContext<'input>;


pub type Set_elementsContext<'input> = BaseParserRuleContext<'input,Set_elementsContextExt<'input>>;

#[derive(Clone)]
pub struct Set_elementsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Set_elementsContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Set_elementsContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_set_elements(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_set_elements(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Set_elementsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_set_elements }
	//fn type_rule_index() -> usize where Self: Sized { RULE_set_elements }
}
antlr4rust::tid!{Set_elementsContextExt<'a>}

impl<'input> Set_elementsContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Set_elementsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Set_elementsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Set_elementsContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Set_elementsContextExt<'input>>{

fn semicolon_elements(&self) -> Option<Rc<Semicolon_elementsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Set_elementsContextAttrs<'input> for Set_elementsContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn set_elements(&mut self,)
	-> Result<Rc<Set_elementsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Set_elementsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_set_elements);
        let mut _localctx: Rc<Set_elementsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule semicolon_elements*/
			recog.base.set_state(939);
			recog.semicolon_elements()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- semicolon_elements ----------------
pub type Semicolon_elementsContextAll<'input> = Semicolon_elementsContext<'input>;


pub type Semicolon_elementsContext<'input> = BaseParserRuleContext<'input,Semicolon_elementsContextExt<'input>>;

#[derive(Clone)]
pub struct Semicolon_elementsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Semicolon_elementsContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Semicolon_elementsContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_semicolon_elements(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_semicolon_elements(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Semicolon_elementsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_semicolon_elements }
	//fn type_rule_index() -> usize where Self: Sized { RULE_semicolon_elements }
}
antlr4rust::tid!{Semicolon_elementsContextExt<'a>}

impl<'input> Semicolon_elementsContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Semicolon_elementsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Semicolon_elementsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Semicolon_elementsContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Semicolon_elementsContextExt<'input>>{

fn comma_elements_all(&self) ->  Vec<Rc<Comma_elementsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comma_elements(&self, i: usize) -> Option<Rc<Comma_elementsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
/// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SEMICOLON, i)
}

}

impl<'input> Semicolon_elementsContextAttrs<'input> for Semicolon_elementsContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn semicolon_elements(&mut self,)
	-> Result<Rc<Semicolon_elementsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Semicolon_elementsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_semicolon_elements);
        let mut _localctx: Rc<Semicolon_elementsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule comma_elements*/
			recog.base.set_state(941);
			recog.comma_elements()?;

			recog.base.set_state(946);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==PS_SEMICOLON {
				{
				{
				recog.base.set_state(942);
				recog.base.match_token(PS_SEMICOLON,&mut recog.err_handler)?;

				/*InvokeRule comma_elements*/
				recog.base.set_state(943);
				recog.comma_elements()?;

				}
				}
				recog.base.set_state(948);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- semicolon_elements_no_relation ----------------
pub type Semicolon_elements_no_relationContextAll<'input> = Semicolon_elements_no_relationContext<'input>;


pub type Semicolon_elements_no_relationContext<'input> = BaseParserRuleContext<'input,Semicolon_elements_no_relationContextExt<'input>>;

#[derive(Clone)]
pub struct Semicolon_elements_no_relationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Semicolon_elements_no_relationContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Semicolon_elements_no_relationContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_semicolon_elements_no_relation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_semicolon_elements_no_relation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Semicolon_elements_no_relationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_semicolon_elements_no_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_semicolon_elements_no_relation }
}
antlr4rust::tid!{Semicolon_elements_no_relationContextExt<'a>}

impl<'input> Semicolon_elements_no_relationContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Semicolon_elements_no_relationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Semicolon_elements_no_relationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Semicolon_elements_no_relationContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Semicolon_elements_no_relationContextExt<'input>>{

fn comma_elements_no_relation_all(&self) ->  Vec<Rc<Comma_elements_no_relationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comma_elements_no_relation(&self, i: usize) -> Option<Rc<Comma_elements_no_relationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
/// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SEMICOLON, i)
}

}

impl<'input> Semicolon_elements_no_relationContextAttrs<'input> for Semicolon_elements_no_relationContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn semicolon_elements_no_relation(&mut self,)
	-> Result<Rc<Semicolon_elements_no_relationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Semicolon_elements_no_relationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_semicolon_elements_no_relation);
        let mut _localctx: Rc<Semicolon_elements_no_relationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule comma_elements_no_relation*/
			recog.base.set_state(949);
			recog.comma_elements_no_relation()?;

			recog.base.set_state(954);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==PS_SEMICOLON {
				{
				{
				recog.base.set_state(950);
				recog.base.match_token(PS_SEMICOLON,&mut recog.err_handler)?;

				/*InvokeRule comma_elements_no_relation*/
				recog.base.set_state(951);
				recog.comma_elements_no_relation()?;

				}
				}
				recog.base.set_state(956);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- comma_elements ----------------
pub type Comma_elementsContextAll<'input> = Comma_elementsContext<'input>;


pub type Comma_elementsContext<'input> = BaseParserRuleContext<'input,Comma_elementsContextExt<'input>>;

#[derive(Clone)]
pub struct Comma_elementsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Comma_elementsContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Comma_elementsContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_comma_elements(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_comma_elements(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Comma_elementsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comma_elements }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comma_elements }
}
antlr4rust::tid!{Comma_elementsContextExt<'a>}

impl<'input> Comma_elementsContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Comma_elementsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Comma_elementsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Comma_elementsContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Comma_elementsContextExt<'input>>{

fn element_all(&self) ->  Vec<Rc<ElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn element(&self, i: usize) -> Option<Rc<ElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COMMA, i)
}

}

impl<'input> Comma_elementsContextAttrs<'input> for Comma_elementsContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comma_elements(&mut self,)
	-> Result<Rc<Comma_elementsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Comma_elementsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_comma_elements);
        let mut _localctx: Rc<Comma_elementsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule element*/
			recog.base.set_state(957);
			recog.element()?;

			recog.base.set_state(962);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==PS_COMMA {
				{
				{
				recog.base.set_state(958);
				recog.base.match_token(PS_COMMA,&mut recog.err_handler)?;

				/*InvokeRule element*/
				recog.base.set_state(959);
				recog.element()?;

				}
				}
				recog.base.set_state(964);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- comma_elements_no_relation ----------------
pub type Comma_elements_no_relationContextAll<'input> = Comma_elements_no_relationContext<'input>;


pub type Comma_elements_no_relationContext<'input> = BaseParserRuleContext<'input,Comma_elements_no_relationContextExt<'input>>;

#[derive(Clone)]
pub struct Comma_elements_no_relationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Comma_elements_no_relationContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Comma_elements_no_relationContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_comma_elements_no_relation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_comma_elements_no_relation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Comma_elements_no_relationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comma_elements_no_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comma_elements_no_relation }
}
antlr4rust::tid!{Comma_elements_no_relationContextExt<'a>}

impl<'input> Comma_elements_no_relationContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Comma_elements_no_relationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Comma_elements_no_relationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Comma_elements_no_relationContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Comma_elements_no_relationContextExt<'input>>{

fn element_no_relation_all(&self) ->  Vec<Rc<Element_no_relationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn element_no_relation(&self, i: usize) -> Option<Rc<Element_no_relationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,PSParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_COMMA, i)
}

}

impl<'input> Comma_elements_no_relationContextAttrs<'input> for Comma_elements_no_relationContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comma_elements_no_relation(&mut self,)
	-> Result<Rc<Comma_elements_no_relationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Comma_elements_no_relationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_comma_elements_no_relation);
        let mut _localctx: Rc<Comma_elements_no_relationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule element_no_relation*/
			recog.base.set_state(965);
			recog.element_no_relation()?;

			recog.base.set_state(970);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==PS_COMMA {
				{
				{
				recog.base.set_state(966);
				recog.base.match_token(PS_COMMA,&mut recog.err_handler)?;

				/*InvokeRule element_no_relation*/
				recog.base.set_state(967);
				recog.element_no_relation()?;

				}
				}
				recog.base.set_state(972);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- element_no_relation ----------------
pub type Element_no_relationContextAll<'input> = Element_no_relationContext<'input>;


pub type Element_no_relationContext<'input> = BaseParserRuleContext<'input,Element_no_relationContextExt<'input>>;

#[derive(Clone)]
pub struct Element_no_relationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Element_no_relationContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Element_no_relationContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_element_no_relation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_element_no_relation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Element_no_relationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_element_no_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_element_no_relation }
}
antlr4rust::tid!{Element_no_relationContextExt<'a>}

impl<'input> Element_no_relationContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Element_no_relationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Element_no_relationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Element_no_relationContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Element_no_relationContextExt<'input>>{

fn plus_minus_expr(&self) -> Option<Rc<Plus_minus_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn set_atom(&self) -> Option<Rc<Set_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Element_no_relationContextAttrs<'input> for Element_no_relationContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn element_no_relation(&mut self,)
	-> Result<Rc<Element_no_relationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Element_no_relationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_element_no_relation);
        let mut _localctx: Rc<Element_no_relationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(976);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(92,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule plus_minus_expr*/
					recog.base.set_state(973);
					recog.plus_minus_expr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule set_atom*/
					recog.base.set_state(974);
					recog.set_atom()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule expr*/
					recog.base.set_state(975);
					recog.expr()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- element ----------------
pub type ElementContextAll<'input> = ElementContext<'input>;


pub type ElementContext<'input> = BaseParserRuleContext<'input,ElementContextExt<'input>>;

#[derive(Clone)]
pub struct ElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for ElementContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for ElementContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_element(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_element(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_element }
	//fn type_rule_index() -> usize where Self: Sized { RULE_element }
}
antlr4rust::tid!{ElementContextExt<'a>}

impl<'input> ElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ElementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ElementContextAttrs<'input>: PSParserContext<'input> + BorrowMut<ElementContextExt<'input>>{

fn plus_minus_expr(&self) -> Option<Rc<Plus_minus_exprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn set_atom(&self) -> Option<Rc<Set_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn relation(&self) -> Option<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ElementContextAttrs<'input> for ElementContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn element(&mut self,)
	-> Result<Rc<ElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_element);
        let mut _localctx: Rc<ElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(981);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(93,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule plus_minus_expr*/
					recog.base.set_state(978);
					recog.plus_minus_expr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule set_atom*/
					recog.base.set_state(979);
					recog.set_atom()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule relation*/
					recog.base.set_state(980);
					recog.relation_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- plus_minus_expr ----------------
pub type Plus_minus_exprContextAll<'input> = Plus_minus_exprContext<'input>;


pub type Plus_minus_exprContext<'input> = BaseParserRuleContext<'input,Plus_minus_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Plus_minus_exprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Plus_minus_exprContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Plus_minus_exprContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_plus_minus_expr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_plus_minus_expr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Plus_minus_exprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_plus_minus_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_plus_minus_expr }
}
antlr4rust::tid!{Plus_minus_exprContextExt<'a>}

impl<'input> Plus_minus_exprContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Plus_minus_exprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Plus_minus_exprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Plus_minus_exprContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Plus_minus_exprContextExt<'input>>{

fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token PLUS_MINUS
/// Returns `None` if there is no child corresponding to token PLUS_MINUS
fn PLUS_MINUS(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_PLUS_MINUS, 0)
}

}

impl<'input> Plus_minus_exprContextAttrs<'input> for Plus_minus_exprContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn plus_minus_expr(&mut self,)
	-> Result<Rc<Plus_minus_exprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Plus_minus_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_plus_minus_expr);
        let mut _localctx: Rc<Plus_minus_exprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(989);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_ADD |PS_SUB |PS_L_PAREN |PS_L_GROUP |PS_L_BRACE |PS_L_BRACE_VISUAL |
			PS_L_BRACE_CMD |PS_L_BRACKET |PS_L_BRACK |PS_PHANTOM_CMD |PS_BOXED_CMD |
			PS_BAR |PS_L_VERT |PS_VERT |PS_NORM |PS_L_ANGLE |PS_L_FLOOR |PS_LL_CORNER |
			PS_L_CEIL |PS_UL_CORNER |PS_FUNC_LIM |PS_FUNC_INT |PS_FUNC_SUM |PS_FUNC_PROD |
			PS_FUNC_LOG |PS_FUNC_LN |PS_FUNC_EXP |PS_FUNC_SIN |PS_FUNC_COS |PS_FUNC_TAN |
			PS_FUNC_CSC |PS_FUNC_SEC |PS_FUNC_COT |PS_FUNC_GAMMA |PS_FUNC_ARCSIN |
			PS_FUNC_ARCCOS |PS_FUNC_ARCTAN |PS_FUNC_ARCCSC |PS_FUNC_ARCSEC |PS_FUNC_ARCCOT |
			PS_FUNC_SINH |PS_FUNC_COSH |PS_FUNC_TANH |PS_FUNC_ARSINH |PS_FUNC_ARCOSH |
			PS_FUNC_ARTANH |PS_FUNC_ARCSINH |PS_FUNC_ARCCOSH |PS_FUNC_ARCTANH |PS_FUNC_SQRT |
			PS_FUNC_GCD |PS_FUNC_LCM |PS_FUNC_FLOOR |PS_FUNC_CEIL |PS_FUNC_MAX |PS_FUNC_MIN |
			PS_FUNC_DET |PS_CMD_FRAC |PS_CMD_BINOM |PS_CMD_OPERATORNAME |PS_CMD_MATRIX_START |
			PS_CMD_ARRAY_START |PS_CMD_DET_START |PS_DIFFERENTIAL |PS_EXP_E |PS_E_NOTATION_E |
			PS_LETTER_NO_E |PS_NUMBER |PS_E_NOTATION |PS_PERCENT_NUMBER |PS_GREEK_CMD |
			PS_OTHER_SYMBOL_CMD |PS_SYMBOL |PS_VARIABLE |PS_ACCENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule expr*/
					recog.base.set_state(983);
					recog.expr()?;

					recog.base.set_state(984);
					recog.base.match_token(PS_PLUS_MINUS,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(985);
					recog.expr()?;

					}
				}

			PS_PLUS_MINUS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(987);
					recog.base.match_token(PS_PLUS_MINUS,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(988);
					recog.expr()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- literal_set ----------------
pub type Literal_setContextAll<'input> = Literal_setContext<'input>;


pub type Literal_setContext<'input> = BaseParserRuleContext<'input,Literal_setContextExt<'input>>;

#[derive(Clone)]
pub struct Literal_setContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PSParserContext<'input> for Literal_setContext<'input>{}

impl<'input,'a> Listenable<dyn PSListener<'input> + 'a> for Literal_setContext<'input>{
		fn enter(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_literal_set(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn PSListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_literal_set(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Literal_setContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PSParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal_set }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal_set }
}
antlr4rust::tid!{Literal_setContextExt<'a>}

impl<'input> Literal_setContextExt<'input>{
	fn new(parent: Option<Rc<dyn PSParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Literal_setContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Literal_setContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Literal_setContextAttrs<'input>: PSParserContext<'input> + BorrowMut<Literal_setContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SET_NATURALS
/// Returns `None` if there is no child corresponding to token SET_NATURALS
fn SET_NATURALS(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SET_NATURALS, 0)
}
/// Retrieves first TerminalNode corresponding to token SET_INTEGERS
/// Returns `None` if there is no child corresponding to token SET_INTEGERS
fn SET_INTEGERS(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SET_INTEGERS, 0)
}
/// Retrieves first TerminalNode corresponding to token SET_RATIONALS
/// Returns `None` if there is no child corresponding to token SET_RATIONALS
fn SET_RATIONALS(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SET_RATIONALS, 0)
}
/// Retrieves first TerminalNode corresponding to token SET_REALS
/// Returns `None` if there is no child corresponding to token SET_REALS
fn SET_REALS(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SET_REALS, 0)
}
/// Retrieves first TerminalNode corresponding to token SET_COMPLEX
/// Returns `None` if there is no child corresponding to token SET_COMPLEX
fn SET_COMPLEX(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SET_COMPLEX, 0)
}
/// Retrieves first TerminalNode corresponding to token SET_PRIMES
/// Returns `None` if there is no child corresponding to token SET_PRIMES
fn SET_PRIMES(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SET_PRIMES, 0)
}
/// Retrieves first TerminalNode corresponding to token SET_EMPTY
/// Returns `None` if there is no child corresponding to token SET_EMPTY
fn SET_EMPTY(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_SET_EMPTY, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,PSParserContextType>>> where Self:Sized{
	self.get_token(PS_R_BRACE, 0)
}

}

impl<'input> Literal_setContextAttrs<'input> for Literal_setContext<'input>{}

impl<'input, I> PSParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn literal_set(&mut self,)
	-> Result<Rc<Literal_setContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Literal_setContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_literal_set);
        let mut _localctx: Rc<Literal_setContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1000);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			PS_SET_NATURALS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(991);
					recog.base.match_token(PS_SET_NATURALS,&mut recog.err_handler)?;

					}
				}

			PS_SET_INTEGERS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(992);
					recog.base.match_token(PS_SET_INTEGERS,&mut recog.err_handler)?;

					}
				}

			PS_SET_RATIONALS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(993);
					recog.base.match_token(PS_SET_RATIONALS,&mut recog.err_handler)?;

					}
				}

			PS_SET_REALS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(994);
					recog.base.match_token(PS_SET_REALS,&mut recog.err_handler)?;

					}
				}

			PS_SET_COMPLEX 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(995);
					recog.base.match_token(PS_SET_COMPLEX,&mut recog.err_handler)?;

					}
				}

			PS_SET_PRIMES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(996);
					recog.base.match_token(PS_SET_PRIMES,&mut recog.err_handler)?;

					}
				}

			PS_SET_EMPTY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					recog.base.set_state(997);
					recog.base.match_token(PS_SET_EMPTY,&mut recog.err_handler)?;

					}
				}

			PS_L_BRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					recog.base.set_state(998);
					recog.base.match_token(PS_L_BRACE,&mut recog.err_handler)?;

					recog.base.set_state(999);
					recog.base.match_token(PS_R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 192, 1003, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 
		7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 
		7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 
		7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 20, 
		7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 
		7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 30, 
		7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 35, 
		7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 39, 2, 40, 
		7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 44, 2, 45, 
		7, 45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 48, 2, 49, 7, 49, 2, 50, 
		7, 50, 2, 51, 7, 51, 2, 52, 7, 52, 2, 53, 7, 53, 2, 54, 7, 54, 2, 55, 
		7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 2, 58, 7, 58, 2, 59, 7, 59, 2, 60, 
		7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 63, 7, 63, 2, 64, 7, 64, 2, 65, 
		7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 2, 68, 7, 68, 2, 69, 7, 69, 2, 70, 
		7, 70, 2, 71, 7, 71, 2, 72, 7, 72, 2, 73, 7, 73, 2, 74, 7, 74, 2, 75, 
		7, 75, 2, 76, 7, 76, 2, 77, 7, 77, 2, 78, 7, 78, 2, 79, 7, 79, 2, 80, 
		7, 80, 1, 0, 1, 0, 1, 0, 3, 0, 166, 8, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 2, 
		1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 180, 8, 3, 1, 4, 1, 4, 
		1, 4, 1, 4, 3, 4, 186, 8, 4, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 6, 
		1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 3, 7, 200, 8, 7, 1, 8, 1, 8, 1, 8, 5, 8, 
		205, 8, 8, 10, 8, 12, 8, 208, 9, 8, 1, 9, 1, 9, 1, 9, 1, 9, 5, 9, 214, 
		8, 9, 10, 9, 12, 9, 217, 9, 9, 1, 9, 3, 9, 220, 8, 9, 1, 9, 1, 9, 1, 9, 
		1, 9, 1, 9, 1, 9, 5, 9, 228, 8, 9, 10, 9, 12, 9, 231, 9, 9, 1, 9, 3, 9, 
		234, 8, 9, 1, 9, 1, 9, 3, 9, 238, 8, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 
		3, 9, 245, 8, 9, 1, 9, 1, 9, 1, 9, 1, 9, 3, 9, 251, 8, 9, 1, 10, 1, 10, 
		1, 10, 1, 10, 5, 10, 257, 8, 10, 10, 10, 12, 10, 260, 9, 10, 1, 10, 3, 
		10, 263, 8, 10, 1, 10, 1, 10, 1, 11, 1, 11, 1, 11, 5, 11, 270, 8, 11, 
		10, 11, 12, 11, 273, 9, 11, 1, 12, 1, 12, 1, 12, 1, 12, 1, 12, 1, 12, 
		5, 12, 281, 8, 12, 10, 12, 12, 12, 284, 9, 12, 1, 13, 1, 13, 1, 13, 1, 
		13, 1, 14, 1, 14, 1, 15, 1, 15, 1, 15, 1, 15, 1, 15, 1, 15, 5, 15, 298, 
		8, 15, 10, 15, 12, 15, 301, 9, 15, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 
		1, 16, 5, 16, 309, 8, 16, 10, 16, 12, 16, 312, 9, 16, 1, 17, 1, 17, 1, 
		17, 1, 17, 1, 17, 1, 17, 5, 17, 320, 8, 17, 10, 17, 12, 17, 323, 9, 17, 
		1, 18, 1, 18, 1, 18, 4, 18, 328, 8, 18, 11, 18, 12, 18, 329, 3, 18, 332, 
		8, 18, 1, 19, 1, 19, 1, 19, 1, 19, 5, 19, 338, 8, 19, 10, 19, 12, 19, 
		341, 9, 19, 3, 19, 343, 8, 19, 1, 20, 1, 20, 5, 20, 347, 8, 20, 10, 20, 
		12, 20, 350, 9, 20, 1, 21, 1, 21, 5, 21, 354, 8, 21, 10, 21, 12, 21, 357, 
		9, 21, 1, 22, 1, 22, 1, 22, 1, 22, 3, 22, 363, 8, 22, 1, 23, 1, 23, 1, 
		23, 1, 23, 1, 23, 1, 23, 3, 23, 371, 8, 23, 1, 24, 1, 24, 1, 24, 1, 24, 
		3, 24, 377, 8, 24, 1, 24, 1, 24, 1, 25, 1, 25, 1, 25, 1, 25, 3, 25, 385, 
		8, 25, 1, 25, 1, 25, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 
		1, 26, 1, 26, 1, 26, 3, 26, 399, 8, 26, 1, 26, 3, 26, 402, 8, 26, 5, 26, 
		404, 8, 26, 10, 26, 12, 26, 407, 9, 26, 1, 27, 1, 27, 1, 27, 1, 27, 1, 
		27, 1, 27, 1, 27, 1, 27, 1, 27, 1, 27, 3, 27, 419, 8, 27, 1, 27, 3, 27, 
		422, 8, 27, 5, 27, 424, 8, 27, 10, 27, 12, 27, 427, 9, 27, 1, 28, 1, 28, 
		1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 
		1, 28, 3, 28, 442, 8, 28, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 
		29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 3, 29, 456, 8, 29, 1, 30, 1, 30, 
		1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 
		1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 
		1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 3, 30, 486, 8, 30, 1, 31, 1, 
		31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 3, 31, 498, 
		8, 31, 1, 32, 1, 32, 1, 32, 1, 32, 1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 
		1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 3, 33, 516, 8, 33, 1, 
		34, 1, 34, 1, 34, 1, 34, 1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 1, 
		35, 1, 35, 3, 35, 530, 8, 35, 1, 36, 1, 36, 1, 36, 1, 36, 1, 36, 1, 36, 
		1, 36, 1, 36, 3, 36, 540, 8, 36, 1, 37, 1, 37, 3, 37, 544, 8, 37, 1, 38, 
		1, 38, 1, 38, 1, 38, 1, 38, 1, 38, 1, 38, 1, 38, 1, 38, 3, 38, 555, 8, 
		38, 1, 39, 1, 39, 1, 39, 1, 39, 5, 39, 561, 8, 39, 10, 39, 12, 39, 564, 
		9, 39, 1, 39, 1, 39, 1, 39, 3, 39, 569, 8, 39, 1, 40, 1, 40, 3, 40, 573, 
		8, 40, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 
		3, 41, 584, 8, 41, 1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 1, 
		42, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 1, 
		43, 1, 43, 1, 43, 1, 43, 1, 43, 3, 43, 608, 8, 43, 1, 44, 1, 44, 1, 45, 
		1, 45, 1, 46, 1, 46, 1, 47, 1, 47, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 
		1, 48, 3, 48, 624, 8, 48, 1, 49, 1, 49, 1, 49, 1, 49, 1, 49, 1, 49, 3, 
		49, 632, 8, 49, 1, 50, 1, 50, 3, 50, 636, 8, 50, 1, 50, 3, 50, 639, 8, 
		50, 1, 50, 3, 50, 642, 8, 50, 1, 50, 3, 50, 645, 8, 50, 3, 50, 647, 8, 
		50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 3, 50, 654, 8, 50, 1, 50, 1, 50, 
		3, 50, 658, 8, 50, 1, 50, 3, 50, 661, 8, 50, 1, 50, 3, 50, 664, 8, 50, 
		1, 50, 3, 50, 667, 8, 50, 3, 50, 669, 8, 50, 1, 50, 1, 50, 1, 50, 1, 50, 
		1, 50, 3, 50, 676, 8, 50, 1, 50, 1, 50, 3, 50, 680, 8, 50, 1, 50, 1, 50, 
		1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 
		1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 3, 50, 702, 8, 
		50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 
		50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 
		50, 1, 50, 3, 50, 725, 8, 50, 1, 50, 3, 50, 728, 8, 50, 1, 50, 1, 50, 
		1, 50, 3, 50, 733, 8, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 3, 50, 740, 
		8, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 
		1, 50, 1, 50, 3, 50, 753, 8, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 
		50, 1, 50, 1, 50, 3, 50, 763, 8, 50, 3, 50, 765, 8, 50, 1, 51, 1, 51, 
		1, 51, 1, 51, 1, 51, 3, 51, 772, 8, 51, 1, 52, 1, 52, 1, 52, 1, 52, 1, 
		52, 1, 52, 1, 52, 1, 52, 3, 52, 782, 8, 52, 1, 53, 1, 53, 1, 53, 1, 53, 
		1, 53, 1, 53, 1, 53, 1, 53, 1, 53, 3, 53, 793, 8, 53, 1, 53, 1, 53, 1, 
		54, 1, 54, 1, 55, 1, 55, 1, 56, 1, 56, 1, 56, 1, 56, 1, 56, 3, 56, 806, 
		8, 56, 1, 57, 1, 57, 1, 58, 1, 58, 1, 58, 1, 58, 1, 58, 3, 58, 815, 8, 
		58, 1, 58, 1, 58, 3, 58, 819, 8, 58, 1, 59, 1, 59, 1, 59, 1, 59, 1, 59, 
		1, 59, 3, 59, 827, 8, 59, 1, 60, 1, 60, 1, 60, 1, 60, 1, 60, 1, 61, 1, 
		61, 1, 61, 1, 61, 1, 61, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 3, 
		62, 845, 8, 62, 1, 62, 1, 62, 1, 62, 5, 62, 850, 8, 62, 10, 62, 12, 62, 
		853, 9, 62, 1, 63, 1, 63, 1, 63, 1, 63, 1, 63, 1, 63, 5, 63, 861, 8, 63, 
		10, 63, 12, 63, 864, 9, 63, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 
		5, 64, 872, 8, 64, 10, 64, 12, 64, 875, 9, 64, 1, 65, 1, 65, 1, 65, 1, 
		65, 1, 65, 1, 65, 5, 65, 883, 8, 65, 10, 65, 12, 65, 886, 9, 65, 1, 66, 
		1, 66, 1, 66, 1, 66, 1, 66, 3, 66, 893, 8, 66, 1, 67, 1, 67, 1, 67, 1, 
		67, 3, 67, 899, 8, 67, 1, 68, 1, 68, 1, 68, 1, 68, 1, 68, 1, 68, 1, 69, 
		1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 
		1, 69, 3, 69, 919, 8, 69, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 
		70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 3, 70, 934, 8, 70, 1, 71, 
		1, 71, 1, 71, 1, 71, 1, 72, 1, 72, 1, 73, 1, 73, 1, 73, 5, 73, 945, 8, 
		73, 10, 73, 12, 73, 948, 9, 73, 1, 74, 1, 74, 1, 74, 5, 74, 953, 8, 74, 
		10, 74, 12, 74, 956, 9, 74, 1, 75, 1, 75, 1, 75, 5, 75, 961, 8, 75, 10, 
		75, 12, 75, 964, 9, 75, 1, 76, 1, 76, 1, 76, 5, 76, 969, 8, 76, 10, 76, 
		12, 76, 972, 9, 76, 1, 77, 1, 77, 1, 77, 3, 77, 977, 8, 77, 1, 78, 1, 
		78, 1, 78, 3, 78, 982, 8, 78, 1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 
		3, 79, 990, 8, 79, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 
		80, 1, 80, 3, 80, 1001, 8, 80, 1, 80, 0, 10, 24, 30, 32, 34, 52, 54, 124, 
		126, 128, 130, 81, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 
		28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 
		64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96, 98, 
		100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122, 124, 126, 
		128, 130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 152, 154, 
		156, 158, 160, 0, 18, 1, 0, 1, 4, 1, 0, 5, 12, 1, 0, 163, 171, 1, 0, 164, 
		165, 1, 0, 24, 25, 4, 0, 26, 27, 127, 129, 133, 133, 151, 151, 3, 0, 158, 
		158, 174, 175, 184, 184, 3, 0, 66, 90, 104, 105, 108, 108, 2, 0, 102, 
		103, 106, 107, 6, 0, 91, 96, 99, 100, 109, 109, 112, 113, 115, 118, 122, 
		126, 4, 0, 97, 98, 110, 111, 114, 114, 119, 121, 1, 0, 64, 65, 2, 0, 158, 
		158, 174, 175, 2, 0, 163, 164, 192, 192, 1, 0, 190, 191, 5, 0, 28, 28, 
		30, 30, 32, 32, 40, 40, 42, 42, 5, 0, 29, 29, 31, 31, 33, 33, 41, 41, 
		43, 43, 1, 0, 163, 164, 1086, 0, 165, 1, 0, 0, 0, 2, 169, 1, 0, 0, 0, 
		4, 171, 1, 0, 0, 0, 6, 173, 1, 0, 0, 0, 8, 185, 1, 0, 0, 0, 10, 189, 1, 
		0, 0, 0, 12, 193, 1, 0, 0, 0, 14, 199, 1, 0, 0, 0, 16, 201, 1, 0, 0, 0, 
		18, 237, 1, 0, 0, 0, 20, 252, 1, 0, 0, 0, 22, 266, 1, 0, 0, 0, 24, 274, 
		1, 0, 0, 0, 26, 285, 1, 0, 0, 0, 28, 289, 1, 0, 0, 0, 30, 291, 1, 0, 0, 
		0, 32, 302, 1, 0, 0, 0, 34, 313, 1, 0, 0, 0, 36, 331, 1, 0, 0, 0, 38, 
		342, 1, 0, 0, 0, 40, 344, 1, 0, 0, 0, 42, 351, 1, 0, 0, 0, 44, 362, 1, 
		0, 0, 0, 46, 364, 1, 0, 0, 0, 48, 372, 1, 0, 0, 0, 50, 380, 1, 0, 0, 0, 
		52, 388, 1, 0, 0, 0, 54, 408, 1, 0, 0, 0, 56, 441, 1, 0, 0, 0, 58, 455, 
		1, 0, 0, 0, 60, 485, 1, 0, 0, 0, 62, 497, 1, 0, 0, 0, 64, 499, 1, 0, 0, 
		0, 66, 515, 1, 0, 0, 0, 68, 517, 1, 0, 0, 0, 70, 529, 1, 0, 0, 0, 72, 
		539, 1, 0, 0, 0, 74, 541, 1, 0, 0, 0, 76, 545, 1, 0, 0, 0, 78, 568, 1, 
		0, 0, 0, 80, 570, 1, 0, 0, 0, 82, 583, 1, 0, 0, 0, 84, 585, 1, 0, 0, 0, 
		86, 607, 1, 0, 0, 0, 88, 609, 1, 0, 0, 0, 90, 611, 1, 0, 0, 0, 92, 613, 
		1, 0, 0, 0, 94, 615, 1, 0, 0, 0, 96, 623, 1, 0, 0, 0, 98, 631, 1, 0, 0, 
		0, 100, 764, 1, 0, 0, 0, 102, 771, 1, 0, 0, 0, 104, 781, 1, 0, 0, 0, 106, 
		783, 1, 0, 0, 0, 108, 796, 1, 0, 0, 0, 110, 798, 1, 0, 0, 0, 112, 805, 
		1, 0, 0, 0, 114, 807, 1, 0, 0, 0, 116, 809, 1, 0, 0, 0, 118, 820, 1, 0, 
		0, 0, 120, 828, 1, 0, 0, 0, 122, 833, 1, 0, 0, 0, 124, 844, 1, 0, 0, 0, 
		126, 854, 1, 0, 0, 0, 128, 865, 1, 0, 0, 0, 130, 876, 1, 0, 0, 0, 132, 
		892, 1, 0, 0, 0, 134, 898, 1, 0, 0, 0, 136, 900, 1, 0, 0, 0, 138, 918, 
		1, 0, 0, 0, 140, 933, 1, 0, 0, 0, 142, 935, 1, 0, 0, 0, 144, 939, 1, 0, 
		0, 0, 146, 941, 1, 0, 0, 0, 148, 949, 1, 0, 0, 0, 150, 957, 1, 0, 0, 0, 
		152, 965, 1, 0, 0, 0, 154, 976, 1, 0, 0, 0, 156, 981, 1, 0, 0, 0, 158, 
		989, 1, 0, 0, 0, 160, 1000, 1, 0, 0, 0, 162, 166, 3, 142, 71, 0, 163, 
		166, 3, 144, 72, 0, 164, 166, 3, 124, 62, 0, 165, 162, 1, 0, 0, 0, 165, 
		163, 1, 0, 0, 0, 165, 164, 1, 0, 0, 0, 166, 167, 1, 0, 0, 0, 167, 168, 
		5, 0, 0, 1, 168, 1, 1, 0, 0, 0, 169, 170, 7, 0, 0, 0, 170, 3, 1, 0, 0, 
		0, 171, 172, 7, 1, 0, 0, 172, 5, 1, 0, 0, 0, 173, 174, 5, 158, 0, 0, 174, 
		179, 5, 149, 0, 0, 175, 180, 5, 161, 0, 0, 176, 177, 5, 34, 0, 0, 177, 
		178, 5, 161, 0, 0, 178, 180, 5, 35, 0, 0, 179, 175, 1, 0, 0, 0, 179, 176, 
		1, 0, 0, 0, 180, 7, 1, 0, 0, 0, 181, 186, 3, 28, 14, 0, 182, 186, 3, 60, 
		30, 0, 183, 186, 5, 24, 0, 0, 184, 186, 5, 25, 0, 0, 185, 181, 1, 0, 0, 
		0, 185, 182, 1, 0, 0, 0, 185, 183, 1, 0, 0, 0, 185, 184, 1, 0, 0, 0, 186, 
		187, 1, 0, 0, 0, 187, 188, 3, 6, 3, 0, 188, 9, 1, 0, 0, 0, 189, 190, 3, 
		6, 3, 0, 190, 191, 5, 160, 0, 0, 191, 192, 3, 6, 3, 0, 192, 11, 1, 0, 
		0, 0, 193, 194, 3, 6, 3, 0, 194, 195, 3, 8, 4, 0, 195, 13, 1, 0, 0, 0, 
		196, 200, 3, 12, 6, 0, 197, 200, 3, 8, 4, 0, 198, 200, 3, 10, 5, 0, 199, 
		196, 1, 0, 0, 0, 199, 197, 1, 0, 0, 0, 199, 198, 1, 0, 0, 0, 200, 15, 
		1, 0, 0, 0, 201, 206, 3, 14, 7, 0, 202, 203, 5, 153, 0, 0, 203, 205, 3, 
		14, 7, 0, 204, 202, 1, 0, 0, 0, 205, 208, 1, 0, 0, 0, 206, 204, 1, 0, 
		0, 0, 206, 207, 1, 0, 0, 0, 207, 17, 1, 0, 0, 0, 208, 206, 1, 0, 0, 0, 
		209, 210, 5, 141, 0, 0, 210, 215, 3, 22, 11, 0, 211, 212, 5, 148, 0, 0, 
		212, 214, 3, 22, 11, 0, 213, 211, 1, 0, 0, 0, 214, 217, 1, 0, 0, 0, 215, 
		213, 1, 0, 0, 0, 215, 216, 1, 0, 0, 0, 216, 219, 1, 0, 0, 0, 217, 215, 
		1, 0, 0, 0, 218, 220, 5, 148, 0, 0, 219, 218, 1, 0, 0, 0, 219, 220, 1, 
		0, 0, 0, 220, 221, 1, 0, 0, 0, 221, 222, 5, 142, 0, 0, 222, 238, 1, 0, 
		0, 0, 223, 224, 5, 143, 0, 0, 224, 229, 3, 22, 11, 0, 225, 226, 5, 148, 
		0, 0, 226, 228, 3, 22, 11, 0, 227, 225, 1, 0, 0, 0, 228, 231, 1, 0, 0, 
		0, 229, 227, 1, 0, 0, 0, 229, 230, 1, 0, 0, 0, 230, 233, 1, 0, 0, 0, 231, 
		229, 1, 0, 0, 0, 232, 234, 5, 148, 0, 0, 233, 232, 1, 0, 0, 0, 233, 234, 
		1, 0, 0, 0, 234, 235, 1, 0, 0, 0, 235, 236, 5, 144, 0, 0, 236, 238, 1, 
		0, 0, 0, 237, 209, 1, 0, 0, 0, 237, 223, 1, 0, 0, 0, 238, 250, 1, 0, 0, 
		0, 239, 244, 5, 159, 0, 0, 240, 241, 5, 40, 0, 0, 241, 242, 3, 16, 8, 
		0, 242, 243, 5, 41, 0, 0, 243, 245, 1, 0, 0, 0, 244, 240, 1, 0, 0, 0, 
		244, 245, 1, 0, 0, 0, 245, 246, 1, 0, 0, 0, 246, 247, 5, 34, 0, 0, 247, 
		248, 3, 16, 8, 0, 248, 249, 5, 35, 0, 0, 249, 251, 1, 0, 0, 0, 250, 239, 
		1, 0, 0, 0, 250, 251, 1, 0, 0, 0, 251, 19, 1, 0, 0, 0, 252, 253, 5, 145, 
		0, 0, 253, 258, 3, 22, 11, 0, 254, 255, 5, 148, 0, 0, 255, 257, 3, 22, 
		11, 0, 256, 254, 1, 0, 0, 0, 257, 260, 1, 0, 0, 0, 258, 256, 1, 0, 0, 
		0, 258, 259, 1, 0, 0, 0, 259, 262, 1, 0, 0, 0, 260, 258, 1, 0, 0, 0, 261, 
		263, 5, 148, 0, 0, 262, 261, 1, 0, 0, 0, 262, 263, 1, 0, 0, 0, 263, 264, 
		1, 0, 0, 0, 264, 265, 5, 146, 0, 0, 265, 21, 1, 0, 0, 0, 266, 271, 3, 
		28, 14, 0, 267, 268, 5, 147, 0, 0, 268, 270, 3, 28, 14, 0, 269, 267, 1, 
		0, 0, 0, 270, 273, 1, 0, 0, 0, 271, 269, 1, 0, 0, 0, 271, 272, 1, 0, 0, 
		0, 272, 23, 1, 0, 0, 0, 273, 271, 1, 0, 0, 0, 274, 275, 6, 12, -1, 0, 
		275, 276, 3, 28, 14, 0, 276, 282, 1, 0, 0, 0, 277, 278, 10, 2, 0, 0, 278, 
		279, 7, 2, 0, 0, 279, 281, 3, 24, 12, 3, 280, 277, 1, 0, 0, 0, 281, 284, 
		1, 0, 0, 0, 282, 280, 1, 0, 0, 0, 282, 283, 1, 0, 0, 0, 283, 25, 1, 0, 
		0, 0, 284, 282, 1, 0, 0, 0, 285, 286, 3, 28, 14, 0, 286, 287, 7, 3, 0, 
		0, 287, 288, 3, 28, 14, 0, 288, 27, 1, 0, 0, 0, 289, 290, 3, 30, 15, 0, 
		290, 29, 1, 0, 0, 0, 291, 292, 6, 15, -1, 0, 292, 293, 3, 32, 16, 0, 293, 
		299, 1, 0, 0, 0, 294, 295, 10, 2, 0, 0, 295, 296, 7, 4, 0, 0, 296, 298, 
		3, 30, 15, 3, 297, 294, 1, 0, 0, 0, 298, 301, 1, 0, 0, 0, 299, 297, 1, 
		0, 0, 0, 299, 300, 1, 0, 0, 0, 300, 31, 1, 0, 0, 0, 301, 299, 1, 0, 0, 
		0, 302, 303, 6, 16, -1, 0, 303, 304, 3, 36, 18, 0, 304, 310, 1, 0, 0, 
		0, 305, 306, 10, 2, 0, 0, 306, 307, 7, 5, 0, 0, 307, 309, 3, 32, 16, 3, 
		308, 305, 1, 0, 0, 0, 309, 312, 1, 0, 0, 0, 310, 308, 1, 0, 0, 0, 310, 
		311, 1, 0, 0, 0, 311, 33, 1, 0, 0, 0, 312, 310, 1, 0, 0, 0, 313, 314, 
		6, 17, -1, 0, 314, 315, 3, 38, 19, 0, 315, 321, 1, 0, 0, 0, 316, 317, 
		10, 2, 0, 0, 317, 318, 7, 5, 0, 0, 318, 320, 3, 34, 17, 3, 319, 316, 1, 
		0, 0, 0, 320, 323, 1, 0, 0, 0, 321, 319, 1, 0, 0, 0, 321, 322, 1, 0, 0, 
		0, 322, 35, 1, 0, 0, 0, 323, 321, 1, 0, 0, 0, 324, 325, 7, 4, 0, 0, 325, 
		332, 3, 36, 18, 0, 326, 328, 3, 40, 20, 0, 327, 326, 1, 0, 0, 0, 328, 
		329, 1, 0, 0, 0, 329, 327, 1, 0, 0, 0, 329, 330, 1, 0, 0, 0, 330, 332, 
		1, 0, 0, 0, 331, 324, 1, 0, 0, 0, 331, 327, 1, 0, 0, 0, 332, 37, 1, 0, 
		0, 0, 333, 334, 7, 4, 0, 0, 334, 343, 3, 38, 19, 0, 335, 339, 3, 40, 20, 
		0, 336, 338, 3, 42, 21, 0, 337, 336, 1, 0, 0, 0, 338, 341, 1, 0, 0, 0, 
		339, 337, 1, 0, 0, 0, 339, 340, 1, 0, 0, 0, 340, 343, 1, 0, 0, 0, 341, 
		339, 1, 0, 0, 0, 342, 333, 1, 0, 0, 0, 342, 335, 1, 0, 0, 0, 343, 39, 
		1, 0, 0, 0, 344, 348, 3, 52, 26, 0, 345, 347, 3, 44, 22, 0, 346, 345, 
		1, 0, 0, 0, 347, 350, 1, 0, 0, 0, 348, 346, 1, 0, 0, 0, 348, 349, 1, 0, 
		0, 0, 349, 41, 1, 0, 0, 0, 350, 348, 1, 0, 0, 0, 351, 355, 3, 54, 27, 
		0, 352, 354, 3, 44, 22, 0, 353, 352, 1, 0, 0, 0, 354, 357, 1, 0, 0, 0, 
		355, 353, 1, 0, 0, 0, 355, 356, 1, 0, 0, 0, 356, 43, 1, 0, 0, 0, 357, 
		355, 1, 0, 0, 0, 358, 363, 5, 172, 0, 0, 359, 363, 3, 46, 23, 0, 360, 
		363, 3, 2, 1, 0, 361, 363, 3, 4, 2, 0, 362, 358, 1, 0, 0, 0, 362, 359, 
		1, 0, 0, 0, 362, 360, 1, 0, 0, 0, 362, 361, 1, 0, 0, 0, 363, 45, 1, 0, 
		0, 0, 364, 370, 5, 46, 0, 0, 365, 371, 3, 50, 25, 0, 366, 371, 3, 48, 
		24, 0, 367, 368, 3, 50, 25, 0, 368, 369, 3, 48, 24, 0, 369, 371, 1, 0, 
		0, 0, 370, 365, 1, 0, 0, 0, 370, 366, 1, 0, 0, 0, 370, 367, 1, 0, 0, 0, 
		371, 47, 1, 0, 0, 0, 372, 373, 5, 149, 0, 0, 373, 376, 5, 34, 0, 0, 374, 
		377, 3, 28, 14, 0, 375, 377, 3, 26, 13, 0, 376, 374, 1, 0, 0, 0, 376, 
		375, 1, 0, 0, 0, 377, 378, 1, 0, 0, 0, 378, 379, 5, 35, 0, 0, 379, 49, 
		1, 0, 0, 0, 380, 381, 5, 150, 0, 0, 381, 384, 5, 34, 0, 0, 382, 385, 3, 
		28, 14, 0, 383, 385, 3, 26, 13, 0, 384, 382, 1, 0, 0, 0, 384, 383, 1, 
		0, 0, 0, 385, 386, 1, 0, 0, 0, 386, 387, 5, 35, 0, 0, 387, 51, 1, 0, 0, 
		0, 388, 389, 6, 26, -1, 0, 389, 390, 3, 56, 28, 0, 390, 405, 1, 0, 0, 
		0, 391, 392, 10, 2, 0, 0, 392, 398, 5, 150, 0, 0, 393, 399, 3, 82, 41, 
		0, 394, 395, 5, 34, 0, 0, 395, 396, 3, 28, 14, 0, 396, 397, 5, 35, 0, 
		0, 397, 399, 1, 0, 0, 0, 398, 393, 1, 0, 0, 0, 398, 394, 1, 0, 0, 0, 399, 
		401, 1, 0, 0, 0, 400, 402, 3, 116, 58, 0, 401, 400, 1, 0, 0, 0, 401, 402, 
		1, 0, 0, 0, 402, 404, 1, 0, 0, 0, 403, 391, 1, 0, 0, 0, 404, 407, 1, 0, 
		0, 0, 405, 403, 1, 0, 0, 0, 405, 406, 1, 0, 0, 0, 406, 53, 1, 0, 0, 0, 
		407, 405, 1, 0, 0, 0, 408, 409, 6, 27, -1, 0, 409, 410, 3, 58, 29, 0, 
		410, 425, 1, 0, 0, 0, 411, 412, 10, 2, 0, 0, 412, 418, 5, 150, 0, 0, 413, 
		419, 3, 82, 41, 0, 414, 415, 5, 34, 0, 0, 415, 416, 3, 28, 14, 0, 416, 
		417, 5, 35, 0, 0, 417, 419, 1, 0, 0, 0, 418, 413, 1, 0, 0, 0, 418, 414, 
		1, 0, 0, 0, 419, 421, 1, 0, 0, 0, 420, 422, 3, 116, 58, 0, 421, 420, 1, 
		0, 0, 0, 421, 422, 1, 0, 0, 0, 422, 424, 1, 0, 0, 0, 423, 411, 1, 0, 0, 
		0, 424, 427, 1, 0, 0, 0, 425, 423, 1, 0, 0, 0, 425, 426, 1, 0, 0, 0, 426, 
		55, 1, 0, 0, 0, 427, 425, 1, 0, 0, 0, 428, 442, 3, 60, 30, 0, 429, 442, 
		3, 62, 31, 0, 430, 442, 3, 64, 32, 0, 431, 442, 3, 66, 33, 0, 432, 442, 
		3, 68, 34, 0, 433, 442, 3, 70, 35, 0, 434, 442, 3, 72, 36, 0, 435, 442, 
		3, 100, 50, 0, 436, 442, 3, 82, 41, 0, 437, 442, 3, 84, 42, 0, 438, 442, 
		3, 86, 43, 0, 439, 442, 3, 18, 9, 0, 440, 442, 3, 20, 10, 0, 441, 428, 
		1, 0, 0, 0, 441, 429, 1, 0, 0, 0, 441, 430, 1, 0, 0, 0, 441, 431, 1, 0, 
		0, 0, 441, 432, 1, 0, 0, 0, 441, 433, 1, 0, 0, 0, 441, 434, 1, 0, 0, 0, 
		441, 435, 1, 0, 0, 0, 441, 436, 1, 0, 0, 0, 441, 437, 1, 0, 0, 0, 441, 
		438, 1, 0, 0, 0, 441, 439, 1, 0, 0, 0, 441, 440, 1, 0, 0, 0, 442, 57, 
		1, 0, 0, 0, 443, 456, 3, 60, 30, 0, 444, 456, 3, 62, 31, 0, 445, 456, 
		3, 64, 32, 0, 446, 456, 3, 66, 33, 0, 447, 456, 3, 68, 34, 0, 448, 456, 
		3, 70, 35, 0, 449, 456, 3, 72, 36, 0, 450, 456, 3, 82, 41, 0, 451, 456, 
		3, 84, 42, 0, 452, 456, 3, 86, 43, 0, 453, 456, 3, 18, 9, 0, 454, 456, 
		3, 20, 10, 0, 455, 443, 1, 0, 0, 0, 455, 444, 1, 0, 0, 0, 455, 445, 1, 
		0, 0, 0, 455, 446, 1, 0, 0, 0, 455, 447, 1, 0, 0, 0, 455, 448, 1, 0, 0, 
		0, 455, 449, 1, 0, 0, 0, 455, 450, 1, 0, 0, 0, 455, 451, 1, 0, 0, 0, 455, 
		452, 1, 0, 0, 0, 455, 453, 1, 0, 0, 0, 455, 454, 1, 0, 0, 0, 456, 59, 
		1, 0, 0, 0, 457, 458, 5, 28, 0, 0, 458, 459, 3, 28, 14, 0, 459, 460, 5, 
		29, 0, 0, 460, 486, 1, 0, 0, 0, 461, 462, 5, 32, 0, 0, 462, 463, 3, 28, 
		14, 0, 463, 464, 5, 33, 0, 0, 464, 486, 1, 0, 0, 0, 465, 466, 5, 34, 0, 
		0, 466, 467, 3, 28, 14, 0, 467, 468, 5, 35, 0, 0, 468, 486, 1, 0, 0, 0, 
		469, 470, 5, 40, 0, 0, 470, 471, 3, 28, 14, 0, 471, 472, 5, 41, 0, 0, 
		472, 486, 1, 0, 0, 0, 473, 474, 5, 36, 0, 0, 474, 475, 3, 28, 14, 0, 475, 
		476, 5, 37, 0, 0, 476, 486, 1, 0, 0, 0, 477, 478, 5, 38, 0, 0, 478, 479, 
		3, 28, 14, 0, 479, 480, 5, 39, 0, 0, 480, 486, 1, 0, 0, 0, 481, 482, 5, 
		42, 0, 0, 482, 483, 3, 28, 14, 0, 483, 484, 5, 43, 0, 0, 484, 486, 1, 
		0, 0, 0, 485, 457, 1, 0, 0, 0, 485, 461, 1, 0, 0, 0, 485, 465, 1, 0, 0, 
		0, 485, 469, 1, 0, 0, 0, 485, 473, 1, 0, 0, 0, 485, 477, 1, 0, 0, 0, 485, 
		481, 1, 0, 0, 0, 486, 61, 1, 0, 0, 0, 487, 488, 5, 44, 0, 0, 488, 489, 
		5, 34, 0, 0, 489, 490, 3, 28, 14, 0, 490, 491, 5, 35, 0, 0, 491, 498, 
		1, 0, 0, 0, 492, 493, 5, 45, 0, 0, 493, 494, 5, 34, 0, 0, 494, 495, 3, 
		28, 14, 0, 495, 496, 5, 35, 0, 0, 496, 498, 1, 0, 0, 0, 497, 487, 1, 0, 
		0, 0, 497, 492, 1, 0, 0, 0, 498, 63, 1, 0, 0, 0, 499, 500, 5, 50, 0, 0, 
		500, 501, 3, 28, 14, 0, 501, 502, 5, 50, 0, 0, 502, 65, 1, 0, 0, 0, 503, 
		504, 5, 46, 0, 0, 504, 505, 3, 28, 14, 0, 505, 506, 5, 46, 0, 0, 506, 
		516, 1, 0, 0, 0, 507, 508, 5, 47, 0, 0, 508, 509, 3, 28, 14, 0, 509, 510, 
		5, 48, 0, 0, 510, 516, 1, 0, 0, 0, 511, 512, 5, 49, 0, 0, 512, 513, 3, 
		28, 14, 0, 513, 514, 5, 49, 0, 0, 514, 516, 1, 0, 0, 0, 515, 503, 1, 0, 
		0, 0, 515, 507, 1, 0, 0, 0, 515, 511, 1, 0, 0, 0, 516, 67, 1, 0, 0, 0, 
		517, 518, 5, 51, 0, 0, 518, 519, 3, 28, 14, 0, 519, 520, 5, 52, 0, 0, 
		520, 69, 1, 0, 0, 0, 521, 522, 5, 53, 0, 0, 522, 523, 3, 28, 14, 0, 523, 
		524, 5, 54, 0, 0, 524, 530, 1, 0, 0, 0, 525, 526, 5, 55, 0, 0, 526, 527, 
		3, 28, 14, 0, 527, 528, 5, 56, 0, 0, 528, 530, 1, 0, 0, 0, 529, 521, 1, 
		0, 0, 0, 529, 525, 1, 0, 0, 0, 530, 71, 1, 0, 0, 0, 531, 532, 5, 57, 0, 
		0, 532, 533, 3, 28, 14, 0, 533, 534, 5, 58, 0, 0, 534, 540, 1, 0, 0, 0, 
		535, 536, 5, 59, 0, 0, 536, 537, 3, 28, 14, 0, 537, 538, 5, 60, 0, 0, 
		538, 540, 1, 0, 0, 0, 539, 531, 1, 0, 0, 0, 539, 535, 1, 0, 0, 0, 540, 
		73, 1, 0, 0, 0, 541, 543, 7, 6, 0, 0, 542, 544, 3, 116, 58, 0, 543, 542, 
		1, 0, 0, 0, 543, 544, 1, 0, 0, 0, 544, 75, 1, 0, 0, 0, 545, 554, 7, 6, 
		0, 0, 546, 547, 3, 118, 59, 0, 547, 548, 3, 116, 58, 0, 548, 555, 1, 0, 
		0, 0, 549, 550, 3, 116, 58, 0, 550, 551, 3, 118, 59, 0, 551, 555, 1, 0, 
		0, 0, 552, 555, 3, 116, 58, 0, 553, 555, 3, 118, 59, 0, 554, 546, 1, 0, 
		0, 0, 554, 549, 1, 0, 0, 0, 554, 552, 1, 0, 0, 0, 554, 553, 1, 0, 0, 0, 
		554, 555, 1, 0, 0, 0, 555, 77, 1, 0, 0, 0, 556, 557, 5, 28, 0, 0, 557, 
		562, 3, 76, 38, 0, 558, 559, 5, 153, 0, 0, 559, 561, 3, 76, 38, 0, 560, 
		558, 1, 0, 0, 0, 561, 564, 1, 0, 0, 0, 562, 560, 1, 0, 0, 0, 562, 563, 
		1, 0, 0, 0, 563, 565, 1, 0, 0, 0, 564, 562, 1, 0, 0, 0, 565, 566, 5, 29, 
		0, 0, 566, 569, 1, 0, 0, 0, 567, 569, 3, 76, 38, 0, 568, 556, 1, 0, 0, 
		0, 568, 567, 1, 0, 0, 0, 569, 79, 1, 0, 0, 0, 570, 572, 5, 161, 0, 0, 
		571, 573, 3, 116, 58, 0, 572, 571, 1, 0, 0, 0, 572, 573, 1, 0, 0, 0, 573, 
		81, 1, 0, 0, 0, 574, 584, 3, 76, 38, 0, 575, 584, 5, 176, 0, 0, 576, 584, 
		3, 80, 40, 0, 577, 584, 5, 173, 0, 0, 578, 584, 5, 162, 0, 0, 579, 584, 
		5, 157, 0, 0, 580, 584, 5, 155, 0, 0, 581, 584, 5, 177, 0, 0, 582, 584, 
		5, 75, 0, 0, 583, 574, 1, 0, 0, 0, 583, 575, 1, 0, 0, 0, 583, 576, 1, 
		0, 0, 0, 583, 577, 1, 0, 0, 0, 583, 578, 1, 0, 0, 0, 583, 579, 1, 0, 0, 
		0, 583, 580, 1, 0, 0, 0, 583, 581, 1, 0, 0, 0, 583, 582, 1, 0, 0, 0, 584, 
		83, 1, 0, 0, 0, 585, 586, 5, 130, 0, 0, 586, 587, 5, 34, 0, 0, 587, 588, 
		3, 28, 14, 0, 588, 589, 5, 35, 0, 0, 589, 590, 5, 34, 0, 0, 590, 591, 
		3, 28, 14, 0, 591, 592, 5, 35, 0, 0, 592, 85, 1, 0, 0, 0, 593, 594, 5, 
		34, 0, 0, 594, 595, 3, 28, 14, 0, 595, 596, 5, 132, 0, 0, 596, 597, 3, 
		28, 14, 0, 597, 598, 5, 35, 0, 0, 598, 608, 1, 0, 0, 0, 599, 600, 5, 131, 
		0, 0, 600, 601, 5, 34, 0, 0, 601, 602, 3, 28, 14, 0, 602, 603, 5, 35, 
		0, 0, 603, 604, 5, 34, 0, 0, 604, 605, 3, 28, 14, 0, 605, 606, 5, 35, 
		0, 0, 606, 608, 1, 0, 0, 0, 607, 593, 1, 0, 0, 0, 607, 599, 1, 0, 0, 0, 
		608, 87, 1, 0, 0, 0, 609, 610, 7, 7, 0, 0, 610, 89, 1, 0, 0, 0, 611, 612, 
		7, 8, 0, 0, 612, 91, 1, 0, 0, 0, 613, 614, 7, 9, 0, 0, 614, 93, 1, 0, 
		0, 0, 615, 616, 7, 10, 0, 0, 616, 95, 1, 0, 0, 0, 617, 624, 3, 88, 44, 
		0, 618, 619, 5, 135, 0, 0, 619, 620, 5, 34, 0, 0, 620, 621, 3, 92, 46, 
		0, 621, 622, 5, 35, 0, 0, 622, 624, 1, 0, 0, 0, 623, 617, 1, 0, 0, 0, 
		623, 618, 1, 0, 0, 0, 624, 97, 1, 0, 0, 0, 625, 632, 3, 90, 45, 0, 626, 
		627, 5, 135, 0, 0, 627, 628, 5, 34, 0, 0, 628, 629, 3, 94, 47, 0, 629, 
		630, 5, 35, 0, 0, 630, 632, 1, 0, 0, 0, 631, 625, 1, 0, 0, 0, 631, 626, 
		1, 0, 0, 0, 632, 99, 1, 0, 0, 0, 633, 646, 3, 96, 48, 0, 634, 636, 3, 
		116, 58, 0, 635, 634, 1, 0, 0, 0, 635, 636, 1, 0, 0, 0, 636, 638, 1, 0, 
		0, 0, 637, 639, 3, 118, 59, 0, 638, 637, 1, 0, 0, 0, 638, 639, 1, 0, 0, 
		0, 639, 647, 1, 0, 0, 0, 640, 642, 3, 118, 59, 0, 641, 640, 1, 0, 0, 0, 
		641, 642, 1, 0, 0, 0, 642, 644, 1, 0, 0, 0, 643, 645, 3, 116, 58, 0, 644, 
		643, 1, 0, 0, 0, 644, 645, 1, 0, 0, 0, 645, 647, 1, 0, 0, 0, 646, 635, 
		1, 0, 0, 0, 646, 641, 1, 0, 0, 0, 647, 653, 1, 0, 0, 0, 648, 649, 5, 28, 
		0, 0, 649, 650, 3, 108, 54, 0, 650, 651, 5, 29, 0, 0, 651, 654, 1, 0, 
		0, 0, 652, 654, 3, 110, 55, 0, 653, 648, 1, 0, 0, 0, 653, 652, 1, 0, 0, 
		0, 654, 765, 1, 0, 0, 0, 655, 668, 3, 98, 49, 0, 656, 658, 3, 116, 58, 
		0, 657, 656, 1, 0, 0, 0, 657, 658, 1, 0, 0, 0, 658, 660, 1, 0, 0, 0, 659, 
		661, 3, 118, 59, 0, 660, 659, 1, 0, 0, 0, 660, 661, 1, 0, 0, 0, 661, 669, 
		1, 0, 0, 0, 662, 664, 3, 118, 59, 0, 663, 662, 1, 0, 0, 0, 663, 664, 1, 
		0, 0, 0, 664, 666, 1, 0, 0, 0, 665, 667, 3, 116, 58, 0, 666, 665, 1, 0, 
		0, 0, 666, 667, 1, 0, 0, 0, 667, 669, 1, 0, 0, 0, 668, 657, 1, 0, 0, 0, 
		668, 663, 1, 0, 0, 0, 669, 675, 1, 0, 0, 0, 670, 671, 5, 28, 0, 0, 671, 
		672, 3, 112, 56, 0, 672, 673, 5, 29, 0, 0, 673, 676, 1, 0, 0, 0, 674, 
		676, 3, 114, 57, 0, 675, 670, 1, 0, 0, 0, 675, 674, 1, 0, 0, 0, 676, 765, 
		1, 0, 0, 0, 677, 679, 3, 74, 37, 0, 678, 680, 3, 118, 59, 0, 679, 678, 
		1, 0, 0, 0, 679, 680, 1, 0, 0, 0, 680, 701, 1, 0, 0, 0, 681, 682, 5, 28, 
		0, 0, 682, 683, 3, 104, 52, 0, 683, 684, 5, 29, 0, 0, 684, 702, 1, 0, 
		0, 0, 685, 686, 5, 40, 0, 0, 686, 687, 3, 104, 52, 0, 687, 688, 5, 41, 
		0, 0, 688, 702, 1, 0, 0, 0, 689, 690, 5, 34, 0, 0, 690, 691, 5, 28, 0, 
		0, 691, 692, 3, 104, 52, 0, 692, 693, 5, 29, 0, 0, 693, 694, 5, 35, 0, 
		0, 694, 702, 1, 0, 0, 0, 695, 696, 5, 34, 0, 0, 696, 697, 5, 40, 0, 0, 
		697, 698, 3, 104, 52, 0, 698, 699, 5, 41, 0, 0, 699, 700, 5, 35, 0, 0, 
		700, 702, 1, 0, 0, 0, 701, 681, 1, 0, 0, 0, 701, 685, 1, 0, 0, 0, 701, 
		689, 1, 0, 0, 0, 701, 695, 1, 0, 0, 0, 702, 765, 1, 0, 0, 0, 703, 724, 
		5, 63, 0, 0, 704, 705, 3, 116, 58, 0, 705, 706, 3, 118, 59, 0, 706, 725, 
		1, 0, 0, 0, 707, 708, 3, 118, 59, 0, 708, 709, 3, 116, 58, 0, 709, 725, 
		1, 0, 0, 0, 710, 711, 5, 149, 0, 0, 711, 712, 5, 34, 0, 0, 712, 713, 5, 
		35, 0, 0, 713, 714, 1, 0, 0, 0, 714, 715, 5, 150, 0, 0, 715, 716, 5, 34, 
		0, 0, 716, 725, 5, 35, 0, 0, 717, 718, 5, 150, 0, 0, 718, 719, 5, 34, 
		0, 0, 719, 720, 5, 35, 0, 0, 720, 721, 1, 0, 0, 0, 721, 722, 5, 149, 0, 
		0, 722, 723, 5, 34, 0, 0, 723, 725, 5, 35, 0, 0, 724, 704, 1, 0, 0, 0, 
		724, 707, 1, 0, 0, 0, 724, 710, 1, 0, 0, 0, 724, 717, 1, 0, 0, 0, 724, 
		725, 1, 0, 0, 0, 725, 732, 1, 0, 0, 0, 726, 728, 3, 30, 15, 0, 727, 726, 
		1, 0, 0, 0, 727, 728, 1, 0, 0, 0, 728, 729, 1, 0, 0, 0, 729, 733, 5, 155, 
		0, 0, 730, 733, 3, 84, 42, 0, 731, 733, 3, 30, 15, 0, 732, 727, 1, 0, 
		0, 0, 732, 730, 1, 0, 0, 0, 732, 731, 1, 0, 0, 0, 733, 765, 1, 0, 0, 0, 
		734, 739, 5, 101, 0, 0, 735, 736, 5, 40, 0, 0, 736, 737, 3, 28, 14, 0, 
		737, 738, 5, 41, 0, 0, 738, 740, 1, 0, 0, 0, 739, 735, 1, 0, 0, 0, 739, 
		740, 1, 0, 0, 0, 740, 741, 1, 0, 0, 0, 741, 742, 5, 34, 0, 0, 742, 743, 
		3, 28, 14, 0, 743, 744, 5, 35, 0, 0, 744, 765, 1, 0, 0, 0, 745, 752, 7, 
		11, 0, 0, 746, 747, 3, 120, 60, 0, 747, 748, 3, 118, 59, 0, 748, 753, 
		1, 0, 0, 0, 749, 750, 3, 118, 59, 0, 750, 751, 3, 120, 60, 0, 751, 753, 
		1, 0, 0, 0, 752, 746, 1, 0, 0, 0, 752, 749, 1, 0, 0, 0, 753, 754, 1, 0, 
		0, 0, 754, 755, 3, 32, 16, 0, 755, 765, 1, 0, 0, 0, 756, 757, 5, 61, 0, 
		0, 757, 758, 3, 106, 53, 0, 758, 759, 3, 32, 16, 0, 759, 765, 1, 0, 0, 
		0, 760, 762, 5, 156, 0, 0, 761, 763, 3, 118, 59, 0, 762, 761, 1, 0, 0, 
		0, 762, 763, 1, 0, 0, 0, 763, 765, 1, 0, 0, 0, 764, 633, 1, 0, 0, 0, 764, 
		655, 1, 0, 0, 0, 764, 677, 1, 0, 0, 0, 764, 703, 1, 0, 0, 0, 764, 734, 
		1, 0, 0, 0, 764, 745, 1, 0, 0, 0, 764, 756, 1, 0, 0, 0, 764, 760, 1, 0, 
		0, 0, 765, 101, 1, 0, 0, 0, 766, 767, 3, 28, 14, 0, 767, 768, 5, 153, 
		0, 0, 768, 769, 3, 102, 51, 0, 769, 772, 1, 0, 0, 0, 770, 772, 3, 28, 
		14, 0, 771, 766, 1, 0, 0, 0, 771, 770, 1, 0, 0, 0, 772, 103, 1, 0, 0, 
		0, 773, 782, 3, 82, 41, 0, 774, 775, 3, 28, 14, 0, 775, 776, 5, 153, 0, 
		0, 776, 782, 1, 0, 0, 0, 777, 778, 3, 28, 14, 0, 778, 779, 5, 153, 0, 
		0, 779, 780, 3, 102, 51, 0, 780, 782, 1, 0, 0, 0, 781, 773, 1, 0, 0, 0, 
		781, 774, 1, 0, 0, 0, 781, 777, 1, 0, 0, 0, 782, 105, 1, 0, 0, 0, 783, 
		784, 5, 149, 0, 0, 784, 785, 5, 34, 0, 0, 785, 786, 7, 12, 0, 0, 786, 
		787, 5, 62, 0, 0, 787, 792, 3, 28, 14, 0, 788, 789, 5, 150, 0, 0, 789, 
		790, 5, 34, 0, 0, 790, 791, 7, 4, 0, 0, 791, 793, 5, 35, 0, 0, 792, 788, 
		1, 0, 0, 0, 792, 793, 1, 0, 0, 0, 793, 794, 1, 0, 0, 0, 794, 795, 5, 35, 
		0, 0, 795, 107, 1, 0, 0, 0, 796, 797, 3, 28, 14, 0, 797, 109, 1, 0, 0, 
		0, 798, 799, 3, 34, 17, 0, 799, 111, 1, 0, 0, 0, 800, 806, 3, 28, 14, 
		0, 801, 802, 3, 28, 14, 0, 802, 803, 5, 153, 0, 0, 803, 804, 3, 112, 56, 
		0, 804, 806, 1, 0, 0, 0, 805, 800, 1, 0, 0, 0, 805, 801, 1, 0, 0, 0, 806, 
		113, 1, 0, 0, 0, 807, 808, 3, 34, 17, 0, 808, 115, 1, 0, 0, 0, 809, 818, 
		5, 149, 0, 0, 810, 819, 3, 82, 41, 0, 811, 814, 5, 34, 0, 0, 812, 815, 
		3, 28, 14, 0, 813, 815, 3, 102, 51, 0, 814, 812, 1, 0, 0, 0, 814, 813, 
		1, 0, 0, 0, 815, 816, 1, 0, 0, 0, 816, 817, 5, 35, 0, 0, 817, 819, 1, 
		0, 0, 0, 818, 810, 1, 0, 0, 0, 818, 811, 1, 0, 0, 0, 819, 117, 1, 0, 0, 
		0, 820, 826, 5, 150, 0, 0, 821, 827, 3, 82, 41, 0, 822, 823, 5, 34, 0, 
		0, 823, 824, 3, 28, 14, 0, 824, 825, 5, 35, 0, 0, 825, 827, 1, 0, 0, 0, 
		826, 821, 1, 0, 0, 0, 826, 822, 1, 0, 0, 0, 827, 119, 1, 0, 0, 0, 828, 
		829, 5, 149, 0, 0, 829, 830, 5, 34, 0, 0, 830, 831, 3, 26, 13, 0, 831, 
		832, 5, 35, 0, 0, 832, 121, 1, 0, 0, 0, 833, 834, 5, 149, 0, 0, 834, 835, 
		5, 34, 0, 0, 835, 836, 3, 26, 13, 0, 836, 837, 5, 35, 0, 0, 837, 123, 
		1, 0, 0, 0, 838, 839, 6, 62, -1, 0, 839, 840, 3, 78, 39, 0, 840, 841, 
		7, 13, 0, 0, 841, 842, 3, 124, 62, 2, 842, 845, 1, 0, 0, 0, 843, 845, 
		3, 126, 63, 0, 844, 838, 1, 0, 0, 0, 844, 843, 1, 0, 0, 0, 845, 851, 1, 
		0, 0, 0, 846, 847, 10, 3, 0, 0, 847, 848, 7, 14, 0, 0, 848, 850, 3, 124, 
		62, 4, 849, 846, 1, 0, 0, 0, 850, 853, 1, 0, 0, 0, 851, 849, 1, 0, 0, 
		0, 851, 852, 1, 0, 0, 0, 852, 125, 1, 0, 0, 0, 853, 851, 1, 0, 0, 0, 854, 
		855, 6, 63, -1, 0, 855, 856, 3, 128, 64, 0, 856, 862, 1, 0, 0, 0, 857, 
		858, 10, 2, 0, 0, 858, 859, 5, 187, 0, 0, 859, 861, 3, 126, 63, 3, 860, 
		857, 1, 0, 0, 0, 861, 864, 1, 0, 0, 0, 862, 860, 1, 0, 0, 0, 862, 863, 
		1, 0, 0, 0, 863, 127, 1, 0, 0, 0, 864, 862, 1, 0, 0, 0, 865, 866, 6, 64, 
		-1, 0, 866, 867, 3, 130, 65, 0, 867, 873, 1, 0, 0, 0, 868, 869, 10, 2, 
		0, 0, 869, 870, 5, 185, 0, 0, 870, 872, 3, 128, 64, 3, 871, 868, 1, 0, 
		0, 0, 872, 875, 1, 0, 0, 0, 873, 871, 1, 0, 0, 0, 873, 874, 1, 0, 0, 0, 
		874, 129, 1, 0, 0, 0, 875, 873, 1, 0, 0, 0, 876, 877, 6, 65, -1, 0, 877, 
		878, 3, 132, 66, 0, 878, 884, 1, 0, 0, 0, 879, 880, 10, 2, 0, 0, 880, 
		881, 5, 186, 0, 0, 881, 883, 3, 130, 65, 3, 882, 879, 1, 0, 0, 0, 883, 
		886, 1, 0, 0, 0, 884, 882, 1, 0, 0, 0, 884, 885, 1, 0, 0, 0, 885, 131, 
		1, 0, 0, 0, 886, 884, 1, 0, 0, 0, 887, 888, 5, 28, 0, 0, 888, 889, 3, 
		126, 63, 0, 889, 890, 5, 29, 0, 0, 890, 893, 1, 0, 0, 0, 891, 893, 3, 
		134, 67, 0, 892, 887, 1, 0, 0, 0, 892, 891, 1, 0, 0, 0, 893, 133, 1, 0, 
		0, 0, 894, 899, 3, 136, 68, 0, 895, 899, 3, 160, 80, 0, 896, 899, 3, 138, 
		69, 0, 897, 899, 3, 140, 70, 0, 898, 894, 1, 0, 0, 0, 898, 895, 1, 0, 
		0, 0, 898, 896, 1, 0, 0, 0, 898, 897, 1, 0, 0, 0, 899, 135, 1, 0, 0, 0, 
		900, 901, 7, 15, 0, 0, 901, 902, 3, 28, 14, 0, 902, 903, 5, 153, 0, 0, 
		903, 904, 3, 28, 14, 0, 904, 905, 7, 16, 0, 0, 905, 137, 1, 0, 0, 0, 906, 
		907, 5, 28, 0, 0, 907, 908, 3, 146, 73, 0, 908, 909, 5, 29, 0, 0, 909, 
		919, 1, 0, 0, 0, 910, 911, 5, 30, 0, 0, 911, 912, 3, 146, 73, 0, 912, 
		913, 5, 31, 0, 0, 913, 919, 1, 0, 0, 0, 914, 915, 5, 40, 0, 0, 915, 916, 
		3, 146, 73, 0, 916, 917, 5, 41, 0, 0, 917, 919, 1, 0, 0, 0, 918, 906, 
		1, 0, 0, 0, 918, 910, 1, 0, 0, 0, 918, 914, 1, 0, 0, 0, 919, 139, 1, 0, 
		0, 0, 920, 921, 5, 34, 0, 0, 921, 922, 3, 146, 73, 0, 922, 923, 5, 35, 
		0, 0, 923, 934, 1, 0, 0, 0, 924, 925, 5, 36, 0, 0, 925, 926, 3, 146, 73, 
		0, 926, 927, 5, 37, 0, 0, 927, 934, 1, 0, 0, 0, 928, 929, 5, 45, 0, 0, 
		929, 930, 5, 34, 0, 0, 930, 931, 3, 146, 73, 0, 931, 932, 5, 35, 0, 0, 
		932, 934, 1, 0, 0, 0, 933, 920, 1, 0, 0, 0, 933, 924, 1, 0, 0, 0, 933, 
		928, 1, 0, 0, 0, 934, 141, 1, 0, 0, 0, 935, 936, 3, 78, 39, 0, 936, 937, 
		7, 17, 0, 0, 937, 938, 3, 148, 74, 0, 938, 143, 1, 0, 0, 0, 939, 940, 
		3, 146, 73, 0, 940, 145, 1, 0, 0, 0, 941, 946, 3, 150, 75, 0, 942, 943, 
		5, 152, 0, 0, 943, 945, 3, 150, 75, 0, 944, 942, 1, 0, 0, 0, 945, 948, 
		1, 0, 0, 0, 946, 944, 1, 0, 0, 0, 946, 947, 1, 0, 0, 0, 947, 147, 1, 0, 
		0, 0, 948, 946, 1, 0, 0, 0, 949, 954, 3, 152, 76, 0, 950, 951, 5, 152, 
		0, 0, 951, 953, 3, 152, 76, 0, 952, 950, 1, 0, 0, 0, 953, 956, 1, 0, 0, 
		0, 954, 952, 1, 0, 0, 0, 954, 955, 1, 0, 0, 0, 955, 149, 1, 0, 0, 0, 956, 
		954, 1, 0, 0, 0, 957, 962, 3, 156, 78, 0, 958, 959, 5, 153, 0, 0, 959, 
		961, 3, 156, 78, 0, 960, 958, 1, 0, 0, 0, 961, 964, 1, 0, 0, 0, 962, 960, 
		1, 0, 0, 0, 962, 963, 1, 0, 0, 0, 963, 151, 1, 0, 0, 0, 964, 962, 1, 0, 
		0, 0, 965, 970, 3, 154, 77, 0, 966, 967, 5, 153, 0, 0, 967, 969, 3, 154, 
		77, 0, 968, 966, 1, 0, 0, 0, 969, 972, 1, 0, 0, 0, 970, 968, 1, 0, 0, 
		0, 970, 971, 1, 0, 0, 0, 971, 153, 1, 0, 0, 0, 972, 970, 1, 0, 0, 0, 973, 
		977, 3, 158, 79, 0, 974, 977, 3, 134, 67, 0, 975, 977, 3, 28, 14, 0, 976, 
		973, 1, 0, 0, 0, 976, 974, 1, 0, 0, 0, 976, 975, 1, 0, 0, 0, 977, 155, 
		1, 0, 0, 0, 978, 982, 3, 158, 79, 0, 979, 982, 3, 134, 67, 0, 980, 982, 
		3, 24, 12, 0, 981, 978, 1, 0, 0, 0, 981, 979, 1, 0, 0, 0, 981, 980, 1, 
		0, 0, 0, 982, 157, 1, 0, 0, 0, 983, 984, 3, 28, 14, 0, 984, 985, 5, 188, 
		0, 0, 985, 986, 3, 28, 14, 0, 986, 990, 1, 0, 0, 0, 987, 988, 5, 188, 
		0, 0, 988, 990, 3, 28, 14, 0, 989, 983, 1, 0, 0, 0, 989, 987, 1, 0, 0, 
		0, 990, 159, 1, 0, 0, 0, 991, 1001, 5, 178, 0, 0, 992, 1001, 5, 179, 0, 
		0, 993, 1001, 5, 180, 0, 0, 994, 1001, 5, 181, 0, 0, 995, 1001, 5, 182, 
		0, 0, 996, 1001, 5, 183, 0, 0, 997, 1001, 5, 189, 0, 0, 998, 999, 5, 34, 
		0, 0, 999, 1001, 5, 35, 0, 0, 1000, 991, 1, 0, 0, 0, 1000, 992, 1, 0, 
		0, 0, 1000, 993, 1, 0, 0, 0, 1000, 994, 1, 0, 0, 0, 1000, 995, 1, 0, 0, 
		0, 1000, 996, 1, 0, 0, 0, 1000, 997, 1, 0, 0, 0, 1000, 998, 1, 0, 0, 0, 
		1001, 161, 1, 0, 0, 0, 96, 165, 179, 185, 199, 206, 215, 219, 229, 233, 
		237, 244, 250, 258, 262, 271, 282, 299, 310, 321, 329, 331, 339, 342, 
		348, 355, 362, 370, 376, 384, 398, 401, 405, 418, 421, 425, 441, 455, 
		485, 497, 515, 529, 539, 543, 554, 562, 568, 572, 583, 607, 623, 631, 
		635, 638, 641, 644, 646, 653, 657, 660, 663, 666, 668, 675, 679, 701, 
		724, 727, 732, 739, 752, 762, 764, 771, 781, 792, 805, 814, 818, 826, 
		844, 851, 862, 873, 884, 892, 898, 918, 933, 946, 954, 962, 970, 976, 
		981, 989, 1000
	];
}
