use crate::example_code;

pub fn missing_attr<Tokens: quote::ToTokens>(tokens: Tokens) -> venial::Error {
    let mut msg = concat!(
        "The 'Logger' implementation requires each enum variant to have ",
        "either an 'info_msg', a 'warn_msg', or a 'error_msg' attribute",
    )
    .to_owned();

    msg += example_code::variant_example().as_str();

    venial::Error::new_at_tokens(tokens, msg)
}
