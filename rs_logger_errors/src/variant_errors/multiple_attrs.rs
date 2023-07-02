/// Error for more than one attribute provided for a variant
pub fn multiple_attrs<Tokens: quote::ToTokens>(tokens: Tokens) -> venial::Error {
    venial::Error::new_at_tokens(
        tokens,
        concat!(
            "Multiple attributes were provided for the variant. ",
            "Only one 'info_msg' attribute, 'warn_msg' attribute, or ",
            "'error_msg' attribute is allowed per variant"
        ),
    )
}
