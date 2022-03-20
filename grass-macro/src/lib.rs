use std::{fs::File, io::Read, path::Path};

use grass_ir::GrassIR;
use ir_expand::ExpansionContext;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, LitStr};

mod ir_expand;

fn grass_impl(ir: &str, span: Span) -> TokenStream {
    let ir: GrassIR = match serde_json::from_str(ir) {
        Err(e) => {
            return syn::Error::new(span, format!("Unable to parse Grass IR {}", e.to_string()))
                .to_compile_error()
                .into()
        }
        Ok(ir) => ir,
    };

    let mut ctx = ExpansionContext::new(span);

    let result = match ir_expand::expand_grass_ir(&ir, &mut ctx) {
        Ok(_code) => {
            let code = ctx.to_token_stream();
            quote! {
                #code;
            }
        }
        Err(err) => err.into_compile_error()
    };

    result.into()
}

#[proc_macro]
pub fn import_grass_ir_from_file(input: TokenStream) -> TokenStream {
    let ir_lit = parse_macro_input!(input as LitStr);
    let ir_lit_val = ir_lit.value();
    let ir_path_rel: &Path = ir_lit_val.as_ref();

    let manifest_path_str = env!("CARGO_MANIFEST_DIR");
    let manifest_path: &Path = manifest_path_str.as_ref();

    let ir_path = if ir_path_rel.is_relative() {
        let mut pb = manifest_path.to_path_buf();
        pb.push(ir_path_rel);
        pb
    } else {
        ir_path_rel.to_path_buf()
    };

    let ir_str = match File::open(&ir_path) {
        Err(e) => {
            return syn::Error::new(
                ir_lit.span(),
                format!(
                    "Unable to open file \"{}\": {}",
                    ir_path.to_string_lossy(),
                    e
                ),
            )
            .to_compile_error()
            .into()
        }
        Ok(mut fp) => {
            let mut buf = String::new();
            if let Err(e) = fp.read_to_string(&mut buf) {
                return syn::Error::new(
                    ir_lit.span(),
                    format!(
                        "Unable to read file \"{}\": {}",
                        ir_path.to_string_lossy(),
                        e
                    ),
                )
                .to_compile_error()
                .into();
            }
            buf
        }
    };
    grass_impl(ir_str.as_str(), ir_lit.span())
}

#[proc_macro]
pub fn import_grass_ir(input: TokenStream) -> TokenStream {
    let ir_lit = parse_macro_input!(input as LitStr);
    grass_impl(ir_lit.value().as_str(), ir_lit.span())
}
