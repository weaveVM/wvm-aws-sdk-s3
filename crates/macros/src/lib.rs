extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

// Bring in parse_quote for easy AST creation.
use syn::parse_quote;

// Define a custom parser for our attribute arguments.
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
    let mut input = parse_macro_input!(item as ItemStruct);

    // If require_bucket is specified, inject the `bucket_name` field.
    if args.require_bucket {
        match &mut input.fields {
            syn::Fields::Named(ref mut fields_named) => {
                // Check if a field called `bucket_name` already exists.
                let already_exists = fields_named
                    .named
                    .iter()
                    .any(|f| f.ident.as_ref().map_or(false, |id| id == "bucket_name"));
                if !already_exists {
                    // Create a new field: `pub bucket_name: String`
                    let new_field: syn::Field = parse_quote! {
                        pub bucket_name: String
                    };
                    fields_named.named.push(new_field);
                }
            }
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "#[weavevm(require_bucket)] only works on structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    // If require_bucket is set, generate the trait implementation.
    let impl_tokens = if args.require_bucket {
        let name = &input.ident;
        let generics = &input.generics;
        quote! {
            impl #generics RequireBucket for #name #generics {
                fn bucket(mut self, bucket_name: &str) -> Self {
                    self.bucket_name = bucket_name.to_string();
                    self
                }
            }
        }
    } else {
        quote! {}
    };

    // Combine the (possibly modified) struct and the trait implementation.
    let expanded = quote! {
        #input
        #impl_tokens
    };

    TokenStream::from(expanded)
}
