extern crate proc_macro;

use proc_macro::TokenStream;
use syn::spanned::Spanned;
use quote::quote;

#[proc_macro]
pub fn inline_proc_macro(input: TokenStream) -> TokenStream {
    // Used later to inline the entire interpreted program
    let input_tokens: proc_macro2::TokenStream = input.clone().into();
    let file: syn::File = match syn::parse2(input.into()) {
        Ok(data) => data,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };

    let mut macros = Vec::new();
    for item in &file.items {
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
                return syn::Error::new(file.span(), "unsupported feature")
                    .to_compile_error().into();
            },
            Fn(func) => {
                let is_proc_macro = func.attrs.iter().any(|a| a.path.is_ident("proc_macro"));
                if is_proc_macro {
                    macros.push(&func.ident);
                }
            },
        }
    }

    let input_tokens_copies: Vec<_> = (0..macros.len()).map(|_| input_tokens.clone()).collect();
    let tokens = quote! {
        #(
            macro_rules! #macros {
                ($($args:tt)*) => {
                    oxy::interpret! {
                        { #input_tokens_copies }
                        ( $($args)* )
                    };
                };
            }
        ),*
    };
    tokens.into()
}
