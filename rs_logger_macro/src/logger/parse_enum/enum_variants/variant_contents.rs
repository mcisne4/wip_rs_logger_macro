use rs_logger_errors::LoggerResult;

// pub enum VariantTypes {
//     NoValues,
//     Tuple,
//     Struct,
// }
// pub struct VariantContents {
//     pub variant_type: VariantTypes,
//     named
// }

pub fn parse(contents: &venial::StructFields) -> LoggerResult<()> {
    // eprintln!("{:#?}", contents);

    match &contents {
        venial::StructFields::Unit => (),
        venial::StructFields::Named(nm) => (),
        venial::StructFields::Tuple(tpl) => (),
    }

    Ok(())
}
