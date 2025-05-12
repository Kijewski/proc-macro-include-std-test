#![feature(proc_macro_expand)]
#![feature(proc_macro_span)]
#![feature(proc_macro_value)]

use std::str::FromStr;

use proc_macro::{Ident, TokenStream, TokenTree};

#[proc_macro]
pub fn extract_edition(_item: TokenStream) -> TokenStream {
    // Include a text file. Path is relative to caller's location.
    let source = "core::prelude::rust_2021::include_str!(\"../../Cargo.toml\")";
    let source = TokenStream::from_str(source).unwrap().expand_expr().unwrap();
    let source = source.into_iter().next().unwrap();
    let TokenTree::Literal(source) = source else {
        panic!("expected literal, got {source:#?}");
    };
    let text = source.str_value().unwrap();

    // Extract a word in `source`, and build an identifier with its span
    let (edition_start, edition) = text.match_indices("edition").next().unwrap();
    let span = source.subspan(edition_start .. edition_start + edition.len()).unwrap();
    let edition = Ident::new(edition, span);
    let edition = TokenTree::Ident(edition);

    // Return the identifier.
    TokenStream::from_iter([edition])
}
