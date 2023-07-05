use crate::example_code;

/// Error for enum declaration having no attributes
pub fn no_attrs<Tokens: quote::ToTokens>(tokens: Tokens) -> venial::Error {
    let mut msg = concat!(
        "No attributes were provided for the enum declaration. ",
        "The 'Logger' implementation requires the enum declaration to have ",
        "a 'crate_idx' attribute, a 'module_idx' attribute, and a 'module_path' attribute"
    )
    .to_owned();

    msg += example_code::enum_example().as_str();

    venial::Error::new_at_tokens(tokens, msg)
}
