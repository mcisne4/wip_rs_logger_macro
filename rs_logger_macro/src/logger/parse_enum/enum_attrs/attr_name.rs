use rs_logger_errors::attributes::VariantAttrs;
use rs_logger_errors::declaration_errors;
use rs_logger_errors::LoggerResult;

pub fn get_attr_name(attr: &venial::Attribute) -> LoggerResult<String> {
    let attr_name = attr.path[0].to_string();

    match attr_name.as_str() {
        "info_msg" => {
            return Err(declaration_errors::has_variant_attr(
                attr,
                VariantAttrs::InfoMsg,
            ))
        }
        "warn_msg" => {
            return Err(declaration_errors::has_variant_attr(
                attr,
                VariantAttrs::WarnMsg,
            ))
        }
        "error_msg" => {
            return Err(declaration_errors::has_variant_attr(
                attr,
                VariantAttrs::ErrorMsg,
            ))
        }
        _ => Ok(attr_name),
    }
}
