use crate::attributes::EnumAttrs;
use crate::example_code;

/// Error for missing enum declaration attributes
pub fn missing_attr<Tokens: quote::ToTokens>(tokens: Tokens, attr: EnumAttrs) -> venial::Error {
    let mut msg = format!(
        "The '{}' attribute is required in the enum declarations",
        attr.as_str(),
    );

    // msg += example_code::EXAMPLE;
    msg += example_code::enum_example().as_str();

    venial::Error::new_at_tokens(tokens, msg)
}
