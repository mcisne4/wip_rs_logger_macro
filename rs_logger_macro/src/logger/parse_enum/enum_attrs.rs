mod attr_name;
mod eq_crate_idx;
mod eq_module_idx;
mod eq_module_path;

use proc_macro2::TokenStream;
use rs_logger_errors::LoggerResult;
use rs_logger_errors::{
    attributes::{EnumAttrs, VariantAttrs},
    declaration_errors,
};

pub fn parse_enum_attrs(
    attributes: &Vec<venial::Attribute>,
    decl_tokens: TokenStream,
) -> LoggerResult<()> {
    if attributes.len() == 0 {
        return Err(declaration_errors::no_attrs(decl_tokens));
    }

    for attr in attributes {
        let attr_name = attr_name::get_attr_name(&attr)?;

        match &attr.value {
            venial::AttributeValue::Empty => (),
            venial::AttributeValue::Equals(punct, _tokens) => (),
            venial::AttributeValue::Group(group_span, _tokens) => (),
        }

        eprintln!("{:#?}", attr.value);
    }

    Ok(())
}
