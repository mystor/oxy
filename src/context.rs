use proc_macro2::TokenStream;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Integer(i32),
    TokenStream(TokenStream),
    Function(Function),
}

#[derive(Debug)]
pub struct Function {
    pub is_proc_macro: bool,
}

#[derive(Debug)]
pub enum Item {
    Function(Function),
    //TODO: Const, static, etc.
}

#[derive(Default, Debug)]
pub struct Context {
    decls: HashMap<String, Item>,
}

impl Context {
    pub fn add_fn_item(&mut self, func: Function) {
    }
}
