extern crate clap;

use clap::{App, Arg, SubCommand};
use rubber_docker::{run, Arguments};
use std::str::FromStr;

fn main() {
    let matches = App::new("rubber-docker")
        .version("1.0")
        .author("Piotr Roslaniec <p.roslaniec@gmail.com>")
        .about("rubber-docker implemented in Rust")
        .subcommand(
            SubCommand::with_name("run")
                .about("run an image")
                .arg_from_usage("-i, --image-name=[PATH] 'Image to unpack'")
                .arg_from_usage("--image-dir=[PATH] 'Directory to unpack image'")
                .arg_from_usage("--container-dir=[PATH] 'Containers directory'")
                .arg(
                    Arg::with_name("command")
                        .long("command")
                        .short("c")
                        .required(true)
                        .min_values(1)
                        .help("Command to be executed"),
                ),
        )
        .get_matches();

    let default_image = "ubuntu";
    let default_image_dir = "/tmp/rdocker/images";
    let default_container_dir = "/tmp/rdocker/containers";

    if let Some(matches) = matches.subcommand_matches("run") {
        let image_name = String::from_str(matches.value_of("image-name").unwrap_or(default_image))
            .expect("Failed to parse str");
        let image_dir =
            String::from_str(matches.value_of("image-dir").unwrap_or(&default_image_dir))
                .expect("Failed to parse str");
        let container_dir = String::from_str(
            matches
                .value_of("container-dir")
                .unwrap_or(&default_container_dir),
        )
        .expect("Failed to parse str");
        let command: Vec<_> = matches
            .values_of("command")
            .unwrap()
            .map(|s| s.to_string())
            .collect();
        let args = Arguments::new(image_name, image_dir, container_dir, command);
        run(args);
    }
}
