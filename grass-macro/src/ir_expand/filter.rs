
use grass_ir::FilterParam;
use quote::quote;

use super::{Expand, ExpansionContext, ExpandResult, field_expr::expand_field_expr, expand_grass_ir};

impl Expand for FilterParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let cond = expand_field_expr(&self.cond, ctx.span());
        let inner_id = expand_grass_ir(&self.input_expr, ctx)?;
        let inner = ctx.get_var_ref(&inner_id);
        let code = quote! {
            #inner . filter ( #cond ) 
        };
        Ok(ctx.push(code))
    }
}