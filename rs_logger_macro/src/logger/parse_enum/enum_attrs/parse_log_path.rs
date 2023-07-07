use super::parse_attr;
use rs_logger_errors::{attributes::Attrs, value_errors, LoggerResult};
use venial::Attribute;

pub fn parse(attribute: &Attribute) -> LoggerResult<String> {
    let lit_value = parse_attr::to_literal(attribute, Attrs::ModuleIdx)?;

    let value = match lit_value.string_value {
        Some(value) => value,
        None => {
            return Err(value_errors::invalid_type(
                attribute,
                Attrs::LogPath,
                lit_value.is_tuple_value,
            ))
        }
    };

    Ok(value)
}
