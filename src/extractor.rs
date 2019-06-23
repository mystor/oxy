use proc_macro2::TokenStream;
use quote::quote;

use crate::error::{OxyError, InterpretError};
use crate::context::{Context, Function};

/// Populates the context with the items declared in the given input. The input should be a Rust
/// token stream in Oxy syntax.
pub fn populate(ctx: &mut Context, input: impl Into<TokenStream>) -> Result<(), OxyError> {
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
            Fn(func) => ctx.add_fn_item(function_from_item(func)),
        }
    }

    Ok(())
}

fn function_from_item(item: syn::ItemFn) -> Function {
    unimplemented!()
}
