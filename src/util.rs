use proc_macro2::Span;
use quote::quote_spanned;
use syn::{self, Path};

/// The path to the `Default` trait to use in generated code. This should point to
/// an export from the `serde_default` crate to work in both std and no_std cases.
pub fn trait_path(span: Span) -> Path {
    syn::parse2(quote_spanned!(span=> ::std::default::Default)).unwrap()
}
