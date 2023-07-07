use super::parse_attr;
use rs_logger_errors::{attributes::Attrs, value_errors, LoggerResult};
use venial::Attribute;

pub fn parse_attr(attr: &Attribute) -> LoggerResult<String> {
    let lit_value = parse_attr::to_literal(attr, Attrs::CrateIdx)?;

    let int = match lit_value.number_value {
        Some(int) => int,
        None => {
            return Err(value_errors::invalid_type(
                attr,
                Attrs::CrateIdx,
                lit_value.is_tuple_value,
            ))
        }
    };

    if int > 15 {
        return Err(value_errors::invalid_number_u4(
            attr,
            lit_value.is_tuple_value,
        ));
    }

    let hex_value = format!("{:01X}", int);

    Ok(hex_value)
}
