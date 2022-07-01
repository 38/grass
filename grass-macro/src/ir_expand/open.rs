use grass_ir::{ConstOrEnv, InputFormat, OpenParam, OpenTarget};
use proc_macro2::{Ident, TokenStream, Span};
use quote::quote;
use syn::LitStr;

use super::{Expand, ExpandResult, ExpansionContext};

fn expand_path(span: Span, target: &OpenTarget) -> Result<TokenStream, u32> {
    match target {
        OpenTarget::Path(ConstOrEnv::Const(path)) => {
            let path = LitStr::new(path, span);
            Ok(quote! { #path })
        }
        OpenTarget::Path(ConstOrEnv::Env(key)) => {
            let path = Ident::new(&key.get_const_bag_ident(), span);
            Ok(quote! { #path.value() })
        }
        OpenTarget::FileNo(fd) => {
            Err(*fd)
        }
        OpenTarget::CmdArg(idx) => {
            Ok(quote! { cmd_args[#idx as usize] })
        }
    }
}

impl Expand for OpenParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        match &self.format {
            InputFormat::Bed => {
                let open_expr = match expand_path(ctx.span(), &self.target) {
                    Ok(path) => quote! { std::fs::File::open(#path)? },
                    Err(0) => quote! { std::io::stdin() },
                    Err(1) => quote! { std::io::stdout() },
                    Err(2) => quote! { std::io::stderr() },
                    Err(fd) => panic!("Unsupported file descriptor #{}", fd),
                };
                if !self.compression {
                    let bed_type_name = format!("Bed{}", self.num_of_fields);
                    let bed_type_id = syn::Ident::new(&bed_type_name, ctx.span());
                    let imports = if self.sorted {
                        quote! {
                            use grass_runtime::LineRecordStreamExt;
                            use grass_runtime::algorithm::AssumeSorted;
                        }
                    } else {
                        quote! {
                            use grass_runtime::LineRecordStreamExt;
                        }
                    };
                    let assume_sorted = if self.sorted {
                        quote! {.assume_sorted()}
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
                } else {
                    todo!()
                }
            }
            InputFormat::Bam => {
                let path = expand_path(ctx.span(), &self.target).expect("Reading bam from pipe isn't supported yet");
                let bam_file = ctx.push(quote!{ 
                    {
                        use grass_runtime::record::BamReader;
                        BamReader::open(#path)?
                    } 
                });
                let bam_file_id = ctx.get_var_ref(&bam_file);

                Ok(ctx.push(quote! {#bam_file_id.iter()}))
            }
            whatever => Err(syn::Error::new(
                ctx.span(),
                format!("Unsupported input file type: {:?}", whatever),
            )),
        }
    }
}
