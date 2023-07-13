# RS_LOGGER

## WIP Strategy

### Implementation Example:

```rs
#[derive(Logger)]
#[crate_idx = 10]
#[module_idx = 255]
#[log_path = "rs_logger::example::path"]
enum MyLogger {
  #[info_msg = "A blank message"]
  Variant1,

  #[warn_msg = "A {} tuple message {}"]
  Variant2(String, usize),

  #[error_msg = "A {0} tuple message {1}"]
  Variant3(String, usize),

  #[log_msg("A {a} struct message {b}")]
  Variant4 {
    a: String,
    b: usize
  }
}
```

### Above Example Expanded:

```rs
enum MyLogger {
  Variant1,
  Variant2(String, usize),
  Variant3(String, usize),
  Variant4 { a: String, b: usize },
}
impl Logger for MyLogger {
  fn msg(&self) -> &str {
    let time = todo!();

    match self {
      Self::Variant1 =>        "[<time> INFO AFF100] A blank message",
      Self::Variant2(a, b) =>  "[<time> WARN AFF201] A <a> tuple message <b>",
      Self::Variant3(a, b) =>  "[<time> ERROR AFF302] A <a> tuple message <b>",
      Self::Variant4 {a, b} => "[<time> INFO AFF103] A <a> struct message <b>",
    }
  }

  fn source(id: &str) -> Option<&str> {
    match id {
      "AFF100" => Some("rs_logger::example::path"),
      "AFF201" => Some("rs_logger::example::path"),
      "AFF302" => Some("rs_logger::example::path"),
      "AFF103" => Some("rs_logger::example::path"),
      _ => None
    }
  }

  fn ids() -> std::collections::HashMap<&str, &str> {
    let ids = std::HashMap::new();

    ids.insert("AFF100", "rs_logger::example::path");
    ids.insert("AFF201", "rs_logger::example::path");
    ids.insert("AFF302", "rs_logger::example::path");
    ids.insert("AFF103", "rs_logger::example::path");

    ids
  }

  fn log(&self) {
    // Write 'self.msg' to log file
  }
}

```

### Todo Checklist:

- [x] Check Logger implementation is done on enum types only
- [x] Read enum attributes
  - [x] Parse 'crate_idx' attribute
  - [x] Parse 'module_idx' attribute
  - [x] Parse 'log_path' attribute
  - [x] Convert attribute data to ID prefix and log path source
- [x] Read variant entries
  - [x] Parse 'info_msg', 'warn_msg', or 'error_msg' attribute
  - [ ] Validate log message can be formatted
  - [ ] Read variant entry
  - [ ] Validate log message has appropriate variant parameters
  - [ ] Generate log entry
- [ ] Add 'msg' method
- [ ] Add 'source' method
- [ ] Add 'ids' method
- [ ] Add 'log' method
