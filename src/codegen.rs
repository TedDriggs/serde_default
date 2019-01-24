use std::borrow::Cow;

use darling::ast::Fields;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{self, Attribute, Generics, Ident, Path};

use util;

/// Receiver for trait impl; this gets the struct ident and the fields
/// in the struct.
pub struct TraitImpl<'a> {
    pub ident: &'a Ident,
    pub attrs: &'a [Attribute],
    /// The generics on the input struct. In keeping with Rust's own implementation
    /// of `derive(Debug)` et al, we'll require that each of these implements `Default`
    /// for our emitted impl to apply.
    pub generics: Cow<'a, Generics>,
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

        let generics = if has_type_params(&self.generics) {
            Cow::Owned(compute_impl_bounds(
                util::trait_path(Span::call_site()),
                self.generics.clone().into_owned(),
            ))
        } else {
            self.generics.clone()
        };

        let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();
        let trait_path = util::trait_path(Span::call_site());

        // We add the "automatically_derived" attribute so that we don't make warnings
        // for unused code; see the compiler code below:
        // https://github.com/rust-lang/rust/blob/1.27.2/src/librustc/middle/liveness.rs#L366-L374
        tokens.append_all(&[quote! {
            #[automatically_derived]
            #(#attrs)*
            impl #impl_gen #trait_path for #ident #ty_gen #where_clause {
                fn default() -> Self {
                    #body
                }
            }
        }]);
    }
}

/// Checks if a set of generics has type parameters
fn has_type_params(generics: &Generics) -> bool {
    generics.type_params().next().is_some()
}

fn compute_impl_bounds(bound: Path, mut generics: Generics) -> Generics {
    if !has_type_params(&generics) {
        return generics;
    }

    let added_bound = syn::TypeParamBound::Trait(syn::TraitBound {
        paren_token: None,
        modifier: syn::TraitBoundModifier::None,
        lifetimes: None,
        path: bound,
    });

    for mut typ in generics.type_params_mut() {
        typ.bounds.push(added_bound.clone());
    }

    generics
}

/// State required to generate initializer code for a struct field inside the body of
/// `Default::default`
pub struct Field<'a> {
    /// The position of this field in the input source; this is used to place errors in
    /// case the generated code has incorrect types.
    pub span: Span,
    /// The field identifier. This can be `None` for newtype and tuple struct fields.
    pub ident: Option<&'a Ident>,
    /// The path of the zero-argument function to invoke for the field's default value
    pub path: Cow<'a, Path>,
}

impl<'a> ToTokens for Field<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Field {
            span,
            ref ident,
            ref path,
        } = *self;

        if let Some(ident) = ident {
            tokens.extend(quote_spanned!(span=> #ident: #path()));
        } else {
            tokens.extend(quote_spanned!(span=> #path()));
        }
    }
}
