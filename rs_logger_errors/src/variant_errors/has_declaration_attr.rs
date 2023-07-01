use crate::attributes::EnumAttrs;

pub fn has_declaration_attr<Tokens: quote::ToTokens>(
    tokens: Tokens,
    attr: EnumAttrs,
) -> venial::Error {
    venial::Error::new_at_tokens(
        tokens,
        format!(
            "The '{}' attribute can only be used on enum declarations",
            attr.as_str()
        ),
    )
}
