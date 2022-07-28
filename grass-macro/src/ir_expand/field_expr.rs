use grass_ir::{ConstOrEnv, ConstValue, FieldExpression};
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub fn expand_field_expr(expr: &FieldExpression, span: Span) -> TokenStream {
    let mut env_const_defs = Vec::new();
    let expr = expand_field_expr_impl(expr, span, &mut env_const_defs);
    quote! {
        {
            #(#env_const_defs)*
            move |_arg| {
                #expr
            }
        }
    }
}

fn expand_field_expr_impl(expr: &FieldExpression, span: Span, env_const_defs: &mut Vec<TokenStream>) -> TokenStream {
    match expr {
        FieldExpression::And(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) & (#rhs)}
        }
        FieldExpression::Or(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) | (#rhs)}
        }
        FieldExpression::Xor(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) ^ (#rhs)}
        }
        FieldExpression::Not(param) => {
            let inner = expand_field_expr_impl(param.operand.as_ref(), span, env_const_defs);
            quote! {!(#inner)}
        }
        FieldExpression::Add(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) + (#rhs)}
        }
        FieldExpression::Sub(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) - (#rhs)}
        }
        FieldExpression::Mul(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) * (#rhs)}
        }
        FieldExpression::Div(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) / (#rhs)}
        }
        FieldExpression::Mod(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) % (#rhs)}
        }
        FieldExpression::Eq(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) == (#rhs)}
        }
        FieldExpression::Ne(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) != (#rhs)}
        }
        FieldExpression::LessThan(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) < (#rhs)}
        }
        FieldExpression::GreaterThan(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) > (#rhs)}
        }
        FieldExpression::LessEqualThan(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) >= (#rhs)}
        }
        FieldExpression::GreaterEqualThan(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) <= (#rhs)}
        }
        FieldExpression::RightShift(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) << (#rhs)}
        }
        FieldExpression::LeftShift(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            quote! {(#lhs) >> (#rhs)}
        }
        FieldExpression::Neg(param) => {
            let operand = expand_field_expr_impl(param.operand.as_ref(), span, env_const_defs);
            quote! { !(#operand) }
        }
        FieldExpression::Cond(param) => {
            let cond = expand_field_expr_impl(param.cond.as_ref(), span, env_const_defs);
            let then = expand_field_expr_impl(param.then.as_ref(), span, env_const_defs);
            let elze = expand_field_expr_impl(param.elze.as_ref(), span, env_const_defs);
            quote! { if #cond {#then} else {#elze} }
        }
        FieldExpression::RegexMatch(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span, env_const_defs);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span, env_const_defs);
            match param.rhs.as_ref() {
                FieldExpression::ConstValue(_) => {
                    let regex_id = syn::Ident::new(&format!("__local_regex_{}", env_const_defs.len()), span);
                    env_const_defs.push(quote!{ 
                        let #regex_id = grass_runtime::Regex::new(#rhs).unwrap(); 
                    });
                    quote!{ #regex_id . is_match(#lhs) }
                }
                _ => {
                    // FIXME: Once we are in this case, we just do a simple contain
                    // But this is not correct
                    quote! { (#lhs).contains(#rhs) }
                }
            }
        }
        FieldExpression::FieldRef(param) => {
            let p = syn::Ident::new(param.field.as_str(), span);
            match param.field.as_str() {
                "start" | "end" => quote! {
                    ({
                        use grass_runtime::property::*;
                        _arg . #p ()
                    } as f64)
                },
                "score" => quote! {
                    ({
                        use grass_runtime::property::*;
                        _arg . #p () .unwrap_or_default()
                    })
                },
                _ => quote! {
                    ({
                        use grass_runtime::property::*;
                        _arg . #p ()
                    })
                }
            }
        }
        FieldExpression::NumberOfComponents => {
            quote! {
                ({
                    use grass_runtime::property::*;
                    _arg.size()
                })
            }
        }
        FieldExpression::ComponentFieldRef(param) => {
            let field_name = syn::Ident::new(param.field_name.as_str(), span);
            let comp_idx = syn::LitInt::new(&format!("{}", param.target), span);
            if param.field_name != "start" && param.field_name != "end" {
                quote! {
                    ({
                        use grass_runtime::property::*;
                        _arg. #comp_idx . #field_name ()
                    })
                }
            } else {
                quote! {
                    ({
                        use grass_runtime::property::*;
                        _arg. #comp_idx . #field_name ()
                    } as f64)
                }
            }
        }
        FieldExpression::ConstValue(param) => match &param.value {
            ConstOrEnv::Const(ConstValue::Float(value)) => {
                let tk = syn::LitFloat::new(&format!("{}f64", value), span);
                quote! { #tk }
            }
            ConstOrEnv::Const(ConstValue::Number(value)) => {
                let tk = syn::LitFloat::new(&format!("{}f64", value), span);
                quote! { #tk }
            }
            ConstOrEnv::Const(ConstValue::Str(value)) => {
                let tk = syn::LitStr::new(value, span);
                quote! { #tk }
            }
            ConstOrEnv::Env(key) => {
                let global_tk = syn::Ident::new(&key.get_const_bag_ident(), span);
                let local_tk = syn::Ident::new(&format!("__env_const_ref_{}", env_const_defs.len()), span);
                env_const_defs.push(quote! { let #local_tk = #global_tk.value(); });
                quote! { #local_tk }
            }
        },
        FieldExpression::FullRecordRef => {
            quote! {
                &_arg
            }
        }
        FieldExpression::RecordRef(param) => {
            let id = syn::LitInt::new(&format!("{}", param.id), span);
            quote! {
                ({
                    use grass_runtime::property::*;
                    &_arg . #id
                })
            }
        }
        FieldExpression::StringRepr(param) => {
            let inner = expand_field_expr_impl(param.value.as_ref(), span, env_const_defs);
            quote! {
                {
                    use grass_runtime::property::*;
                    let mut buffer = Vec::new();
                    #inner . dump(&mut buffer).unwrap();
                    String::from_utf8(buffer).unwrap()
                }
            }
        }
    }
    .into()
}
