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
    parse_macro_input, ExprPath, ItemFn, Path, Token,
};

struct Args {
    harness: Option<Path>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { harness: None });
        }

        let harness_ident: ExprPath = input.parse()?;
        if !harness_ident.path.is_ident("harness") {
            Err(input.error(
                "we only recognize #[test(harness = some::path)], #[test(harness)], and #[test]",
            ))
        } else if input.peek(Token![=]) {
            let syn::token::Eq { .. } = input.parse()?;
            let ExprPath { path, .. } = input.parse()?;
            Ok(Self {
                harness: Some(path),
            })
        } else {
            Ok(Self {
                harness: Some(harness_ident.path),
            })
        }
    }
}

/// currently supports #[test_harness::test(harness = path::to::harness_fn)] and #[test]
/// see crate-level docs for usage and examples
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    let Args { harness } = parse_macro_input!(args as Args);
    match harness {
        Some(harness) => with_harness(harness, input),
        None => without_harness(input),
    }
}

fn without_harness(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    quote! {
        #[::core::prelude::v1::test]
        #input
    }
    .into()
}

fn with_harness(harness: Path, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    #[allow(clippy::redundant_clone)] // clippy bug
    let fn_name = input.sig.ident.clone();
    quote! {
        #[::core::prelude::v1::test]
        fn #fn_name() -> impl ::std::process::Termination {
            #input
            #harness(#fn_name)
        }
    }
    .into()
}
