use std::process::Command;

pub fn run(exec_path: &str) {
	let mut child = Command::new(exec_path)
		.arg(".")
		.spawn()
		.expect("Failed to execute child");
	let ecode = child.wait().expect("Failed to wait on child");
	println!("The PID is: {}", child.id());
	println!("The exit code is: {}", ecode);
}