use grass_ir::{ConstOrEnv, LoadGenomeFileParam};
use proc_macro2::Ident;
use quote::quote;

use super::{Expand, ExpandResult, ExpansionContext};

impl Expand for LoadGenomeFileParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        let code = match self {
            LoadGenomeFileParam::File(ConstOrEnv::Const(path)) => {
                quote! {
                    {
                        use grass_runtime::Genome;
                        Genome::load_genome_file(std::fs::File::open(#path)?)?;
                    }
                }
            }
            LoadGenomeFileParam::File(ConstOrEnv::Env(key)) => {
                let path_tk = Ident::new(&key.get_const_bag_ident(), ctx.span());
                quote! {
                    {
                        use grass_runtime::Genome;
                        Genome::load_genome_file(std::fs::File::open(#path_tk.value())?)?;
                    }
                }
            }
        };
        Ok(ctx.push(code))
    }
}
