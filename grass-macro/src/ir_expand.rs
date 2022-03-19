use std::collections::HashMap;

use grass_ir::GrassIR;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use uuid::adapter::Simple;

use self::format::expand_write_record_rec; 

mod open;
mod write;
mod let_binding;
mod intersect;
mod filter;
mod field_expr;
mod format; 
mod alter;
mod assume_sorted;
    
pub fn expand_grass_ir(ir: &GrassIR, ctx: &mut ExpansionContext) -> ExpandResult{
    match ir {
        GrassIR::Open(open_param) => open_param.expand(ctx),
        GrassIR::WriteFile(write_param) => {
            match write_param.what.as_ref() {
                GrassIR::Let(param) => {
                    match param.value.as_ref() {
                        GrassIR::Format(param) => {
                            expand_write_record_rec(param, &write_param.target, ctx)
                        },
                        _ => write_param.expand(ctx),
                    }
                },
                GrassIR::Format(param) => {
                    expand_write_record_rec(param, &write_param.target, ctx)
                }
                _ => write_param.expand(ctx)
            }
        }
        GrassIR::Let(param) => param.expand(ctx),
        GrassIR::Intersection(param) => param.expand(ctx),
        GrassIR::Filter(param) => param.expand(ctx),
        GrassIR::Alter(param) => param.expand(ctx),
        GrassIR::AssumeSorted(param) => param.expand(ctx),
        _ => panic!("Unimplemented IR {}", ir.as_ref()),
    }
}

pub type TempVar = Simple;

#[allow(dead_code)]
pub struct ExpansionContext {
    span: Span,
    code_fragments: Vec<TokenStream>,
    symbol_table: HashMap<String, TempVar>,
}

impl ExpansionContext{
    pub fn new(span: Span) -> Self {
        Self {
            span,
            code_fragments: Vec::new(),
            symbol_table: HashMap::new(),
        }
    }
    pub fn push(&mut self, expr: TokenStream) -> TempVar {
        let uuid = uuid::Uuid::new_v4().to_simple();
        let fresh_id = self.get_var_ref(&uuid);
        let code = expr;
        let code = quote! {
            let #fresh_id = #code;
        };
        self.code_fragments.push(code);
        uuid
    }
    pub fn get_var_ref(&self, id: &TempVar) -> syn::Ident {
        syn::Ident::new(&format!("_grass_query_temp_{}", id), self.span)
    }
    pub fn to_token_stream(&self) -> TokenStream {
        let fragments = self.code_fragments.as_slice();
        quote! {
            #(#fragments)*
        }
    }
    pub fn span(&self) -> Span {
        self.span
    }
}

type ExpandResult = Result<TempVar, syn::Error>;

pub trait Expand {
    fn expand(&self, ctx: &mut ExpansionContext) -> ExpandResult;
}

