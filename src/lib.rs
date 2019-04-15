use std::process::Command;
use std::str::FromStr;
use std::string::String;
use uuid::Uuid;

fn uuid() -> String {
	let mut buffer = Uuid::encode_buffer();
	let uuid = Uuid::new_v4().to_simple().encode_lower(&mut buffer);
	match String::from_str(uuid) {
		Ok(s) => s,
		Err(_) => panic!("Failed to parse uuid str"),
	}
}

#[derive(Debug)]
pub struct Arguments {
	image_name: String,
	image_dir: String,
	container_dir: String,
	command: String,
}

impl Arguments {
	pub fn new(
		image_name: String,
		image_dir: String,
		container_dir: String,
		command: String,
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
	let container_id = uuid();
	contain(args.command, container_id);
}

fn contain(command: String, container_id: String) {
	let mut child = Command::new(command)
		.arg(".")
		.spawn()
		.expect("Failed to execute child");
	let ecode = child.wait().expect("Failed to wait on child");
	println!("The PID is: {}", child.id());
	println!("The exit code is: {}", ecode);
}
