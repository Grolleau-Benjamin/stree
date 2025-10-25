use stree::cli::args::get_hello;

#[test]
fn get_hello_correct_output() {
    assert_eq!(get_hello(), "Hello, world!");
}
