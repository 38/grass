use grass_ir::{AssignTagParam, TagValue};
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for AssignTagParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner = expand_grass_ir(self.inner.as_ref(), ctx)?;
        let inner_id = ctx.get_var_ref(&inner);
        let tag_token = match &self.tag {
            TagValue::String(s) => quote! { #s },
            TagValue::Int(n) => quote! { #n },
            TagValue::Float(n) => quote! { #n },
        };
        let code = quote! {
            {
                use grass_runtime::algorithm::TaggedIterExt;
                #inner_id . tagged(#tag_token)
            }
        };
        Ok(ctx.push(code))
    }
}
