mod enum_decl;
mod parse_attr;
mod parse_enum;

use rs_logger_errors::LoggerResult;

pub fn logger(input: proc_macro::TokenStream) -> LoggerResult<proc_macro2::TokenStream> {
    let input = proc_macro2::TokenStream::from(input);

    // --- Parse Token Stream --- //
    let declaration = venial::parse_declaration(input)?;

    // --- Validate Enum Declaration --- //
    let enum_data = enum_decl::validate_enum_decl(&declaration)?;

    // --- Parse Enum Data --- //
    let parsed_data = parse_enum::parse_enum_data(&enum_data)?;

    // --- To Token Stream --- //
    Ok(quote::quote! {
          //
    })
}
