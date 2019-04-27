use nix::unistd::fork;
use nix::unistd::ForkResult;

use crate::lib::{core, util};

#[derive(Debug)]
pub struct Arguments {
    image_name: String,
    image_dir: String,
    container_dir: String,
    command: Vec<String>,
}

impl Arguments {
    pub fn new(
        image_name: String,
        image_dir: String,
        container_dir: String,
        command: Vec<String>,
    ) -> Arguments {
        Arguments {
            image_name,
            image_dir,
            container_dir,
            command,
        }
    }
}


pub fn run(args: Arguments) {
    let container_id = util::uuid();
    println!("Creating new container with id: {}", container_id);

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => println!("Spawned new child with pid: {}", child),
        Ok(ForkResult::Child) => {
            println!("Running in a new child process");
            core::contain(
                args.image_name,
                args.image_dir,
                args.container_dir,
                args.command,
                container_id.clone(),
            );
        }
        Err(_) => println!("Fork failed"),
    }
}
