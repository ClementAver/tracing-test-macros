extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, Span, TokenStream as Tokens};
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    if sig.asyncness.is_some() {
        let msg = "async functions cannot be used for tests";
        return syn::Error::new_spanned(sig.fn_token, msg)
            .into_compile_error()
            .into();
    }

    let import_qualifier = import_qualifier();

    quote! {
        #(#attrs)*
        #[::std::prelude::v1::test]
        #vis #sig {
            #import_qualifier::tracing::init_tracing();
            #block
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn tokio_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = input;

    if sig.asyncness.is_none() {
        let msg = "the `async` keyword is missing from the function declaration";
        return syn::Error::new_spanned(sig.fn_token, msg)
            .into_compile_error()
            .into();
    }

    sig.asyncness = None;

    let import_qualifier = import_qualifier();

    quote! {
        #(#attrs)*
        #[::std::prelude::v1::test]
        #vis #sig {
            #import_qualifier::tracing::init_tracing();
            #import_qualifier::tokio::get_runtime().block_on(async {
                #block
            });
        }
    }
    .into()
}

fn import_qualifier() -> Tokens {
    match crate_name("tracing-test-macros")
        .unwrap_or_else(|err| panic!("failed to get tracing-test-macros dependency\n{err}"))
    {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
    }
}
