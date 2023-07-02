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

// === Enum Implementations === //
#[derive(Logger)]
pub enum MyLogger {
    Item1,
    Item2,
    Item3,
}
