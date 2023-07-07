mod decl_tokens;
mod enum_attrs;

use rs_logger_errors::LoggerResult;

pub fn parse_enum_data(enum_ast: &venial::Enum) -> LoggerResult<()> {
    let decl_tokens = decl_tokens::get_declaration_tokens(&enum_ast);
    let enum_data = enum_attrs::parse_enum_attrs(&enum_ast.attributes, decl_tokens)?;

    Ok(())
}
