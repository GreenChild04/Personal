use diary_cli::cli;

fn main() {
    // _io_test(); // Testing to do with the io
    cli::run();
}

fn _io_test() {
    diary_cli::input_yes_no("Test", "Do you want candy?");
    std::process::exit(0); // End after io_test
}
