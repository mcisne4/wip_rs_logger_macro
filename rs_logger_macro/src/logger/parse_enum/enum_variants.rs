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
        let x = variant_contents::parse(&enum_variant.contents)?;

        // let c = &enum_variant.contents;
    }

    Ok(())
}
