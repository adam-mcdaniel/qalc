use clap::{clap_app, crate_authors, crate_version, AppSettings::ArgRequiredElseHelp};
use qalc::{process, target::TI84};
use std::fs::read_to_string;

fn main() {
    let matches = clap_app!(qalc =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "compiler/interpreter for qalc language")
        (@arg path: +required "path to qalc file")
        (@group target +required =>
            (@arg ti84: -t --ti84 "compile to ti84 basic")
            (@arg run: -r --run "interpret")
        )
    )
    .setting(ArgRequiredElseHelp)
    .get_matches();

    if matches.is_present("ti84") {
        if let Ok(contents) = read_to_string(matches.value_of("path").unwrap()) {
            if let Ok(out) = process(contents, TI84) {
                println!("{}", out);
            }
        } else {
            eprintln!("invalid input file");
        }
    } else if matches.is_present("run") {
        panic!("only compilation to ti84 basic is currently supported");
    } else {
        panic!("only compilation to ti84 basic is currently supported");
    }
}
