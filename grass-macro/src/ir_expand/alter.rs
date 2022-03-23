use grass_ir::AlterParam;
use quote::quote;

use super::{Expand, ExpansionContext, ExpandResult, expand_grass_ir, field_expr::expand_field_expr};


impl Expand for AlterParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner_id = expand_grass_ir(&self.original_expr, ctx)?;
        let inner_var = ctx.get_var_ref(&inner_id);

        let setter_id = syn::Ident::new(&format!("set_{}", self.field), ctx.span());

        let value = expand_field_expr(&self.value, ctx.span());

        let code = quote! {
            {
                #inner_var . map(
                    |mut item| {
                        let new_value = Some(&item).map(#value).unwrap();
                        item . #setter_id (new_value);
                        item
                    }
                )
            }
        };
        Ok(ctx.push(code))
    }
}