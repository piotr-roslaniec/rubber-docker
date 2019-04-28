use std::char::from_digit;
use std::fs::{create_dir_all, remove_dir_all};
use std::os::unix::fs::symlink;
use std::path::Path;
use std::string::String;

use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::sched::{unshare, CloneFlags};
use nix::sys::stat::{makedev, mknod, Mode, SFlag};
use nix::unistd::{chdir, chroot, pivot_root, sethostname};

use crate::lib::cli;
use crate::lib::util;

#[derive(Debug)]
pub struct Container<'a> {
    image_name: String,
    image_dir: String,
    container_dir: String,
    command: Vec<&'a str>,
    container_id: String,
}

impl<'a> Container<'a> {
    pub fn new(args: cli::Arguments) -> Container {
        Container {
            image_name: args.image_name,
            image_dir: args.image_dir,
            container_dir: args.container_dir,
            command: args.command,
            container_id: util::uuid(),
        }
    }

    pub fn contain(&self) {
        println!("Unshare namespace");
        unshare_namespace();

        println!("Set hostname");
        set_hostname(self.container_id.clone());

        println!("Mount bind root");
        mount_bind_root();

        println!("Create container root fs");
        let container_rootfs = create_container_rootfs(
            self.container_id.clone(),
            self.container_dir.clone(),
            self.image_name.clone(),
            self.image_dir.clone(),
        );

        println!("Create mounts");
        create_mounts(container_rootfs.clone());

        println!("Change root fs with chroot");
        change_root(container_rootfs.clone());

        println!("Rearrange the mount namespace with pivot_root");
        pivot_root_fs(container_rootfs.clone());

        println!("Change directory to new root");
        change_dir("/");

        println!("Unmount old root fs");
        umount_old_fs();

        println!("Execute command");
        println!("{}", util::execute(self.command.clone()));
    }
}

fn unshare_namespace() {
    // Unshare the namespaces from the parent.pause program execution
    // CLONE_NEWNS will initialize child with new mount namespace
    // with a copy of the namespace of the parent.
    util::print_debug("Namespaces before", util::execute(vec!["lsns"]));
    unshare(CloneFlags::CLONE_NEWNS).expect("Failed to unshare");
    util::print_debug("Namespaces after", util::execute(vec!["lsns"]));
}

fn set_hostname(hostname: String) {
    // The UTS namespace allows per-container hostnames.
    // Set the hostname to be the container ID.
    util::print_debug("Hostname before", util::execute(vec!["hostname"]));
    sethostname(hostname).expect("Failed to set hostname");
    util::print_debug("Hostname after", util::execute(vec!["hostname"]));
}

fn mount_bind_root() {
    // Mount / as a private namespace to prevent host namespace pollution
    // Bind mount directories recursively
    let none: Option<&str> = None;
    let ns_flags = MsFlags::MS_PRIVATE | MsFlags::MS_REC;
    mount(none, Path::new("/"), none, ns_flags, none).expect("Failed to mount at new root");
}

fn change_root(container_rootfs: String) {
    util::print_debug("/dev before", util::execute(vec!["ls", "-lh", "/dev"]));
    chroot(Path::new(&container_rootfs.clone())).expect("Failed to chroot");
    util::print_debug("/dev after", util::execute(vec!["ls", "-lh", "/dev"]));
}

fn umount_old_fs() {
    // Perform a lazy unmount
    let old_root = Path::new("/old_root");
    umount2(old_root, MntFlags::MNT_DETACH).expect("Failed to unmount old root");
    remove_dir_all(old_root).expect("Failed to remove old root");
}

fn pivot_root_fs(new_root: String) {
    let old_root = Path::new(&new_root).join("old_root");
    create_dir_all(&old_root).expect("Failed to create old_root directory");
    pivot_root(Path::new(&new_root.clone()), old_root.to_str().unwrap())
        .expect("Failed to pivot_root");
}

fn change_dir(new_dir: &str) {
    util::print_debug("Current dir before", util::execute(vec!["pwd"]));
    chdir(Path::new(new_dir)).expect("Failed to chdir");
    util::print_debug("Current dir after", util::execute(vec!["pwd"]));
}

fn create_mounts(container_rootfs: String) {
    util::print_debug("Mounts before", util::execute(vec!["findmnt", "-l"]));

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
        "Failed to create mount to target {}",
        &proc_guest.to_str().unwrap()
    ));
    mount(Some("sysfs"), &sys_guest, Some("sysfs"), no_flags, no_data).expect(&format!(
        "Failed to create mount to target {}",
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
        "Failed to create mount to target {}",
        &dev_guest.to_str().unwrap()
    ));
    add_devices(container_rootfs.clone());
    util::print_debug("Mounts after", util::execute(vec!["findmnt", "-l"]));
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
    // Create container root filesystem as overlay CoW filesystem
    util::print_debug(
        "Container dir before:",
        util::execute(vec![
            "tree",
            "-L",
            "2",
            "-d",
            &format!("{}/{}", &container_dir, &container_id),
        ]),
    );
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
        "Failed to create mount to target {}",
        &container_rootfs
    ));
    util::print_debug(
        "Container dir after:",
        util::execute(vec![
            "tree",
            "-d",
            "-L",
            "2",
            "-d",
            &format!("{}/{}", &container_dir, &container_id),
        ]),
    );
    return container_rootfs;
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
        "Failed to create mount to target {}",
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
