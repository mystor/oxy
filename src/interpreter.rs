use proc_macro2::TokenStream;

use crate::context::{Context, Value};
use crate::error::OxyError;

/// Evaluates the function given by path with the given arguments
pub fn eval_fn<TS: Into<TokenStream>>(
    ctx: &mut Context,
    path: impl Into<syn::Path>,
    input: &[TS],
) -> Result<Value, OxyError> {
    unimplemented!();
}
