use grass_ir::InlineRustParam;
use proc_macro2::TokenStream;
use quote::quote;

use crate::ir_expand::expand_grass_ir;

use super::{Expand, ExpansionContext, ExpandResult};

impl Expand for InlineRustParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let mut args = Vec::new();
        let mut vals = Vec::new();
        for (key, val) in self.env.iter() {
            let arg_ident = syn::Ident::new(key, ctx.span());
            let val_id = expand_grass_ir(val, ctx)?;
            let val_ident = ctx.get_var_ref(&val_id);
            args.push(arg_ident);
            vals.push(val_ident);
        }
        let inline_code : TokenStream = syn::parse_str(self.src.as_str())?;
        let code = quote! {
            {
                let _temp = std::iter::once((#(#vals,)*)).map(|(#(#args,)*)|{
                    #inline_code
                }).next();
                unsafe { _temp.unwrap_unchecked() }
            }
        };
        Ok(ctx.push(code))
    }
}