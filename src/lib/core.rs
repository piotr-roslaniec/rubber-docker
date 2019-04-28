use std::char::from_digit;
use std::fs::{create_dir_all, remove_dir_all};
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;
use std::string::String;

use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::sched::{unshare, CloneFlags};
use nix::sys::stat::{makedev, mknod, Mode, SFlag};
use nix::unistd::chdir;
use nix::unistd::pivot_root;

use crate::lib::cli;
use crate::lib::util;

#[derive(Debug)]
pub struct Container {
    image_name: String,
    image_dir: String,
    container_dir: String,
    command: Vec<String>,
    container_id: String,

}

impl Container {
    pub fn new(args: cli::Arguments) -> Container {
        Container {
            image_name: args.image_name,
            image_dir: args.image_dir,
            container_dir: args.container_dir,
            command: args.command,
            container_id: util::uuid(),
        }
    }

    pub fn get_container_id(&self) -> &String {
        &self.container_dir
    }

    pub fn contain(&self) {
        // Unshare the namespaces from the parent.
        // CLONE_NEWNS will initialize child with new mount namespace
        // with a copy of the namespace of the parent.
        unshare(CloneFlags::CLONE_NEWNS).expect("Failed to unshare");

        let none: Option<&str> = None;
        let mut ns_flags = MsFlags::from_bits(0).unwrap();
        // Mount as private namespace to prevent host namespace pollution
        ns_flags.toggle(MsFlags::MS_PRIVATE);
        // Bind mount directories recursively
        ns_flags.toggle(MsFlags::MS_REC);
        mount(none, Path::new("/"), none, ns_flags, none).expect("Failed to mount at new root");

        let container_rootfs = create_container_rootfs(
            self.container_id.clone(),
            self.container_dir.clone(),
            self.image_name.clone(),
            self.image_dir.clone(),
        );
        create_mounts(container_rootfs.clone());
        add_devices(container_rootfs.clone());

        let old_root = Path::new(&container_rootfs).join("old-root");
        create_dir_all(old_root).expect("Failed to create old-root directory");
        pivot_root(Path::new(&container_rootfs.clone()), Path::new("/"))
            .expect("Failed to pivot_root");

        chdir(Path::new("/")).expect("Failed to chdir");

        // Perform a lazy unmount
        umount2(Path::new("/old-root"), MntFlags::MNT_DETACH).expect("Failed to unmount old root");
        remove_dir_all(Path::new("/old-root")).expect("Failed to remove old root");

        execute(self.command.clone());
    }
}

fn execute(command: Vec<String>) -> ExitStatus {
    let mut child = Command::new(&command[0])
        .args(&command[1..])
        .spawn()
        .expect("Failed to execute child command");
    return child.wait().expect("Failed to wait on child command");
}

fn get_image_path(image_name: String, image_dir: String) -> String {
    let image_suffix = "tar".to_owned();
    let image = format!("{}.{}", image_name, image_suffix);
    Path::new(&image_dir)
        .join(image)
        .to_str()
        .unwrap()
        .to_owned()
}

fn create_image_root(image_name: String, image_dir: String) -> String {
    let root_dir = format!("{}.root.d", image_name);
    let image_path = get_image_path(image_name.clone(), image_dir.clone());
    let image_root = Path::new(&image_dir)
        .join(root_dir)
        .to_str()
        .unwrap()
        .to_owned();
    if !Path::new(&image_root).exists() {
        create_dir_all(&image_root).unwrap();
        util::untar(image_path, image_root.clone());
    }
    return image_root;
}

fn get_container_path(container_id: String, container_dir: String, subdir_name: String) -> String {
    Path::new(&container_dir)
        .join(container_id)
        .join(subdir_name)
        .to_str()
        .unwrap()
        .to_owned()
}

fn create_container_rootfs(
    container_id: String,
    container_dir: String,
    image_name: String,
    image_dir: String,
) -> String {
    // Create image root
    // Image root is "lowerdir" (read-only)
    let image_root = create_image_root(image_name.clone(), image_dir.clone());

    // Create directories for copy-on-write ("upperdir"), overlay workdir and a mount point
    let container_cow_rw = get_container_path(
        container_id.clone(),
        container_dir.clone(),
        "cow_rw".to_owned(),
    );
    let container_cow_workdir = get_container_path(
        container_id.clone(),
        container_dir.clone(),
        "cow_workdir".to_owned(),
    );
    let container_rootfs = get_container_path(
        container_id.clone(),
        container_dir.clone(),
        "rootfs".to_owned(),
    );

    // Create directories for copy-on-write (upperdir), overlay workdir,
    // and a mount point
    for dir in vec![
        container_cow_rw.clone(),
        container_cow_workdir.clone(),
        container_rootfs.clone(),
    ] {
        let dir_path = Path::new(&dir);
        if !dir_path.exists() {
            create_dir_all(dir_path).expect("Failed to create directory");
        }
    }

    // Mount the overlay
    let overlay_paths = format!(
        "lowerdir={},upperdir={},workdir={}",
        image_root, container_cow_rw, container_cow_workdir,
    );
    let overlay_paths: Option<&str> = Some(&overlay_paths);
    mount(
        Some("overlay"),
        Path::new(&container_rootfs),
        Some("overlay"),
        MsFlags::MS_NODEV, // Disallow access to device special files
        overlay_paths,
    )
    .expect(&format!(
        "Failed to create mount from /tmpfs to {}",
        &container_rootfs
    ));

    return container_rootfs;
}

fn create_mounts(container_rootfs: String) {
    let root = Path::new(&container_rootfs);
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

fn add_devices(container_rootfs: String) {
    // Add basic devices
    let dev_path = Path::new(&container_rootfs).join("dev");
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
        let dest = Path::new(&container_rootfs).join("dev").join(device);
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
        } else {
            println!("Device exists, skipping: {:?}", device_path)
        }
    }
}
