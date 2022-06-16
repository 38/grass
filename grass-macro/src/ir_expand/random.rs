use grass_ir::{ConstOrEnv, SortedRandomParam};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use super::{Expand, ExpandResult, ExpansionContext};

fn _expand_value<T: ToTokens>(value: &ConstOrEnv<T>, span: Span) -> TokenStream {
    match value {
        ConstOrEnv::Const(v) => quote! { #v },
        ConstOrEnv::Env(key) => {
            let id = Ident::new(&key.get_const_bag_ident(), span);
            quote! {
                #id.value()
            }
        }
    }
}

impl Expand for SortedRandomParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let count = _expand_value(&self.count, ctx.span());
        let min_len = _expand_value(&self.min_length, ctx.span());
        let max_len = _expand_value(&self.max_length, ctx.span());
        let code = quote! {
            {
                use grass_runtime::algorithm::SortedRandomInterval;
                SortedRandomInterval::new((#min_len) as usize, (#max_len) as usize, (#count) as usize)
            }
        };
        Ok(ctx.push(code))
    }
}
