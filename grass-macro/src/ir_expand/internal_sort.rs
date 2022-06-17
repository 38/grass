use grass_ir::InternalSortParam;
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for InternalSortParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner = expand_grass_ir(self.inner.as_ref(), ctx)?;
        let inner_id = ctx.get_var_ref(&inner);
        let code = quote! {
            {
                use grass_runtime::property::*;
                use grass_runtime::algorithm::AssumeSorted;
                let mut buffer: Vec<_> = #inner_id.collect();
                buffer.sort_unstable();
                buffer.into_iter().assume_sorted()
            }
        };
        Ok(ctx.push(code.into()))
    }
}
