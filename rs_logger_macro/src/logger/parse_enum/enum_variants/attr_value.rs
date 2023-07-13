use crate::logger::parse_attr;
use rs_logger_errors::{attributes::VariantAttrs, value_errors, LoggerResult};

pub fn parse_attr_value(
    attr: &venial::Attribute,
    variant_attr: &VariantAttrs,
) -> LoggerResult<String> {
    let lit_value = parse_attr::to_literal(attr, variant_attr.as_attrs())?;

    match lit_value.string_value {
        Some(value) => Ok(value),
        None => Err(value_errors::invalid_type(
            attr,
            variant_attr.as_attrs(),
            lit_value.is_tuple_value,
        )),
    }
}
