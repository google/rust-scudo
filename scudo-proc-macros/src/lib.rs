// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quote::quote;
use syn::{
    parse, parse::Parse, parse_macro_input, punctuated::Punctuated, Error, Ident, ItemFn, LitStr,
    Token,
};

/// An `Option` holds a key and a value. The value could either be an
/// `Ident` (like "true") or a `Literal` (like "3.14").
struct Option {
    key: Ident,
    value: Box<dyn ToString>,
}

/// Holds a comma seperated list of `Option`s.
struct OptionsList {
    content: Punctuated<Option, Token![,]>,
}

impl Parse for Option {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        _ = input.parse::<Token![=]>()?;

        let error = Error::new(input.span(), "Expect a valid value like '3.14' or 'true'");

        // Parse the value as `ToString`.
        let value = match input.parse() {
            Ok(TokenTree::Punct(punct)) => {
                if punct.as_char() != '-' {
                    return Err(error);
                }

                // Parsing a negative number, the next element needs to be a literal in this case.
                if let TokenTree::Literal(lit) = input.parse()? {
                    Ok(Box::new(format!("-{}", lit)) as Box<dyn ToString>)
                } else {
                    Err(error)
                }
            }
            Ok(TokenTree::Literal(lit)) => Ok(Box::new(lit) as Box<dyn ToString>),
            Ok(TokenTree::Ident(ident)) => {
                if ident != "true" && ident != "false" {
                    Err(error)
                } else {
                    Ok(Box::new(ident) as Box<dyn ToString>)
                }
            }
            _ => Err(error),
        }?;

        Ok(Option { key, value })
    }
}

impl Parse for OptionsList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(OptionsList {
            content: input.parse_terminated(Option::parse)?,
        })
    }
}

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

#[proc_macro_attribute]
pub fn set_scudo_options(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Check that this macro is only used on the main method.
    let input: ItemFn = match parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };

    if input.sig.ident != "main" {
        let msg = "This macro is only allowed to be used on the main method";
        return token_stream_with_error(item, syn::Error::new_spanned(&input.sig.ident, msg));
    };

    // Parse the options.
    let options = parse_macro_input!(attr as OptionsList).content;

    // Build the options string.
    let mut option_str = String::new();
    for option in options.iter() {
        option_str += &format!("{}={}:", option.key, option.value.to_string());
    }
    // This interfaces with C, so null-terminate the String.
    option_str += "\0";
    let option_lit = LitStr::new(&option_str, Span::call_site());

    // The resulting code defines the `__scudo_default_options` method that returns
    // a pointer to the created string and leaves the main method untouched.
    let options_method = TokenStream::from(quote! {
        use libc::c_char;

        #[no_mangle]
        const extern "C" fn __scudo_default_options() -> *const c_char {
            #option_lit.as_ptr() as *const c_char
        }

    });
    let mut result = item;
    result.extend(options_method);

    result
}
