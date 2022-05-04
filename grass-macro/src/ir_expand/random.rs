use grass_ir::SortedRandomParam;
use quote::quote;

use super::{Expand, ExpandResult, ExpansionContext};

impl Expand for SortedRandomParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let count = self.count;
        let min_len = self.min_length;
        let max_len = self.max_length;
        let code = quote! {
            {
                use grass_runtime::algorithm::SortedRandomInterval;
                SortedRandomInterval::new((#min_len) as usize, (#max_len) as usize, (#count) as usize)
            }
        };
        Ok(ctx.push(code))
    }
}
