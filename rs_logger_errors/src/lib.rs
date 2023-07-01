mod example_code;

pub mod attributes;

pub mod declaration_errors;
pub mod value_errors;
pub mod variant_errors;

pub type LoggerResult<T> = std::result::Result<T, venial::Error>;
