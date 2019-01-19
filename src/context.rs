use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::error::{OxyError, InterpretError};

#[derive(Debug)]
pub enum Value {
    Function(Function),
}

#[derive(Debug)]
pub struct Function {
    is_proc_macro: bool,
}

impl Function {
    pub fn from_item(item: syn::ItemFn) -> Self {
        Self {
            is_proc_macro: false,
        }
    }
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

        /// Populates the context with the items declared in the given input. The input should be a Rust
    /// token stream in Oxy syntax.
    pub fn populate(&mut self, input: impl Into<TokenStream>) -> Result<(), OxyError> {
        let file: syn::File = syn::parse2(input.into())?;
        if file.shebang.is_some() {
            Err(InterpretError::Unsupported("Shebang line at the top of the file".to_string()))?;
        } else if !file.attrs.is_empty() {
            Err(InterpretError::Unsupported("File-level attributes".to_string()))?;
        }

        for item in file.items {
            use syn::Item::*;
            match item {
                ExternCrate(_) |
                Use(_) |
                Static(_) |
                Const(_) |
                Mod(_) |
                ForeignMod(_) |
                Type(_) |
                Existential(_) |
                Struct(_) |
                Enum(_) |
                Union(_) |
                Trait(_) |
                TraitAlias(_) |
                Impl(_) |
                Macro(_) |
                Macro2(_) |
                Verbatim(_) => {
                    Err(InterpretError::Unsupported(quote!(#item).to_string()))?;
                },
                Fn(func) => self.add_fn_item(Function::from_item(func)),
            }
        }

        Ok(())
    }
}
