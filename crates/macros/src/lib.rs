extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

// Define a custom parser for the attribute arguments.
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token};

/// Our expected attribute arguments, e.g., `require_bucket`
struct WeavevmArgs {
    require_bucket: bool,
}

impl Parse for WeavevmArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut require_bucket = false;
        // Parse a comma-separated list of identifiers.
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            if ident == "require_bucket" {
                require_bucket = true;
            }
            if input.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
            }
        }
        Ok(WeavevmArgs { require_bucket })
    }
}

#[proc_macro_attribute]
pub fn weavevm(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute arguments using our custom parser.
    let args = parse_macro_input!(attr as WeavevmArgs);
    // Parse the input as a struct.
    let input = parse_macro_input!(item as ItemStruct);

    let require_bucket = args.require_bucket;

    // Generate tokens for the original struct.
    let struct_tokens = quote! { #input };

    // Conditionally generate the impl for the `RequireBucket` trait.
    let impl_tokens = if require_bucket {
        let name = &input.ident;
        let generics = &input.generics;
        quote! {
            impl #generics RequireBucket for #name #generics {
                fn bucket(mut self, bucket_name: &str) -> Self {
                    // Assumes that the struct has a field named `bucket_name`.
                    self.bucket_name = bucket_name.to_string();
                    self
                }
            }
        }
    } else {
        quote! {}
    };

    // Combine the original struct and the conditional implementation.
    let expanded = quote! {
        #struct_tokens
        #impl_tokens
    };

    TokenStream::from(expanded)
}
