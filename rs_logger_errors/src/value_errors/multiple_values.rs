use crate::attributes::Attrs;
use crate::example_code;

/// Error for when multiple values are found for an attribute
pub fn multiple_values<Tokens: quote::ToTokens>(
    tokens: Tokens,
    attr: Attrs,
    is_tuple_value: bool,
) -> venial::Error {
    let mut msg = format!(
        "Multiple values found for the '{}' attribute. The '{}' attribute requires ",
        attr.as_str(),
        attr.as_str()
    );

    msg += match attr {
        Attrs::CrateIdx => "a single integer value to be passed",
        Attrs::ModuleIdx => "a single integer value to be passed",
        _ => "a single string value to be passed",
    };

    msg += example_code::EXAMPLE;

    msg += match is_tuple_value {
        true => match attr {
            Attrs::CrateIdx => example_code::CRATE_IDX_TPL,
            Attrs::ModuleIdx => example_code::MOD_IDX_TPL,
            Attrs::ModulePath => example_code::LOCATION_TPL,
            Attrs::InfoMsg => example_code::INFO_MSG_TPL,
            Attrs::WarnMsg => example_code::WARN_MSG_TPL,
            Attrs::ErrorMsg => example_code::ERROR_MSG_TPL,
        },
        false => match attr {
            Attrs::CrateIdx => example_code::CRATE_IDX_TPL,
            Attrs::ModuleIdx => example_code::MOD_IDX_TPL,
            Attrs::ModulePath => example_code::LOCATION_TPL,
            Attrs::InfoMsg => example_code::INFO_MSG_TPL,
            Attrs::WarnMsg => example_code::WARN_MSG_TPL,
            Attrs::ErrorMsg => example_code::ERROR_MSG_TPL,
        },
    };

    venial::Error::new_at_tokens(tokens, msg)
}
