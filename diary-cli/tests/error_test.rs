use diary_cli::error::Error;

#[test]
#[should_panic(expected="[Testing]")]
fn inter_error_panic_unwrap() {
    Error::throw("Testing", "Example Error", "Here is an example error body");
}