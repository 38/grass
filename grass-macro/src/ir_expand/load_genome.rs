use grass_ir::LoadGenomeFileParam;
use quote::quote;

use super::{Expand, ExpandResult, ExpansionContext};

impl Expand for LoadGenomeFileParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let code = match self {
            LoadGenomeFileParam::File(path) => {
                quote! {
                    {
                        use grass_runtime::Genome;
                        Genome::load_genome_file(#path)?;
                    }
                }
            }
        };
        Ok(ctx.push(code))
    }
}
