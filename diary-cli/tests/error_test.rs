use diary_cli::error;

#[test]
#[should_panic(expected="Testing")]
fn error_panic_unwrap() {
    error::init::<()>("Testing", "Example Error", "Here is an example error body")
        .crash();
}