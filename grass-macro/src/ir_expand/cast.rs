use grass_ir::CastToBedParam;
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for CastToBedParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner_id = expand_grass_ir(&self.inner, ctx)?;
        let inner = ctx.get_var_ref(&inner_id);
        let bed_variant_id = syn::Ident::new(&format!("Bed{}", self.num_of_fields), ctx.span());
        let post_steps = if self.sorted {
            quote! { let result = result.assume_sorted(); }
        } else {
            quote! { (); }
        };
        let code = quote! {
            {
                use grass_runtime::algorithm::AssumeSorted;
                use grass_runtime::property::*;
                use grass_runtime::record::#bed_variant_id;
                let result = #inner . map (|input| #bed_variant_id::new(&input));
                #post_steps;
                result
            }
        };
        Ok(ctx.push(code))
    }
}
