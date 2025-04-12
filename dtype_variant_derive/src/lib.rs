use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Token, parse_macro_input, punctuated::Punctuated};

mod derive;

pub(crate) fn dtype_variant_path() -> syn::Path {
    let found_crate = proc_macro_crate::crate_name("dtype_variant").expect("dtype_variant is present in `Cargo.toml`");
    match found_crate {
        proc_macro_crate::FoundCrate::Itself => format_ident!("crate").into(),
        proc_macro_crate::FoundCrate::Name(name) => syn::parse(name.parse().unwrap()).unwrap(),
    }
}

#[proc_macro_derive(DType, attributes(dtype))]
pub fn dtype_derive(input: TokenStream) -> TokenStream {
    derive::dtype_derive_impl(input)
}

struct DTypeInput {
    variants: Punctuated<Ident, Token![,]>,
}

impl syn::parse::Parse for DTypeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::bracketed!(content in input);
        Ok(DTypeInput {
            variants: content.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

#[proc_macro]
pub fn build_dtype_tokens(input: TokenStream) -> TokenStream {
    let DTypeInput { variants } = parse_macro_input!(input as DTypeInput);

    let expanded = variants.iter().map(|variant| {
        let variant_name = format_ident!("{}Variant", variant);

        quote! {
            pub struct #variant_name;
        }
    });

    quote! {
        #(#expanded)*
    }
    .into()
}
