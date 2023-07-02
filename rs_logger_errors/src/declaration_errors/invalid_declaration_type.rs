/// Error for non-enum declaration types
pub fn invalid_declaration_type<Tokens: quote::ToTokens>(tokens: Tokens) -> venial::Error {
    venial::Error::new_at_tokens(
        tokens,
        "Implementation of 'Logger' is only available for enum types",
    )
}
