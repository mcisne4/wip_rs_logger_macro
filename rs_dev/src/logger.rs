use rs_logger::Logger;

// === Invalid Data Types === //

// #[derive(Logger)]
// pub struct MyLogger {
//   a: 42
// }

// #[derive(Logger)]
// pub union MyUnion {
//     a: i32,
// }

// // === Enum: No Attributes === //
// #[derive(Logger)]
// pub enum MyLogger {
//     Item1,
//     Item2,
//     Item3,
// }

// // === Enum: Has Variant Attributes === //
// #[derive(Logger)]
// #[info_msg = "Should Fail"]
// #[warn_msg = "Should Fail"]
// #[error_msg = "Should Fail"]
// pub enum MyLogger {
//     Item1,
//     Item2,
//     Item3,
// }

// === Enum: Empty Values === //
#[derive(Logger)]
#[crate_idx()]
#[module_idx()]
#[module_path]
pub enum MyLogger {
    Item1,
    Item2,
    Item3,
}

// === Enum: Empty Values === //
// #[derive(Logger)]
// #[info_msg = "Should Fail"]
// #[warn_msg = "Should Fail"]
// #[error_msg = "Should Fail"]
// #[crate_idx = "Hello World"]
// pub enum MyLogger {
//     Item1,
//     Item2,
//     Item3,
// }
