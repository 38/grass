use grass_ir::{ConstValue, FieldExpression};
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub fn expand_field_expr(expr: &FieldExpression, span: Span) -> TokenStream {
    let expr = expand_field_expr_impl(expr, span);
    quote! {
        |_arg| {
            #expr
        }
    }
}

fn expand_field_expr_impl(expr: &FieldExpression, span: Span) -> TokenStream {
    match expr {
        FieldExpression::And(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) & (#rhs)}
        }
        FieldExpression::Or(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) | (#rhs)}
        }
        FieldExpression::Xor(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) ^ (#rhs)}
        }
        FieldExpression::Not(param) => {
            let inner = expand_field_expr_impl(param.operand.as_ref(), span);
            quote! {!(#inner)}
        }
        FieldExpression::Add(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) + (#rhs)}
        }
        FieldExpression::Sub(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) - (#rhs)}
        }
        FieldExpression::Mul(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) * (#rhs)}
        }
        FieldExpression::Div(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) / (#rhs)}
        }
        FieldExpression::Mod(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) % (#rhs)}
        }
        FieldExpression::Eq(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) == (#rhs)}
        }
        FieldExpression::Ne(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) != (#rhs)}
        }
        FieldExpression::LessThan(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) < (#rhs)}
        }
        FieldExpression::GreaterThan(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) > (#rhs)}
        }
        FieldExpression::LessEqualThan(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) >= (#rhs)}
        }
        FieldExpression::GreaterEqualThan(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) <= (#rhs)}
        }
        FieldExpression::RightShift(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) << (#rhs)}
        }
        FieldExpression::LeftShift(param) => {
            let lhs = expand_field_expr_impl(param.lhs.as_ref(), span);
            let rhs = expand_field_expr_impl(param.rhs.as_ref(), span);
            quote! {(#lhs) >> (#rhs)}
        }
        FieldExpression::Neg(param) => {
            let operand = expand_field_expr_impl(param.operand.as_ref(), span);
            quote! { !(#operand) }
        }
        FieldExpression::Cond(param) => {
            let cond = expand_field_expr_impl(param.cond.as_ref(), span);
            let then = expand_field_expr_impl(param.then.as_ref(), span);
            let elze = expand_field_expr_impl(param.elze.as_ref(), span);
            quote! {
                if #cond {#then} else {#elze}
            }
        }
        FieldExpression::FieldRef(param) => {
            let p = syn::Ident::new(param.field.as_str(), span);
            if param.field != "start" && param.field != "end" {
                quote! {
                    ({
                        use grass_runtime::property::*;
                        _arg . #p ()
                    })
                }
            } else {
                quote! {
                    ({
                        use grass_runtime::property::*;
                        _arg . #p ()
                    } as f64)
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
            ConstValue::Float(value) => {
                let tk = syn::LitFloat::new(&format!("{}f64", value), span);
                quote! { #tk }
            }
            ConstValue::Number(value) => {
                let tk = syn::LitFloat::new(&format!("{}f64", value), span);
                quote! { #tk }
            }
            ConstValue::Str(value) => {
                let tk = syn::LitStr::new(value, span);
                quote! { #tk }
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
            let inner = expand_field_expr_impl(param.value.as_ref(), span);
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
