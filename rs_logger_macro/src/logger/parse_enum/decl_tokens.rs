use proc_macro2::{TokenStream, TokenTree};

pub fn get_declaration_tokens(enum_ast: &venial::Enum) -> proc_macro2::TokenStream {
    let mut token_trees: Vec<TokenTree> = vec![];

    if let Some(vis) = &enum_ast.vis_marker {
        token_trees.push(vis.tk_token1.clone());
    }

    token_trees.push(enum_ast.tk_enum.clone().into());
    token_trees.push(enum_ast.name.clone().into());

    TokenStream::from_iter(token_trees)
}
