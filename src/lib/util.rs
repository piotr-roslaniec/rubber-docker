use std::env;
use std::fs::File;
use std::process::{Command, Stdio};
use std::str::FromStr;
use tar::{Archive, EntryType};
use uuid::Uuid;

pub fn uuid() -> String {
    let mut buffer = Uuid::encode_buffer();
    let uuid = Uuid::new_v4().to_simple().encode_lower(&mut buffer);
    match String::from_str(uuid) {
        Ok(s) => s,
        Err(_) => panic!("Failed to parse uuid str"),
    }
}

pub fn untar(image_path: String, dest: String) {
    let tar = File::open(image_path).expect("Failed to access image file");
    let mut archive = Archive::new(tar);
    archive.set_preserve_permissions(true);

    for file in archive.entries().unwrap() {
        let mut file = file.unwrap();
        let entry_type = file.header().entry_type();
        // Tar archive may contain devices - filter them out
        if entry_type != EntryType::Char && entry_type != EntryType::Block {
            file.unpack_in(&dest)
                .expect("Failed to unpack file from tar archive");
        }
    }
}

pub fn execute(command: Vec<&str>) -> String {
    let child = Command::new(&command[0])
        .args(&command[1..])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute child");
    let output = child.wait_with_output().expect("Failed to wait on child");
    assert!(output.status.success());
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn is_debug() -> bool {
    match env::var_os("DEBUG") {
        Some(_) => true,
        None => false,
    }
}

pub fn print_debug(prefix: &str, data: String) {
    if is_debug() {
        let data: Vec<String> = data.lines().map(|line| format!("> {}", line)).collect();
        let data = data.join("\n");
        println!("=== {} =============\n{}\n", prefix.trim(), data.trim());
    }
}
