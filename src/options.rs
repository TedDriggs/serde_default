use std::borrow::Cow;

use darling::ast::Data;
use darling::util::{Ignored, SpannedValue};
use darling::{Error, FromMeta, Result};
use syn::{Attribute, Ident, Path};

use codegen;

#[derive(FromDeriveInput)]
#[darling(
    attributes(serde),
    supports(struct_any),
    forward_attrs(allow, cfg),
    allow_unknown_fields
)]
pub struct Options {
    ident: Ident,
    attrs: Vec<Attribute>,
    data: Data<Ignored, SpannedValue<FieldOptions>>,
}

impl<'a> From<&'a Options> for codegen::TraitImpl<'a> {
    fn from(options: &'a Options) -> Self {
        codegen::TraitImpl {
            ident: &options.ident,
            attrs: &options.attrs,
            fields: options
                .data
                .as_ref()
                .map_struct_fields(codegen::Field::from)
                .take_struct()
                .unwrap(),
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
                    Cow::Owned(parse_quote!(::std::default::Default::default))
                }
            },
        }
    }
}

enum DefaultDeclaration {
    Trait,
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
