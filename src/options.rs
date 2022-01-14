use std::borrow::Cow;

use darling::{
    ast::Data,
    util::{Ignored, SpannedValue},
    Error, FromDeriveInput, FromField, FromMeta, Result,
};
use syn::{Attribute, Generics, Ident, Path};

use crate::{codegen, util};

/// Collector for struct-level information about the type deriving `SerdeDefault`.
#[derive(FromDeriveInput)]
#[darling(
    attributes(serde),
    supports(struct_any),
    forward_attrs(allow, cfg),
    allow_unknown_fields
)]
pub struct Options {
    /// The name of the deriving type.
    ident: Ident,
    /// The attributes from the deriving type to apply to the emitted trait impl.
    /// These will be inserted into our struct by `darling`.
    attrs: Vec<Attribute>,
    /// Information about the body of the struct. Since we told `darling` to produce
    /// errors if we're given an enum, we ignore enum variants and only provide a type
    /// for fields.
    data: Data<Ignored, SpannedValue<FieldOptions>>,
    /// The generics of the input type, as passed to us by `darling`.
    generics: Generics,
}

impl<'a> From<&'a Options> for codegen::TraitImpl<'a> {
    fn from(options: &'a Options) -> Self {
        codegen::TraitImpl {
            ident: &options.ident,
            attrs: &options.attrs,
            generics: Cow::Borrowed(&options.generics),
            fields: options
                .data
                .as_ref()
                .map_struct_fields(codegen::Field::from)
                .take_struct()
                .expect("Input body can't be an enum"),
        }
    }
}

#[derive(FromField)]
#[darling(attributes(serde), allow_unknown_fields)]
pub struct FieldOptions {
    ident: Option<Ident>,
    default: Option<DefaultDeclaration>,
}

impl<'a> From<&'a SpannedValue<FieldOptions>> for codegen::Field<'a> {
    fn from(options: &'a SpannedValue<FieldOptions>) -> Self {
        codegen::Field {
            ident: options.ident.as_ref(),
            span: options.span(),
            path: match options.default.as_ref() {
                Some(DefaultDeclaration::Path(path)) => Cow::Borrowed(path),
                Some(DefaultDeclaration::Trait) | None => {
                    let mut trait_path = util::trait_path(options.span());
                    trait_path
                        .segments
                        .push(Ident::new("default", options.span()).into());
                    Cow::Owned(trait_path)
                }
            },
        }
    }
}

/// A declaration of the `default` attribute in `serde`.
enum DefaultDeclaration {
    /// Just the word `default`, which means "use the `Default` trait"
    Trait,
    /// A key-value pair, in which case the value is the path of the zero-arg function
    Path(Path),
}

impl FromMeta for DefaultDeclaration {
    fn from_word() -> Result<Self> {
        Ok(DefaultDeclaration::Trait)
    }

    fn from_string(value: &str) -> Result<Self> {
        syn::parse_str(value)
            .map(DefaultDeclaration::Path)
            .map_err(|_| Error::unknown_value(value))
    }
}
