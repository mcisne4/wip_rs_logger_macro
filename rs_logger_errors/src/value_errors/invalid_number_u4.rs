use crate::example_code;

pub fn invalid_number_u4<Tokens: quote::ToTokens>(
    tokens: Tokens,
    is_tuple_value: bool,
) -> venial::Error {
    let mut msg = String::from("The 'crate_idx' attribute should be an integer between 0 and 15");

    msg += example_code::EXAMPLE;

    msg += match is_tuple_value {
        true => example_code::CRATE_IDX_TPL,
        false => example_code::CRATE_IDX_EQ,
    };

    venial::Error::new_at_tokens(tokens, msg)
}
