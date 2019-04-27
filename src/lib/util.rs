use std::fs::File;
use std::str::FromStr;

use tar::Archive;
use uuid::Uuid;

pub fn uuid() -> String {
    let mut buffer = Uuid::encode_buffer();
    let uuid = Uuid::new_v4().to_simple().encode_lower(&mut buffer);
    match String::from_str(uuid) {
        Ok(s) => s,
        Err(_) => panic!("Failed to parse uuid str"),
    }
}

pub fn untar(image_path: String, container_root: String) {
    let tar = File::open(image_path).expect("Failed to access image file");
    let mut archive = Archive::new(tar);
    archive.set_preserve_permissions(true);
    archive
        .unpack(container_root)
        .expect("Failed to unpack image to container root");
}
