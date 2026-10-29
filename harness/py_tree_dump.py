import json, sys
from antlr4 import InputStream, CommonTokenStream
from antlr4.error.ErrorListener import ErrorListener
from antlr4.tree.Tree import TerminalNode
from latex2sympy2_extended.antlr_parser import PSParser, PSLexer

class Raise(ErrorListener):
    def syntaxError(self, *a):
        raise Exception("err")

def dump(node):
    if isinstance(node, TerminalNode):
        return node.getText()
    name = PSParser.ruleNames[node.getRuleIndex()]
    parts = [dump(c) for c in (node.children or []) if not (isinstance(c, TerminalNode) and c.getSymbol().type == -1)]
    return "(" + " ".join([name] + parts) + ")"

for s in json.load(open(sys.argv[1], encoding="utf-8")):
    try:
        lex = PSLexer(InputStream(s)); lex.removeErrorListeners(); lex.addErrorListener(Raise())
        p = PSParser(CommonTokenStream(lex)); p.removeErrorListeners(); p.addErrorListener(Raise())
        print(dump(p.math()))
    except Exception:
        print("ERR")
