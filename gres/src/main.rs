use std::time::Instant;
use gres::cli::run;
use soulog::*;

fn main() {
    let time = Instant::now();
    run();
    println!("{}", colour_format![
        blue("["), green("Gres"), blue("] "),
        none("Proccess completed in "),
        green(&time.elapsed().as_secs_f64().to_string()),
        green("s"),
    ]);
}