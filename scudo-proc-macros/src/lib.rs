// Copyright 2022 Google LLC
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

//! A proc-macro crate for the Rust bindings to the
//! [Scudo allocator](https://llvm.org/docs/ScudoHardenedAllocator.html#options).
//!
//! The exported [`macro@set_scudo_options`] attribute macro allows to set Scudo
//! options with an annotation on the main method:
//!
//! ```rust
//! use scudo_proc_macros::set_scudo_options;
//!
//! #[set_scudo_options(delete_size_mismatch = false, release_to_os_interval_ms = 1)]
//! fn main() {
//!     // Use Scudo with the provided options.
//! }
//! ```
//!
//! For more on Scudo options, visit the official documentation
//! [here](https://llvm.org/docs/ScudoHardenedAllocator.html#options).
//!
//! Please note: the proc macro exported by this crate works both with the
//! [scudo-sys](https://crates.io/crates/scudo-sys) crate as well as with the
//! idiomatic Rust binding crate, [scudo](https://crates.io/crates/scudo).

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

/// Sets options for the Scudo Allocator. This macro takes a list of
/// comma-seperated options, where each option is in the form
/// key = value. The value could either be a number like '3.14' or a
/// boolean value like 'true'. For a list of all available Scudo options,
/// please visit the [official documentations](https://llvm.org/docs/ScudoHardenedAllocator.html#options)
/// Pleaso note that this macro can only be used on the main method of a
/// Rust program.
///
/// # Example
///
/// ```rust
/// use scudo_proc_macros::set_scudo_options;
///
/// #[set_scudo_options(delete_size_mismatch = false, release_to_os_interval_ms = 1)]
/// fn main() {
///     // Use Scudo with the provided options.
/// }
/// ```
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_fail() {
        let test_cases = trybuild::TestCases::new();
        test_cases.compile_fail("tests/ui/*_fail.rs");
    }

    #[test]
    fn test_pass() {
        let test_cases = trybuild::TestCases::new();
        test_cases.pass("tests/ui/*_pass.rs");
    }
}
