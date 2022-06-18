use grass_ir::TwoWayMergeParam;
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for TwoWayMergeParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let a = expand_grass_ir(self.expr_1.as_ref(), ctx)?;
        let a_id = ctx.get_var_ref(&a);

        let b = expand_grass_ir(self.expr_2.as_ref(), ctx)?;
        let b_id = ctx.get_var_ref(&b);

        let code = quote! {
            {
                use grass_runtime::algorithm::TwoWayMergeExt;
                #a_id . merge_with(#b_id)
            }
        };
        Ok(ctx.push(code))
    }
}
