use super::parse_attr;
use rs_logger_errors::{attributes::Attrs, value_errors, LoggerResult};
use venial::Attribute;

pub fn parse(attribute: &Attribute) -> LoggerResult<String> {
    let lit_value = parse_attr::to_literal(attribute, Attrs::ModuleIdx)?;

    let int = match lit_value.number_value {
        Some(int) => int,
        None => {
            return Err(value_errors::invalid_type(
                attribute,
                Attrs::ModuleIdx,
                lit_value.is_tuple_value,
            ))
        }
    };

    if int > 255 {
        return Err(value_errors::invalid_number_u8(
            attribute,
            lit_value.is_tuple_value,
        ));
    }

    let hex_value = format!("{:02X}", int);

    Ok(hex_value)
}
