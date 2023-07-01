use crate::attributes::Attrs;
use crate::example_code;

pub fn is_empty<Tokens: quote::ToTokens>(
    tokens: Tokens,
    attr: Attrs,
    is_tuple_value: bool,
) -> venial::Error {
    let mut msg = format!(
        "No value provided for the '{}' attribute. The '{}' attribute requires ",
        attr.as_str(),
        attr.as_str()
    );

    msg += match attr {
        Attrs::CrateIdx => "an integer value to be passed",
        Attrs::ModIdx => "an integer value to be passed",
        _ => "a string value to be passed",
    };

    msg += example_code::EXAMPLE;

    msg += match is_tuple_value {
        true => match attr {
            Attrs::CrateIdx => example_code::CRATE_IDX_TPL,
            Attrs::ModIdx => example_code::MOD_IDX_TPL,
            Attrs::Location => example_code::LOCATION_TPL,
            Attrs::InfoMsg => example_code::INFO_MSG_TPL,
            Attrs::WarnMsg => example_code::WARN_MSG_TPL,
            Attrs::ErrorMsg => example_code::ERROR_MSG_TPL,
        },
        false => match attr {
            Attrs::CrateIdx => example_code::CRATE_IDX_EQ,
            Attrs::ModIdx => example_code::MOD_IDX_EQ,
            Attrs::Location => example_code::LOCATION_EQ,
            Attrs::InfoMsg => example_code::INFO_MSG_EQ,
            Attrs::WarnMsg => example_code::WARN_MSG_EQ,
            Attrs::ErrorMsg => example_code::ERROR_MSG_EQ,
        },
    };

    venial::Error::new_at_tokens(tokens, msg)
}
