use grass_ir::{FormatParam, WriteTarget};
use quote::quote;

use super::{expand_grass_ir, ExpansionContext, ExpandResult, field_expr::expand_field_expr};



pub fn expand_write_record_rec(param: &FormatParam, target: &WriteTarget, ctx: &mut ExpansionContext) -> ExpandResult {
    let inner_ref = expand_grass_ir(&param.expr, ctx)?;
    let inner_var = ctx.get_var_ref(&inner_ref);
    let fmt_str = &param.fmt_str;
    let mut arguments = vec![];
    for (k, v) in param.values.iter() {
        let key_id = syn::Ident::new(k, ctx.span());
        let value = expand_field_expr(v, ctx.span());
        arguments.push(quote!{#key_id  = {
            Some(&item).map(#value).unwrap()
        }})
    }
    let code = match target {
        WriteTarget::FileNo(fd) => quote! {
            {
                #[cfg(unix)]
                use std::os::unix::io::FromRawFd;
                use std::io::Write;
                use grass_runtime::property::Serializable;
                let mut out_f = unsafe { std::fs::File::from_raw_fd(#fd) };
                for item in #inner_var {
                    writeln!(out_f, #fmt_str, #(#arguments,)*)?;
                }
            }
        },
        WriteTarget::Path(path) => quote! {
            {
                use std::io::Write;
                use grass_runtime::property::Serializable;
                let mut out_f = std::fs::File::open(#path);
                for item in #inner_var {
                    writeln!(#fmt_str, #(#arguments,)*)?;
                }
            }
        },
    };
    Ok(ctx.push(code))
}