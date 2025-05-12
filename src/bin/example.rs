use proc_macro_include_std_test::extract_edition;

fn main() {
    // `extract_edition!()` extract the identifier `edition` in `Cargo.toml`.
    // The identifier is undefined in this scope, so it gets underlined in the error message.
    println!("{}", extract_edition!());
}
