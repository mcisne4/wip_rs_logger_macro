use super::InnerValue;
use rs_logger_errors::{variant_msg_errors, LoggerResult};

enum BracketValueType {
    Continue,
    Unnamed,
    NamedInt,
    NamedVar,
    Unresolved,
}

pub struct LeftBracket<'a> {
    value: String,
    attr: &'a venial::Attribute,
    found_end_whitespace: bool,
    complete: bool,
    value_type: BracketValueType,
}
impl<'a> LeftBracket<'a> {
    pub fn new(attr: &'a venial::Attribute) -> Self {
        Self {
            value: String::new(),
            attr,
            found_end_whitespace: false,
            complete: false,
            value_type: BracketValueType::Unresolved,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn to_inner_value(self) -> LoggerResult<Option<InnerValue>> {
        match self.complete {
            false => Ok(None),
            true => match self.value_type {
                BracketValueType::Unnamed => Ok(Some(InnerValue::Unnamed)),
                BracketValueType::NamedInt => Ok(Some(InnerValue::NamedInt(self.value))),
                BracketValueType::NamedVar => Ok(Some(InnerValue::NamedVar(self.value))),
                BracketValueType::Unresolved => {
                    Err(variant_msg_errors::invalid_argument(self.attr))
                }
                _ => Ok(None),
            },
        }
    }

    fn close(&mut self, update_value_type: Option<BracketValueType>) {
        self.complete = true;
        if let Some(value_type) = update_value_type {
            self.value_type = value_type;
        }
    }

    pub fn evaluate_char(&mut self, chr: char) -> LoggerResult<()> {
        match chr {
            '{' => match self.value.len() {
                0 => {
                    self.close(Some(BracketValueType::Continue));
                    Ok(())
                }
                _ => Err(variant_msg_errors::invalid_argument(self.attr)),
            },
            '}' => match self.value.len() {
                0 => {
                    self.close(Some(BracketValueType::Unnamed));
                    Ok(())
                }
                _ => {
                    self.close(None);
                    Ok(())
                }
            },
            _ => {
                if chr.is_whitespace() {
                    match self.value.len() {
                        0 => return Ok(()),
                        _ => {
                            self.found_end_whitespace = true;
                            return Ok(());
                        }
                    }
                }

                if chr.is_numeric() {
                    match self.value.len() {
                        0 => {
                            self.value_type = BracketValueType::NamedInt;
                            self.value += chr.to_string().as_str();
                            return Ok(());
                        }
                        _ => match self.value_type {
                            BracketValueType::NamedInt => {
                                self.value += chr.to_string().as_str();
                                return Ok(());
                            }
                            _ => return Err(variant_msg_errors::invalid_argument(self.attr)),
                        },
                    }
                }

                if chr.is_alphabetic() || chr == '_' {
                    match self.value.len() {
                        0 => {
                            self.value_type = BracketValueType::NamedVar;
                            self.value += chr.to_string().as_str();
                            return Ok(());
                        }
                        _ => match self.value_type {
                            BracketValueType::NamedVar => {
                                self.value += chr.to_string().as_str();
                                return Ok(());
                            }
                            _ => return Err(variant_msg_errors::invalid_argument(self.attr)),
                        },
                    }
                }

                Err(variant_msg_errors::invalid_argument(self.attr))
            }
        }
    }

    pub fn end(&mut self) {
        self.complete = true;
        self.value_type = BracketValueType::Unresolved;
    }
}
