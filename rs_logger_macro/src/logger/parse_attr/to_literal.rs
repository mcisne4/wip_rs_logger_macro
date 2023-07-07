use litrs::Literal;
use rs_logger_errors::{attributes::Attrs, value_errors, LoggerResult};
use venial::Attribute;

#[derive(Debug)]
pub struct LiteralValue {
    pub string_value: Option<String>,
    pub number_value: Option<usize>,
    pub is_tuple_value: bool,
}

pub fn to_literal(attr: &Attribute, log_attr: Attrs) -> LoggerResult<LiteralValue> {
    let (token_tree, is_tuple_value) = match &attr.value {
        venial::AttributeValue::Empty => return Err(value_errors::is_empty(attr, log_attr, false)),
        venial::AttributeValue::Equals(_, tokentree_vec) => match &tokentree_vec.len() {
            0 => return Err(value_errors::is_empty(attr, log_attr, false)),
            1 => (&tokentree_vec[0], false),
            _ => return Err(value_errors::multiple_values(attr, log_attr, false)),
        },
        venial::AttributeValue::Group(_, tokentree_vec) => match &tokentree_vec.len() {
            0 => return Err(value_errors::is_empty(attr, log_attr, true)),
            1 => (&tokentree_vec[0], true),
            _ => return Err(value_errors::multiple_values(attr, log_attr, true)),
        },
    };

    match Literal::try_from(token_tree) {
        Err(_) => Err(value_errors::unparseable(attr, log_attr, is_tuple_value)),
        Ok(Literal::String(string_lit)) => {
            let value = string_lit.value();
            Ok(LiteralValue {
                string_value: Some(value.to_owned()),
                number_value: None,
                is_tuple_value,
            })
        }
        Ok(Literal::Integer(integer_lit)) => {
            let value_opt = integer_lit.value::<usize>();
            match value_opt {
                Some(int) => Ok(LiteralValue {
                    string_value: None,
                    number_value: Some(int),
                    is_tuple_value,
                }),
                None => Err(value_errors::unparseable(attr, log_attr, is_tuple_value)),
            }
        }
        Ok(_other) => Err(value_errors::invalid_type(attr, log_attr, is_tuple_value)),
    }
}
