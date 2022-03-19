use grass_ir::WriteFileParam;
use quote::quote;

use super::{Expand, ExpandResult, ExpansionContext, expand_grass_ir};

impl Expand for WriteFileParam {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult {
        match &self.target {
            grass_ir::WriteTarget::FileNo(fd) => {
                let inner = expand_grass_ir(self.what.as_ref(), ctx)?;
                let inner_ref = ctx.get_var_ref(&inner);
                let code = quote! {
                    {
                        #[cfg(unix)]
                        use std::os::unix::io::FromRawFd;
                        use std::io::Write;
                        use grass_runtime::property::Serializable;
                        let mut out_f = unsafe { std::fs::File::from_raw_fd(#fd) };
                        for item in #inner_ref {
                            item.dump(&mut out_f)?;
                            out_f.write_all(b"\n")?;
                        }
                    }
                };
                Ok(ctx.push(code))
            },
            grass_ir::WriteTarget::Path(path) => {
                let inner = expand_grass_ir(self.what.as_ref(), ctx)?;
                let inner_ref = ctx.get_var_ref(&inner);
                let code = quote! {
                    {
                        use std::io::Write;
                        use grass_runtime::property::Serializable;
                        let mut out_f = std::fs::File::open(#path);
                        for item in #inner_ref {
                            item.dump(&mut out_f)?;
                            out_f.write_all(b"\n")?;
                        }
                    }
                };
                Ok(ctx.push(code))
            }
        }
    }
}