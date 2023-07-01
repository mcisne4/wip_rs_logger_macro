pub const EXAMPLE: &str = "\n\nExample:\n";

pub const ENUM_DEC: &str = "enum MyLogger {\n  ";
pub const DERIVE: &str = "#[derive(Logger)]\n";

pub const VARIANT1: &str = "  Variant1,\n";
pub const VARIANT2: &str = "  Variant2,\n";
pub const VARIANT3: &str = "  Variant3,\n";

pub const CRATE_IDX_EQ: &str = "#[crate_idx = 15]\n";
pub const CRATE_IDX_TPL: &str = "#[crate_idx(15)]\n";

pub const MOD_IDX_EQ: &str = "#[module_idx = 249]\n";
pub const MOD_IDX_TPL: &str = "#[module_idx(249)]\n";

pub const LOCATION_EQ: &str = "#[module_path = \"rs_logs::example::path\"]\n";
pub const LOCATION_TPL: &str = "#[module_path(\"rs_logs::example::path\")]\n";

pub const INFO_MSG_EQ: &str = "#[info_msg = \"This is an info message\"]\n";
pub const INFO_MSG_TPL: &str = "#[info_msg(\"This is an info message\")]\n";

pub const WARN_MSG_EQ: &str = "#[warn_msg = \"This is a warning message\"]\n";
pub const WARN_MSG_TPL: &str = "#[warn_msg(\"This is a warning message\")]\n";

pub const ERROR_MSG_EQ: &str = "#[error_msg = \"This is an error message\"]\n";
pub const ERROR_MSG_TPL: &str = "#[error_msg(\"This is an error message\")]\n";

mod enum_example;
pub use enum_example::enum_example;

mod variant_example;
pub use variant_example::variant_example;
