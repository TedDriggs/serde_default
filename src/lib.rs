//! Generate a `Default` impl based on field-level defaults in `serde` attributes.
//!
//! # Usage
//! On a struct that derives `Serialize` or `Deserialize`, add `DefaultFromSerde`.
//!
//! ```rust
//! # use serde_default::DefaultFromSerde;
//! #[derive(Debug, DefaultFromSerde, PartialEq, Eq)]
//! pub struct MyStruct {
//!     #[serde(default = "field_1_default")]
//!     field1: u16,
//!     #[serde(default)]
//!     field2: String,
//! }
//!
//! fn field_1_default() -> u16 {
//!     3
//! }
//!
//! fn main() {
//!     assert_eq!(MyStruct::default(), MyStruct { field1: 3, field2: "".into() });
//! }
//! ```

extern crate proc_macro;

mod codegen;
mod options;
mod util;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

use options::Options;

/// Generate a `Default` impl based on defaults in `serde` attributes.
///
/// Only structs are supported - attempting to use this on enums will result in an error.
///
/// # Example
/// ```rust
/// # use serde_default::DefaultFromSerde;
/// #[derive(Debug, DefaultFromSerde, PartialEq, Eq)]
/// pub struct MyStruct {
///     #[serde(default = "field_1_default")]
///     field1: u16,
///     #[serde(default)]
///     field2: String,
/// }
///
/// fn field_1_default() -> u16 {
///     3
/// }
///
/// fn main() {
///     assert_eq!(MyStruct::default(), MyStruct { field1: 3, field2: "".into() });
/// }
/// ```
#[proc_macro_derive(DefaultFromSerde, attributes(serde))]
pub fn derive_default(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let opts = match Options::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let code = codegen::TraitImpl::from(&opts);
    quote!(#code).into()
}
