/// Error for unmatched left bracket in attribute string
pub fn unmatched_left_bracket<Tokens: quote::ToTokens>(tokens: Tokens) -> venial::Error {
    venial::Error::new_at_tokens(
        tokens,
        concat!(
            "An unmatched left bracket '{' charcter was found\n\n",
            "If you are enclosig a parameter, a matching right bracket '}' ",
            "character is required\n",
            "- Example: \"'a' is '{}'\"\n\n",
            "If you intended to print '{', you can escape it using '{{'\n",
            "- Example: \"{{ a: 23 }}\"\n"
        ),
    )
}
