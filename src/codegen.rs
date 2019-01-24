use std::borrow::Cow;

use darling::ast::Fields;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Attribute, Ident, Path};

/// Receiver for trait impl; this gets the struct ident and the fields
/// in the struct.
pub struct TraitImpl<'a> {
    pub ident: &'a Ident,
    pub attrs: &'a [Attribute],
    pub fields: Fields<Field<'a>>,
}

impl<'a> TraitImpl<'a> {
    fn body(&self) -> TokenStream {
        if self.fields.style.is_struct() {
            let fields = self.fields.as_ref();
            quote! { Self { #(#fields),* }}
        } else {
            let ident = &self.ident;
            let fields = self.fields.as_ref();
            quote! { #ident(#(#fields),*) }
        }
    }
}

impl<'a> ToTokens for TraitImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let body = self.body();
        let ident = &self.ident;
        let attrs = self.attrs.iter();
        tokens.extend(quote! {
            #(#attrs)*
            impl ::std::default::Default for #ident {
                fn default() -> Self {
                    #body
                }
            }
        });
    }
}

pub struct Field<'a> {
    pub span: &'a Span,
    pub ident: Option<&'a Ident>,
    pub path: Cow<'a, Path>,
}

impl<'a> ToTokens for Field<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Field {
            ref span,
            ref ident,
            ref path,
        } = *self;

        let span = *span.clone();
        if let Some(ident) = ident {
            tokens.extend(quote_spanned!(span=> #ident: #path()));
        } else {
            tokens.extend(quote_spanned!(span=> #path()));
        }
    }
}
