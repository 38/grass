use super::{Expand, ExpandResult, ExpansionContext, expand_grass_ir};

use grass_ir::LetBinding;

impl Expand for LetBinding {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner_id = expand_grass_ir(&self.value, ctx)?;
        ctx.symbol_table.insert(self.id.clone(), inner_id);
        Ok(inner_id)
    }
}