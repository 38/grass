use grass_ir::CastToBed3Param;
use quote::quote;

use super::{Expand, ExpansionContext, ExpandResult, expand_grass_ir};

impl Expand for CastToBed3Param {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner_id = expand_grass_ir(&self.inner, ctx)?;
        let inner = ctx.get_var_ref(&inner_id);
        let code = quote! {
            {
                use grass_runtime::property::*;
                use grass_runtime::record::Bed3;
                #inner . map (|input| {
                    Bed3 {
                        start: input.start(),
                        end: input.end(),
                        chrom: input.chrom(),
                    }
                })
            }
        };
        Ok(ctx.push(code))
    }
}