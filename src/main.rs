extern crate clap;

mod lib;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("rubber-docker")
        .version("1.0")
        .author("Piotr Roslaniec <p.roslaniec@gmail.com>")
        .about("rubber-docker implemented in Rust")
        .subcommand(
            SubCommand::with_name("run")
                .about("run an executable")
                .arg_from_usage("-e, --executable=[PATH] 'path to executable'"),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(e) = matches.value_of("executable") {
            lib::run(e);
        }
    }
}
