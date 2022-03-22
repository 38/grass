use grass_ir::{OpenParam, InputFormat, OpenTarget};
use syn::LitStr;
use quote::quote;

use super::{Expand, ExpansionContext, ExpandResult};

impl Expand for OpenParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        match &self.format {
            InputFormat::Bed => {
                let open_expr = match &self.target {
                    OpenTarget::Path(path) => {
                        let path = LitStr::new(path, ctx.span());
                        quote! {
                            std::fs::File::open(#path)?
                        }
                    }
                    OpenTarget::FileNo(fd) => {
                        match fd {
                            0 => quote! { std::io::stdin() },
                            1 => quote! { std::io::stdout() },
                            2 => quote! { std::io::stderr() },
                            _ => Err(syn::Error::new(ctx.span(), format!("Unsupported file descriptor #{}", fd)))?
                        }
                    }
                    OpenTarget::CmdArg(idx) => {
                        quote! {
                            std::fs::File::open(cmd_args[#idx as usize])?
                        }
                    }
                };
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
                            (#open_expr).into_record_iter::<grass_runtime::record::#bed_type_id>() 
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
