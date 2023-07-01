#[proc_macro_derive(
    Logger,
    attributes(crate_idx, module_idx, module_path, info_msg, warn_msg, error_msg)
)]
pub fn logger_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}
