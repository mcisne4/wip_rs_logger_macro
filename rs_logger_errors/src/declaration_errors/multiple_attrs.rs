use crate::attributes::EnumAttrs;
use crate::example_code;

/// Error for multiple attributes of the same type
pub fn multiple_attrs<Tokens: quote::ToTokens>(tokens: Tokens, attr: EnumAttrs) -> venial::Error {
    let mut msg = format!(
        "The '{}' attribute is only allowed once per enum declaration",
        attr.as_str()
    );

    msg += example_code::EXAMPLE;
    msg += example_code::enum_example().as_str();

    venial::Error::new_at_tokens(tokens, msg)
}
