use crate::attributes::Attrs;
use crate::example_code;

pub fn invalid_type<Tokens: quote::ToTokens>(
    tokens: Tokens,
    attr: Attrs,
    is_tuple_value: bool,
) -> venial::Error {
    let mut msg = format!(
        "An invalid value type was provided for the `{}` attribute. ",
        attr.as_str()
    );

    match attr {
        Attrs::CrateIdx => {
            msg += "The `crate_idx` attribute value should  be an integer between 0 and 15"
        }
        Attrs::ModIdx => {
            msg += "The `mod_idx` value attribute should be an integer between 0 and 255"
        }
        _ => {
            msg += &format!("The '{}' attribute value should be a string", attr.as_str());
        }
    }

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
