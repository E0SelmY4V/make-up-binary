use std::fmt::Display;

use crate::Expr;

pub fn display_expr<T: Display>(expr: &Expr<T>) -> String {
    match expr {
        Expr::Not(n) => format!("!{}", display_expr(n)),
        Expr::And(a, b) => format!("({} & {})", display_expr(a), display_expr(b)),
        Expr::Or(a, b) => format!("({} | {})", display_expr(a), display_expr(b)),
        Expr::Exist(n) => format!("{n}"),
        Expr::Magic => String::from("ERROR"),
    }
}
