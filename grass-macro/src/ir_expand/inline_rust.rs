use grass_ir::{InlineRustParam, InlineRustEnviron, InlineRustConst, ConstOrEnv};
use proc_macro2::TokenStream;
use quote::quote;

use crate::ir_expand::expand_grass_ir;

use super::{Expand, ExpandResult, ExpansionContext};

impl Expand for InlineRustParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let mut args = Vec::new();
        let mut vals = Vec::new();
        for (key, val) in self.env.iter() {
            let arg_ident = syn::Ident::new(key, ctx.span());
            //let val_id = expand_grass_ir(val, ctx)?;
            //let val_ident = ctx.get_var_ref(&val_id);
            let val_ident = match val {
                InlineRustEnviron::Iter(iter) => {
                    let expansion = expand_grass_ir(iter, ctx)?;
                    let id = ctx.get_var_ref(&expansion);
                    quote! { #id }
                },
                InlineRustEnviron::Const(ConstOrEnv::Const(InlineRustConst::Float(val))) => {
                    quote! { #val }
                },
                InlineRustEnviron::Const(ConstOrEnv::Const(InlineRustConst::Integer(val))) => {
                    quote! { #val }
                },
                InlineRustEnviron::Const(ConstOrEnv::Const(InlineRustConst::String(val))) => {
                    quote! { #val }
                },
                InlineRustEnviron::Const(ConstOrEnv::Env(env)) => {
                    let id = syn::Ident::new(&env.get_const_bag_ident(), ctx.span());
                    quote! { #id . value() }
                }
            };
            args.push(arg_ident);
            vals.push(val_ident);
        }
        let inline_code: TokenStream = syn::parse_str(self.src.as_str())?;
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
