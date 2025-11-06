use std::collections::HashMap;

pub type Context = HashMap<&'static str, u64>;

pub trait Expr {
    fn exec_expr(&mut self, context: &Context) -> u64;
}

pub trait Stmt {
    fn exec_stmt(&mut self, context: &Context);
}