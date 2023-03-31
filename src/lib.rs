#![forbid(unsafe_code)]
#![deny(
    clippy::dbg_macro,
    missing_copy_implementations,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    nonstandard_style,
    unused_qualifications
)]
#![warn(missing_docs, clippy::nursery, clippy::cargo)]
#![allow(clippy::must_use_candidate, clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, ExprAssign, ExprPath, ItemFn,
};

struct Args {
    harness: syn::Path,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ExprAssign { left, right, .. } = ExprAssign::parse(input)?;
        match (*left, *right) {
            (
                Expr::Path(ExprPath { path: left, .. }),
                Expr::Path(ExprPath { path: harness, .. }),
            ) if left.is_ident("harness") => Ok(Self { harness }),
            _ => Err(input.error("we only recognize test(harness = some::path)")),
        }
    }
}

/// currently only supports #[test_harness::test(harness = path::to::harness_fn)]
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    let Args { harness } = parse_macro_input!(args as Args);
    let input = parse_macro_input!(input as ItemFn);
    let fn_name = input.sig.ident.clone();
    let ret = quote! {
        #[::core::prelude::v1::test]
        fn #fn_name() {
            #input
            #harness(#fn_name);
        }
    };
    //    println!("{}", ret);
    ret.into()
}
