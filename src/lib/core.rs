use std::char::from_digit;
use std::fs::create_dir_all;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::Command;
use std::string::String;

use nix::mount::{MntFlags, mount, MsFlags, umount2};
use nix::sched::{CloneFlags, unshare};
use nix::sys::stat::{makedev, mknod, Mode, SFlag};
use nix::unistd::chdir;
use nix::unistd::pivot_root;

use crate::lib::util;

pub fn contain(
    image_name: String,
    image_dir: String,
    container_dir: String,
    command: Vec<String>,
    container_id: String,
) {
    unshare(CloneFlags::CLONE_NEWNS).expect("Failed to unshare");
    println!("Unshared the mount namespace");

    let none: Option<&str> = None;
    let mut ns_flags = MsFlags::from_bits(0).unwrap();
    // Mount as private namespace to prevent host namespace pollution
    ns_flags.toggle(MsFlags::MS_PRIVATE);
    // Bind mount directories recursively
    ns_flags.toggle(MsFlags::MS_REC);
    let old_root = Path::new("/");

    mount(none, old_root, none, ns_flags, none)
        .expect("Failed to mount at new root");

    let new_root = create_container_root(
        image_name,
        image_dir,
        container_id.clone(),
        container_dir,
    );
    println!("Created new root fs for container: {}", new_root);

    create_mounts(new_root.clone());
    println!("Created mounts");

    add_devices(new_root.clone());
    println!("Added devices");

    pivot_root(Path::new(&new_root.clone()), old_root)
        .expect("Failed to pivot_root");
    println!("pivot_root-ed into container root");

    chdir(old_root).expect("Failed to chdir");
    println!("chdir-ed into container root");

    // Perform a lazy unmount
    umount2(old_root, MntFlags::MNT_DETACH)
        .expect("Failed to unmount old root");

    execute(command);
}

fn execute(command: Vec<String>) {
    let mut child = Command::new(&command[0])
        .args(&command[1..])
        .spawn()
        .expect("Failed to execute child command");
    let ecode = child.wait().expect("Failed to wait on child command");
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
    create_dir_all(&container_root).unwrap();
    return container_root;
}

fn create_mounts(container_root: String) {
    let root = Path::new(&container_root);
    let proc_guest = root.join("proc");
    let sys_guest = root.join("sys");
    let dev_guest = root.join("dev");

    let no_flags = MsFlags::from_bits(0).unwrap();
    let no_data: Option<&str> = None;
    let mode_755: Option<&str> = Some("mode=755");
    let mut tmpfs_flags = MsFlags::from_bits(0).unwrap();
    // Ignore suid and sgid bits
    tmpfs_flags.toggle(MsFlags::MS_NOSUID);
    // Always update last access time when files are accessed
    tmpfs_flags.toggle(MsFlags::MS_STRICTATIME);

    mount(Some("proc"), &proc_guest, Some("proc"), no_flags, no_data).expect(&format!(
        "Failed to create mount from host /proc to guest {}",
        &proc_guest.to_str().unwrap()
    ));
    mount(Some("sysfs"), &sys_guest, Some("sysfs"), no_flags, no_data).expect(&format!(
        "Failed to create mount from host /sysfs to guest {}",
        &sys_guest.to_str().unwrap()
    ));
    mount(
        Some("tmpfs"),
        &dev_guest,
        Some("tmpfs"),
        tmpfs_flags,
        mode_755,
    )
        .expect(&format!(
            "Failed to create mount from host /tmpfs to guest {}",
            &dev_guest.to_str().unwrap()
        ));
}

fn add_devices(container_root: String) {
    // Add basic devices
    let dev_path = Path::new(&container_root).join("dev");
    let devpts_path = dev_path.join("pts");
    if !devpts_path.exists() {
        create_dir_all(&devpts_path).expect("Failed to create /dev/pts directory");
    }

    let no_flags = MsFlags::from_bits(0).unwrap();
    let no_data: Option<&str> = None;
    mount(
        Some("devpts"),
        &devpts_path,
        Some("devpts"),
        no_flags,
        no_data,
    )
        .expect(&format!(
            "Failed to create mount from host /devpts to guest {}",
            &devpts_path.to_str().unwrap()
        ));
    for (i, device) in vec!["stdin", "stdout", "stderr"].iter().enumerate() {
        let mut dev_num = String::from("");
        dev_num.insert(0, from_digit(i as u32, 10).unwrap());
        let source = Path::new("/proc/self/fd").join(dev_num);
        let dest = Path::new(&container_root).join("dev").join(device);
        symlink(&source, &dest).expect(&format!(
            "Failed to create symlink from {} to {}",
            source.to_str().unwrap(),
            dest.to_str().unwrap()
        ));
    }

    // Add extra devices
    let devices = vec![
        ("null", SFlag::S_IFCHR, 1, 3),
        ("zero", SFlag::S_IFCHR, 1, 5),
        ("random", SFlag::S_IFCHR, 1, 8),
        ("urandom", SFlag::S_IFCHR, 1, 9),
        ("console", SFlag::S_IFCHR, 136, 3),
        ("null", SFlag::S_IFCHR, 1, 3),
    ];

    for (device, kind, major, minor) in devices {
        let device_path = dev_path.join(device);
        if !device_path.exists() {
            let device_path = device_path.to_str().unwrap();
            let perm = Mode::from_bits(0666).unwrap();
            let device = makedev(major, minor);
            mknod(device_path, kind, perm, device)
                .expect(&format!("Failed to mknod device: {}", device_path));
        }
    }
}
