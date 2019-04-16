extern crate clap;

use clap::{App, SubCommand};
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
                .arg_from_usage("--cmd=[COMMAND] 'Command to run inside container'"),
        )
        .get_matches();

    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => panic!("Failed to get home directory path"),
    };
    let home_dir = home_dir.to_str().unwrap();
    let default_image = "ubuntu";
    let default_image_dir = format!("{}/rubber-docker/images", home_dir);
    let default_container_dir = format!("{}/rubber-docker/containers", home_dir);
    let default_cmd = "/bin/ls";

    let mut image_name: String;
    let mut image_dir: String;
    let mut container_dir: String;
    let mut command: String;
    if let Some(matches) = matches.subcommand_matches("run") {
        image_name = match String::from_str(matches.value_of("image-name").unwrap_or(default_image))
        {
            Ok(s) => s,
            Err(_) => panic!("Failed to parse str"),
        };
        image_dir =
            match String::from_str(matches.value_of("image-dir").unwrap_or(&default_image_dir)) {
                Ok(s) => s,
                Err(_) => panic!("Failed to parse str"),
            };
        container_dir = match String::from_str(
            matches
                .value_of("container-dir")
                .unwrap_or(&default_container_dir),
        ) {
            Ok(s) => s,
            Err(_) => panic!("Failed to parse str"),
        };
        command = match String::from_str(matches.value_of("cmd").unwrap_or(default_cmd)) {
            Ok(s) => s,
            Err(_) => panic!("Failed to parse str"),
        };
        let args = Arguments::new(image_name, image_dir, container_dir, command);
        run(args);
    }
}
