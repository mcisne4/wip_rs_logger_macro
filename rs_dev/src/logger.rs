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
// #[derive(Logger)]
// #[crate_idx()]
// #[module_idx()]
// #[log_path]
// pub enum MyLogger {
//     Item1,
//     Item2,
//     Item3,
// }

// === Enum: Invalid Value Types === //
// #[derive(Logger)]
// #[crate_idx = "Hello World"]
// #[crate_idx("Hello World")]
// #[module_idx = false]
// #[module_idx(false)]
// #[log_path = 236]
// #[log_path(236)]
// pub enum MyLogger {
//     Item1,
//     Item2,
//     Item3,
// }

// === Enum: Invalid Integer Values === //
// #[derive(Logger)]
// #[crate_idx = -5]
// #[crate_idx = 16]
// #[module_idx(-4)]
// #[module_idx(256)]
// #[log_path = "rs_logger::test::path"]
// pub enum MyLogger {
//     Item1,
//     Item2,
//     Item3,
// }

// === Enum: Multiple Values === //
// #[derive(Logger)]
// #[crate_idx(12, 15, 5)]
// #[module_idx(a = 4, b = 34)]
// #[log_path = "rs_logger::test::path"]
// pub enum MyLogger {
//     Item1,
//     Item2,
//     Item3,
// }

// === Enum: Missing Attributes === //
// #[derive(Logger)]
// #[module_idx(255)]
// #[log_path = "rs_logger::test::path"]
// pub enum MyLoggerA {
//     Item1,
//     Item2,
//     Item3,
// }

// #[derive(Logger)]
// #[crate_idx = 15]
// #[log_path = "rs_logger::test::path"]
// pub enum MyLoggerB {
//     Item1,
//     Item2,
//     Item3,
// }

// #[derive(Logger)]
// #[crate_idx = 15]
// #[module_idx(255)]
// pub enum MyLoggerC {
//     Item1,
//     Item2,
//     Item3,
// }

// === Enum: Multiple Attributes === //
// #[derive(Logger)]
// #[crate_idx = 15]
// #[crate_idx = 15]
// #[module_idx = 255]
// #[module_idx = 255]
// #[log_path = "rs_logger::test::path"]
// #[log_path = "rs_logger::test::path"]
// pub enum MyLogger {
//     Item1,
//     Item2,
//     Item3,
// }

// === Enum: Proper Values (EQUAL) === //
#[derive(Logger)]
#[crate_idx = 15]
#[module_idx = 255]
#[log_path = "rs_logger::test::path"]
pub enum MyLogger1 {
    Item1,
    Item2,
    Item3,
}

// === Enum: Proper Values (TUPLE) === //
#[derive(Logger)]
#[crate_idx(13)]
#[module_idx(155)]
#[log_path("rs_logger::test::path")]
pub enum MyLogger2 {
    Item1,
    Item2,
    Item3,
}

// === Enum: Proper Values (Combination) === //
#[derive(Logger)]
#[crate_idx = 4]
#[module_idx(4)]
#[log_path = "rs_logger::test::path"]
pub enum MyLogger3 {
    Item1,
    Item2,
    Item3,
}
