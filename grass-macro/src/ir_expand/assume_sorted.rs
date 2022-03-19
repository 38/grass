use grass_ir::AssumeSortedParam;
use quote::quote;

use super::{Expand, ExpansionContext, ExpandResult, expand_grass_ir};


impl Expand for AssumeSortedParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner = expand_grass_ir(&self.inner, ctx)?;
        let inner_var = ctx.get_var_ref(&inner);

        let code = quote! {
            {
                use grass_runtime::algorithm::AssumeSorted;
                #inner_var . assume_sorted()   
            }
        };

        Ok(ctx.push(code))
    }
}