mod types;

use rs_logger_errors::{variant_msg_errors, LoggerResult};
use types::{InnerValue, LeftBracket};

pub fn validate_string_value(value: String, attr: &venial::Attribute) -> LoggerResult<()> {
    eprintln!("Original: '{}'", value);

    let mut params: Vec<InnerValue> = vec![];

    let mut skip_chars = 0_usize;

    for (idx, ch0) in value.chars().enumerate() {
        if skip_chars > 0 {
            skip_chars -= 1;
            continue;
        }

        match ch0 {
            '{' => {
                let mut left_bracket = LeftBracket::new(attr);

                while !left_bracket.is_complete() {
                    match value.chars().nth(idx + skip_chars + 1) {
                        None => left_bracket.end(),
                        Some(ch_next) => {
                            skip_chars += 1;

                            left_bracket.evaluate_char(ch_next)?;
                        }
                    }
                }

                match left_bracket.to_inner_value()? {
                    Some(inner_value) => params.push(inner_value),
                    None => (),
                }
            }
            '}' => {
                if let Some(ch_next) = value.chars().nth(idx + skip_chars + 1) {
                    if ch_next != '}' {
                        return Err(variant_msg_errors::unmatched_right_bracket(attr));
                    }
                }
            }

            _ => continue,
        }
    }

    for param in params {
        match param {
            InnerValue::NamedInt(val) => eprintln!("\tNamed Int: {}", val),
            InnerValue::NamedVar(val) => eprintln!("\tNamed Var: {}", val),
            InnerValue::Unnamed => eprintln!("\tUnnamed Value"),
        }
    }

    Ok(())
}
