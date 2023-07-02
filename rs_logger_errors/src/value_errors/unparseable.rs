use crate::attributes::Attrs;
use crate::example_code;

/// Error for when an attribute cannot be parsed
pub fn unparseable<Tokens: quote::ToTokens>(
    tokens: Tokens,
    attr: Attrs,
    is_tuple_value: bool,
) -> venial::Error {
    let mut msg = format!(
        "Could not parse the '{}' attribute\n\nThe '{}' attribute should be formatted as follows:",
        attr.as_str(),
        attr.as_str()
    );
    msg += "\n- ";

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
            Attrs::CrateIdx => example_code::CRATE_IDX_EQ,
            Attrs::ModuleIdx => example_code::MOD_IDX_EQ,
            Attrs::ModulePath => example_code::LOCATION_EQ,
            Attrs::InfoMsg => example_code::INFO_MSG_EQ,
            Attrs::WarnMsg => example_code::WARN_MSG_EQ,
            Attrs::ErrorMsg => example_code::ERROR_MSG_EQ,
        },
    };

    venial::Error::new_at_tokens(tokens, msg)
}
