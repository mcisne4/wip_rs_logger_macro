mod enum_decl;

use rs_logger_errors::LoggerResult;

pub fn logger(input: proc_macro::TokenStream) -> LoggerResult<proc_macro2::TokenStream> {
    let input = proc_macro2::TokenStream::from(input);

    // --- Parse Token Stream --- //
    let declaration = venial::parse_declaration(input)?;

    // --- Validate Enum Declaration --- //
    let _enum_data = enum_decl::validate_enum_decl(&declaration)?;

    // --- To Token Stream --- //
    Ok(quote::quote! {
          //
    })
}
