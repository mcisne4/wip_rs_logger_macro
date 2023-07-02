use crate::attributes::VariantAttrs;

/// Error for when a variant attribute is used on enum declarations
pub fn has_variant_attr<Tokens: quote::ToTokens>(
    tokens: Tokens,
    attr: VariantAttrs,
) -> venial::Error {
    venial::Error::new_at_tokens(
        tokens,
        format!(
            "The '{}' attribute can only be used on enum variants",
            attr.as_str()
        ),
    )
}
