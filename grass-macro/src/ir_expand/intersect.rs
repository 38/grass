use grass_ir::IntersectParam;
use quote::quote;

use super::{Expand, ExpandResult, ExpansionContext, expand_grass_ir};


impl Expand for IntersectParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        if self.sorted {
            let left = expand_grass_ir(self.lhs.as_ref(), ctx)?;
            let right = expand_grass_ir(self.rhs.as_ref(), ctx)?;
            let left_token = ctx.get_var_ref(&left);
            let right_token = ctx.get_var_ref(&right);
            let code = match self.flavor {
                grass_ir::IntersectFlavor::Inner => quote! {
                    {
                        use grass_runtime::algorithm::SortedIntersect;
                        #left_token .sorted_intersect(#right_token)
                    }
                },
                _ => todo!()
            };
            Ok(ctx.push(code))
        } else {
            todo!()
        }
    }
}