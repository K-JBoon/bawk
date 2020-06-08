use crate::ast::*;
use crate::lexer::*;
use crate::lexer::Token::*;

use plex::parser;

parser! {
    fn parse_(Token, Span);

    // combine two spans
    (a, b) {
        Span {
            lo: a.lo,
            hi: b.hi,
        }
    }

    program: Program {
            rules[r] => Program { rules: r }
    }

    rules: Vec<Rule> {
        => vec![],
        rules[mut rls] BeginRule statements[stmts] => {
            rls.push(Rule { rule_type: RuleType::BeginRule, statements: stmts });
            rls
        },
        rules[mut rls] DefaultRule statements[stmts] => {
            rls.push(Rule { rule_type: RuleType::DefaultRule, statements: stmts });
            rls
        },
        rules[mut rls] EndRule statements[stmts] => {
            rls.push(Rule { rule_type: RuleType::EndRule, statements: stmts });
            rls
        }
    }

    statements: Vec<Statement> {
        => vec![],
        statements[mut sts] statement[s] EndOfRule => {
            sts.push(s);
            sts
        }
    }

    statement: Statement {
        => Statement { expressions: vec![] },
        statement[mut st] assign[a] => {
            st.expressions.push(a);
            st
        }
    }

    assign: Expr {
        Print assign[a] Semi => Expr {
            span: span!(),
            node: Expr_::Print(Box::new(a)),
        },
        Ident(var) Equals assign[rhs] Semi => Expr {
            span: span!(),
            node: Expr_::Assign(var, Box::new(rhs)),
        },
        atom[a] => a,
    }

    atom: Expr {
        // round brackets to destructure tokens
        Ident(i) => Expr {
            span: span!(),
            node: Expr_::Var(i),
        },
        Integer(i) => Expr {
            span: span!(),
            node: Expr_::Integer(i),
        },
        Text(t) => Expr {
            span: span!(),
            node: Expr_::Text(t)
        },
        LParen assign[a] RParen => a
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I,
) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}
