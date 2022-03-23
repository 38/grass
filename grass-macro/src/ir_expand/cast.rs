use grass_ir::CastToBedParam;
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for CastToBedParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner_id = expand_grass_ir(&self.inner, ctx)?;
        let inner = ctx.get_var_ref(&inner_id);
        let bed_variant_id = syn::Ident::new(&format!("Bed{}", self.num_of_fields), ctx.span());
        let code = quote! {
            {
                use grass_runtime::property::*;
                use grass_runtime::record::#bed_variant_id;
                #inner . map (|input| #bed_variant_id::new(&input))
            }
        };
        Ok(ctx.push(code))
    }
}
