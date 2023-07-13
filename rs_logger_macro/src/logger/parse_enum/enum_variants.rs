mod attr_value;
mod has_variant_attr;
mod log_id;
mod validate_value;
mod variant_contents;

use rs_logger_errors::LoggerResult;
use venial::{EnumVariant, Punctuated};

pub fn parse_enum_variants(
    variants: &Punctuated<EnumVariant>,
    id_prefix: String,
    log_path: String,
) -> LoggerResult<()> {
    for (variant_idx, (enum_variant, _punct)) in variants.into_iter().enumerate() {
        let ident = &enum_variant.name;
        eprintln!("\nVARIANT {:?}: {:#?}", variant_idx, ident.to_string());

        let (variant_attr, attr) = has_variant_attr::get_attribute(&enum_variant)?;
        let attr_value = attr_value::parse_attr_value(&attr, &variant_attr)?;
        let res = validate_value::validate_string_value(attr_value, attr);

        let id = log_id::generate_log_id(id_prefix.clone(), &variant_attr, variant_idx);

        let x = variant_contents::parse(&enum_variant.contents)?;
    }

    Ok(())
}
