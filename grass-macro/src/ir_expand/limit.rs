use grass_ir::{LimitParam, ConstOrEnv, ConstBagRef};
use proc_macro2::Ident;
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for LimitParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let what = expand_grass_ir(self.what.as_ref(), ctx)?;
        let what_id = ctx.get_var_ref(&what);

        let count_tk = match &self.count {
            ConstOrEnv::Const(value) => {
                let value = *value as usize;
                quote!{#value}
            }
            ConstOrEnv::Env(env_var) => {
                let ident = Ident::new(&env_var.get_const_bag_ident(), ctx.span());
                quote! {#ident . value() as usize}
            }
        };

        let code = quote! {
            {
                #what_id . take(#count_tk)
            }
        };

        Ok(ctx.push(code))
    }
}
