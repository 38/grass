use grass_ir::MergeOverlapParam;
use quote::quote;

use super::{expand_grass_ir, Expand, ExpandResult, ExpansionContext};

impl Expand for MergeOverlapParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let inner = expand_grass_ir(self.input_expr.as_ref(), ctx)?;
        let inner_id = ctx.get_var_ref(&inner);
        let code = quote! {
            {
                use grass_runtime::{
                    algorithm::{Components, AssumeSorted},
                    property::RegionCore,
                    record::Bed3,
                    Itertools
                };
                use genawaiter::{sync::gen, yield_};
                let mut cluster_id = 0;
                let merge_gen = gen!({
                    let group_by_cluster = #inner_id
                        .components()
                        .group_by(move |x| {
                            if x.depth == 0 {
                                cluster_id += 1;
                                cluster_id - 1
                            } else {
                                cluster_id
                            }
                        });
                    for (_, mut xs) in group_by_cluster.into_iter() {
                        let first = xs.next().unwrap();
                        let last = xs.last().unwrap();
                        let result = Bed3 {
                            chrom: first.chrom(),
                            start: first.start(),
                            end: last.end(),
                        };
                        yield_!(result)
                    }
                });
                futures::executor::block_on_stream(merge_gen).into_iter().assume_sorted()
            }
        };
        Ok(ctx.push(code.into()))
    }
}
