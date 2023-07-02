use crate::example_code;

/// Error for the module_idx attribute if the value is not between 0 and 255
pub fn invalid_number_u8<Tokens: quote::ToTokens>(
    tokens: Tokens,
    is_tuple_value: bool,
) -> venial::Error {
    let mut msg = String::from("The 'module_idx' attribute should be an integer between 0 and 255");

    msg += example_code::EXAMPLE;

    msg += match is_tuple_value {
        true => example_code::MOD_IDX_TPL,
        false => example_code::MOD_IDX_EQ,
    };

    venial::Error::new_at_tokens(tokens, msg)
}
