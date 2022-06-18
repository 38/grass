use grass_ir::InvertParam;
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for InvertParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner = expand_grass_ir(self.inner.as_ref(), ctx)?;
        let inner_id = ctx.get_var_ref(&inner);
        let code = quote! {
            {
                use grass_runtime::algorithm::SortedInversionExt;
                #inner_id . invert()
            }
        };
        Ok(ctx.push(code.into()))
    }
}
