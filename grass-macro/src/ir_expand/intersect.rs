use grass_ir::{IntersectFlavor, IntersectParam};
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for IntersectParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        if self.sorted {
            let left = expand_grass_ir(self.lhs.as_ref(), ctx)?;
            let right = expand_grass_ir(self.rhs.as_ref(), ctx)?;
            let left_token = ctx.get_var_ref(&left);
            let right_token = ctx.get_var_ref(&right);
            let code = match self.flavor {
                IntersectFlavor::Inner => quote! {
                    {
                        use grass_runtime::algorithm::SortedIntersect;
                        #left_token .sorted_intersect(#right_token)
                    }
                },
                IntersectFlavor::LeftOuter => quote! {
                    {
                        use grass_runtime::algorithm::SortedIntersect;
                        #left_token . sorted_left_outer_intersect(#right_token)
                    }
                },
                IntersectFlavor::RightOuter => quote! {
                    {
                        use grass_runtime::algorithm::{SortedIntersect, AssumeSorted};
                        #right_token . sorted_left_outer_intersect(#left_token) . map (|i| (i.1, i.0)) . assume_sorted()
                    }
                },
                _ => todo!(),
            };
            Ok(ctx.push(code))
        } else {
            todo!("unsorted intersect is not implemented yet")
        }
    }
}
