mod logger;

#[proc_macro_derive(
    Logger,
    attributes(crate_idx, module_idx, log_path, info_msg, warn_msg, error_msg)
)]
pub fn logger_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match logger::logger(input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
