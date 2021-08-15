use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{spanned::Spanned, Error, ImplItemMethod, Result};

pub(super) struct Funcs {
    pub pre_init: Option<TokenStream2>,
    pub post_init: Option<TokenStream2>,
    pub manual_view: Option<TokenStream2>,
}

impl Funcs {
    pub fn new(funcs: &[ImplItemMethod]) -> Result<Self> {
        let mut pre_init = None;
        let mut post_init = None;
        let mut manual_view = None;

        for func in funcs {
            let ident = &func.sig.ident;
            let stmts = &func.block.stmts;
            let tokens = quote! { #(#stmts)* };

            if ident == "pre_init" {
                if pre_init.is_some() {
                    return Err(Error::new(
                        func.span().unwrap().into(),
                        "widget method defined multiple times",
                    ));
                }
                pre_init = Some(tokens);
            } else if ident == "post_init" {
                if post_init.is_some() {
                    return Err(Error::new(
                        func.span().unwrap().into(),
                        "post_init method defined multiple times",
                    ));
                }
                post_init = Some(tokens);
            } else if ident == "manual_view" {
                if manual_view.is_some() {
                    return Err(Error::new(
                        func.span().unwrap().into(),
                        "manual_view method defined multiple times",
                    ));
                }
                manual_view = Some(tokens);
            } else {
                return Err(Error::new(
                    func.span().unwrap().into(),
                    "Expected identifier pre_init, post_init or manual_view",
                ));
            }
        }

        Ok(Funcs {
            pre_init,
            post_init,
            manual_view,
        })
    }
}