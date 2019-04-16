use std::fs::{create_dir_all, File};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::string::String;
use uuid::Uuid;
use tar::Archive;

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
	create_container_root(
		args.image_name,
		args.image_dir,
		container_id.clone(),
		args.container_dir,
	);
	contain(args.command, container_id.clone());
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

fn get_image_path(image_name: String, image_dir: String, image_suffix: String) -> String {
	let image = format!("{}.{}", image_name, image_suffix);
	Path::new(&image_dir)
		.join(image)
		.to_str()
		.unwrap()
		.to_owned()
}

fn get_container_path(container_id: String, container_dir: String, subdir_name: String) -> String {
	Path::new(&container_dir)
		.join(container_id)
		.join(subdir_name)
		.to_str()
		.unwrap()
		.to_owned()
}

fn untar(image_path: String, container_root: String) -> Result<(), std::io::Error> {
	let tar = File::open(image_path)?;
	let mut archive = Archive::new(tar);
	archive.unpack(container_root)?;
	Ok(())
}

fn create_container_root(
	image_name: String,
	image_dir: String,
	container_id: String,
	container_dir: String,
) -> String {
	let image_suffix = "tar".to_owned();
	let image_path = get_image_path(image_name, image_dir, image_suffix);
	let subdir_name = "rootfs".to_owned();
	let container_root = get_container_path(container_id, container_dir, subdir_name);

	if !Path::new(&image_path).exists() {
		panic!(format!("Image path does not exist: {}", image_path))
	}
	if !Path::new(&image_path).exists() {
		match create_dir_all(&container_root) {
			Ok(r) => r,
			Err(_) => panic!(format!(
				"Failed to create container root directory: {}",
				container_root
			)),
		};
	}
	match untar(image_path, container_root.clone()) {
		Ok(r) => r,
		Err(_) => panic!("Failed to untar the image")
	}
	return container_root;
}
