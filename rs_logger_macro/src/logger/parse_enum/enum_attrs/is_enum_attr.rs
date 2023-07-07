use rs_logger_errors::attributes::{EnumAttrs, VariantAttrs};
use rs_logger_errors::declaration_errors;
use rs_logger_errors::LoggerResult;

pub fn which_enum_attr(attr: &venial::Attribute) -> LoggerResult<Option<EnumAttrs>> {
    let attr_name = attr.path[0].to_string();

    match attr_name.as_str() {
        "crate_idx" => Ok(Some(EnumAttrs::CrateIdx)),
        "module_idx" => Ok(Some(EnumAttrs::ModuleIdx)),
        "log_path" => Ok(Some(EnumAttrs::LogPath)),
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
        _ => Ok(None),
    }
}
