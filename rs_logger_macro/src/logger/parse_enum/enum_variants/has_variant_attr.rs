use rs_logger_errors::{
    attributes::{EnumAttrs, VariantAttrs},
    variant_errors, LoggerResult,
};

pub fn get_attribute(
    enum_variant: &venial::EnumVariant,
) -> LoggerResult<(VariantAttrs, &venial::Attribute)> {
    let mut variant_data_opt: Option<(VariantAttrs, &venial::Attribute)> = None;

    for attr in &enum_variant.attributes {
        if let Some(ident) = attr.get_single_path_segment() {
            let variant_attr_opt = match ident.to_string().as_str() {
                "crate_idx" => {
                    return Err(variant_errors::has_declaration_attr(
                        attr,
                        EnumAttrs::CrateIdx,
                    ))
                }
                "module_idx" => {
                    return Err(variant_errors::has_declaration_attr(
                        attr,
                        EnumAttrs::ModuleIdx,
                    ));
                }
                "log_path" => {
                    return Err(variant_errors::has_declaration_attr(
                        attr,
                        EnumAttrs::LogPath,
                    ))
                }
                "info_msg" => Some(VariantAttrs::InfoMsg),
                "warn_msg" => Some(VariantAttrs::WarnMsg),
                "error_msg" => Some(VariantAttrs::ErrorMsg),
                _ => None,
            };

            if let Some(variant_attr) = variant_attr_opt {
                match variant_data_opt {
                    Some(_) => return Err(variant_errors::multiple_attrs(attr)),
                    None => variant_data_opt = Some((variant_attr, &attr)),
                }
            }
        }
    }

    let variant_data = match variant_data_opt {
        None => return Err(variant_errors::missing_attr(enum_variant)),
        Some(data) => data,
    };
    Ok(variant_data)
}
