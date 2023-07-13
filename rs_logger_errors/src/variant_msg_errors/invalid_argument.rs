/// Error for invalid parameter in the attribute string
pub fn invalid_argument<Tokens: quote::ToTokens>(tokens: Tokens) -> venial::Error {
    venial::Error::new_at_tokens(
        tokens,
        concat!(
            "An invalid formatted string parameter was found\n\n",
            "Parameters should be formatted as follows:\n",
            "- tuple parameter: \"{}\"\n",
            "- tuple parameter: \"{0}\"\n",
            "- struct parameter: \"{abc}\"\n"
        ),
    )
}
