extern crate proc_macro;

mod code_gen;
mod parser;

use crate::parser::Node;
use crate::proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let node = parse_macro_input!(input as Node);
    code_gen::gen_node(node).into()
}