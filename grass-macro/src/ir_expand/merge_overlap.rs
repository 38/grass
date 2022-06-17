use grass_ir::MergeOverlapParam;
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for MergeOverlapParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner = expand_grass_ir(self.input_expr.as_ref(), ctx)?;
        let inner_id = ctx.get_var_ref(&inner);
        let code = quote! {
            {
                use grass_runtime::algorithm::{Components, AssumeSorted};
                use grass_runtime::property::RegionCore;
                use grass_runtime::record::Bed3;
                use grass_runtime::Itertools;
                let mut cluster_id = 0;
                /*let group_by_cluster = #inner_id
                    .components()
                    .group_by(move |x| {
                        if x.depth == 0 {
                            cluster_id += 1;
                            cluster_id - 1
                        } else {
                            cluster_id
                        }
                    });
                group_by_cluster
                    .into_iter()
                    .map(|(_, mut xs)|{
                        let first = xs.next().unwrap();
                        let last = xs.last().unwrap();
                        Bed3 {
                            chrom: first.chrom(),
                            start: first.start(),
                            end: last.end(),
                        }
                    })*/
                //TODO
            }
        };
        Ok(ctx.push(code.into()))
    }
}
