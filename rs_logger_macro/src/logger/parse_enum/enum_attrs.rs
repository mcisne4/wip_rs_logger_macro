mod is_enum_attr;
mod parse_crate_idx;
mod parse_log_path;
mod parse_module_idx;

use super::super::parse_attr;
use proc_macro2::TokenStream;
use rs_logger_errors::{attributes::EnumAttrs, declaration_errors, LoggerResult};

pub struct EnumData {
    pub id_prefix: String,
    pub log_path: String,
}

pub fn parse_enum_attrs(
    attributes: &Vec<venial::Attribute>,
    decl_tokens: TokenStream,
) -> LoggerResult<EnumData> {
    if attributes.len() == 0 {
        return Err(declaration_errors::no_attrs(decl_tokens));
    }

    let mut crate_idx_opt: Option<String> = None;
    let mut module_idx_opt: Option<String> = None;
    let mut log_path_opt: Option<String> = None;

    for attr in attributes {
        let enum_attr = match is_enum_attr::which_enum_attr(&attr)? {
            Some(val) => val,
            None => continue,
        };

        match enum_attr {
            EnumAttrs::CrateIdx => match crate_idx_opt {
                Some(_) => return Err(declaration_errors::multiple_attrs(attr, enum_attr)),
                None => crate_idx_opt = Some(parse_crate_idx::parse_attr(attr)?),
            },
            EnumAttrs::ModuleIdx => match module_idx_opt {
                Some(_) => return Err(declaration_errors::multiple_attrs(attr, enum_attr)),
                None => module_idx_opt = Some(parse_module_idx::parse(attr)?),
            },
            EnumAttrs::LogPath => match log_path_opt {
                Some(_) => return Err(declaration_errors::multiple_attrs(attr, enum_attr)),
                None => log_path_opt = Some(parse_log_path::parse(attr)?),
            },
        }
    }

    let crate_idx = match crate_idx_opt {
        Some(value) => value,
        None => {
            return Err(declaration_errors::missing_attr(
                decl_tokens,
                EnumAttrs::CrateIdx,
            ))
        }
    };

    let module_idx = match module_idx_opt {
        Some(value) => value,
        None => {
            return Err(declaration_errors::missing_attr(
                decl_tokens,
                EnumAttrs::ModuleIdx,
            ))
        }
    };

    let log_path = match log_path_opt {
        Some(value) => value,
        None => {
            return Err(declaration_errors::missing_attr(
                decl_tokens,
                EnumAttrs::LogPath,
            ))
        }
    };

    let id_prefix = crate_idx + module_idx.as_str();

    Ok(EnumData {
        id_prefix,
        log_path,
    })
}
