extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    try_test(attr, item, false)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn tokio_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    try_test(attr, item, true)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn try_test(attr: TokenStream, input: ItemFn, is_tokio: bool) -> syn::Result<Tokens> {
    let inner_test = if attr.is_empty() {
        quote! { std::prelude::v1::test }
    } else {
        attr.into()
    };

    let ItemFn {
        attrs: _attrs,
        vis,
        sig,
        block,
    } = input;

    let init_tracing = quote! {
        crate::INIT.call_once(|| tracing_subscriber::fmt()
            .compact()
            .with_max_level(tracing::Level::TRACE)
            .without_time()
            .with_line_number(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NEW | tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .try_init()
            .expect("could not init env filter")
        )
    };

    let block = if is_tokio {
        quote! {
          RUNTIME.block_on(async {
            #block
          });
        }
    } else {
        quote! { #block }
    };

    let result = quote! {
      #[#inner_test]
      #vis #sig {
      mod init_test_tracing {
        pub fn init() {
          #init_tracing;
        }
      }
      init_test_tracing::init();
      #block
    }
    };

    Ok(result)
}
