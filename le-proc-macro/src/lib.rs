extern crate proc_macro;
use proc_macro::TokenStream;

use regex::Regex;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[test]
fn lolwut() { }
