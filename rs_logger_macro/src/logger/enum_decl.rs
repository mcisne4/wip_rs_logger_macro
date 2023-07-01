use rs_logger_errors::declaration_errors::invalid_declaration_type;
use rs_logger_errors::LoggerResult;

pub fn validate_enum_decl<'a>(decl: &'a venial::Declaration) -> LoggerResult<&'a venial::Enum> {
    match decl.as_enum() {
        Some(yes) => Ok(yes),
        None => Err(invalid_declaration_type(decl)),
    }
}
