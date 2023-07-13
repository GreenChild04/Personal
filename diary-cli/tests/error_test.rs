use diary_cli::error;

#[test]
#[should_panic(expected="Testing")]
fn error_panic_unwrap() {
    error::init::<()>("Testing", "Example Error", "Here is an example error body")
        .crash();
}

#[test]
#[should_panic(expected="Fatal")]
fn error_retry() {
    error::init::<()>("Testing", "Example Retry", "Here is an example error retry")
        .retry(3, || Err("ono"));
}