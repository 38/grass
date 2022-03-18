use grass_ir::{OpenParam, InputFormat};
use syn::LitStr;
use quote::quote;

use super::{Expand, ExpansionContext, ExpandResult};

impl Expand for OpenParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        match &self.format {
            InputFormat::Bed => {
                let path = LitStr::new(&self.path, ctx.span());
                if !self.compression {
                    let bed_type_name = format!("Bed{}", self.num_of_fields);
                    let bed_type_id = syn::Ident::new(&bed_type_name, ctx.span());
                    let imports = if self.sorted {
                        quote!{
                            use grass_runtime::LineRecordStreamExt;
                            use grass_runtime::algorithm::AssumeSorted;
                        }
                    } else {
                        quote!{
                            use grass_runtime::LineRecordStreamExt;
                        }
                    };
                    let assume_sorted = if self.sorted {
                        quote!{.assume_sorted()}
                    } else {
                        quote! {}
                    };
                    let code = quote! {
                        {
                            #imports
                            std::fs::File::open(#path)?.into_record_iter::<grass_runtime::record::#bed_type_id>() 
                            #assume_sorted
                        }
                    };
                    Ok(ctx.push(code))
                }  else {
                    todo!()
                }
            }
            whatever => {
                Err(syn::Error::new(ctx.span(), format!("Unsupported input file type: {:?}", whatever)))
            }
        }
    }
}