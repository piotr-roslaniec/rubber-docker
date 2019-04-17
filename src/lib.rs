use nix::mount::{mount, MsFlags};
use nix::unistd::{fork, ForkResult};
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::string::String;
use tar::Archive;
use uuid::Uuid;

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
	let container_id = uuid();
	println!("Creating new container with id: {}", container_id);

	match fork() {
		Ok(ForkResult::Parent { child, .. }) => println!("Spawned new child with pid: {}", child),
		Ok(ForkResult::Child) => {
			println!("Running in a new child process");
			contain(args, container_id.clone());
		}
		Err(_) => println!("Fork failed"),
	}
}

fn contain(args: Arguments, container_id: String) {
	let container_root = create_container_root(
		args.image_name,
		args.image_dir,
		container_id.clone(),
		args.container_dir,
	);
	println!("Created new root fs for container: {}", container_root);

	create_mounts(container_root.clone());

	let mut child = Command::new(&args.command[0])
		.args(&args.command[1..])
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

fn uuid() -> String {
	let mut buffer = Uuid::encode_buffer();
	let uuid = Uuid::new_v4().to_simple().encode_lower(&mut buffer);
	match String::from_str(uuid) {
		Ok(s) => s,
		Err(_) => panic!("Failed to parse uuid str"),
	}
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
		Err(_) => panic!("Failed to untar the image"),
	}
	return container_root;
}

fn create_mounts(container_root: String) {
	let root = Path::new(&container_root);
	let no_flags = MsFlags::from_bits(0).unwrap();
	let no_data: Option<&Path> = None;
	let mut tmpfs_flags = MsFlags::from_bits(0).unwrap();
	// Ignore suid and sgid bits
	tmpfs_flags.toggle(MsFlags::MS_NOSUID);
	// Always update last access time when files are accessed
	tmpfs_flags.toggle(MsFlags::MS_STRICTATIME);

	match mount(
		Some("proc"),
		&root.join("proc"),
		Some("proc"),
		no_flags,
		no_data,
	) {
		Ok(r) => r,
		Err(_) => panic!("Failed to create mount from host /proc to guest /proc"),
	}
	match mount(
		Some("sysfs"),
		&root.join("sys"),
		Some("sysfs"),
		no_flags,
		no_data,
	) {
		Ok(r) => r,
		Err(_) => panic!("Failed to create mount from host /sysfs to guest /sys"),
	}
	match mount(
		Some("tmpfs"),
		&root.join("dev"),
		Some("tmpfs"),
		tmpfs_flags,
		no_data,
	) {
		Ok(r) => r,
		Err(_) => panic!("Failed to create mount from host /tmpfs to guest /dev"),
	}
}
