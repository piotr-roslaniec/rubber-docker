extern crate clap;

use clap::{App, Arg, SubCommand};
use rubber_docker::{run, Arguments};
use std::env;
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

    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => panic!("Failed to get home directory path"),
    };
    let home_dir = home_dir.to_str().unwrap();
    let default_image = "busybox";
    let default_image_dir = format!("{}/rubber-docker/images", home_dir);
    let default_container_dir = format!("{}/rubber-docker/containers", home_dir);

    if let Some(matches) = matches.subcommand_matches("run") {
        let image_name =
            match String::from_str(matches.value_of("image-name").unwrap_or(default_image)) {
                Ok(s) => s,
                Err(_) => panic!("Failed to parse str"),
            };
        let image_dir =
            match String::from_str(matches.value_of("image-dir").unwrap_or(&default_image_dir)) {
                Ok(s) => s,
                Err(_) => panic!("Failed to parse str"),
            };
        let container_dir = match String::from_str(
            matches
                .value_of("container-dir")
                .unwrap_or(&default_container_dir),
        ) {
            Ok(s) => s,
            Err(_) => panic!("Failed to parse str"),
        };
        let command: Vec<_> = matches
            .values_of("command")
            .unwrap()
            .map(|s| s.to_string())
            .collect();
        let args = Arguments::new(image_name, image_dir, container_dir, command);
        run(args);
    }
}
