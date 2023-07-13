use rs_logger_errors::attributes::VariantAttrs;

pub fn generate_log_id(
    id_prefix: String,
    variant_attr: &VariantAttrs,
    variant_idx: usize,
) -> String {
    let mut id = id_prefix;

    id += match variant_attr {
        VariantAttrs::InfoMsg => "1",
        VariantAttrs::WarnMsg => "2",
        VariantAttrs::ErrorMsg => "3",
    };

    id += format!("{:02X}", variant_idx).as_str();

    id
}
