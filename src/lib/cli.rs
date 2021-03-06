use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(
        long = "image-dir",
        default_value = "/tmp/rdocker/images",
        help = "Directory to store unpacked images"
    )]
    pub image_dir: String,

    #[structopt(
        long = "container-dir",
        default_value = "/tmp/rdocker/containers",
        help = "Directory to store containers"
    )]
    pub container_dir: String,

    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    #[structopt(name = "run", help = "Run a container")]
    Run {
        #[structopt(long = "image-name", help = "Name of image to be used")]
        image_name: String,

        #[structopt(long = "command", help = "Command to be executed")]
        command: Vec<String>,

        #[structopt(
            long = "memory",
            default_value = "1G",
            help = "Memory limit in bytes. Use suffixes to represent units (k, m, g)"
        )]
        memory: String,

        #[structopt(
            long = "memory-swap",
            default_value = "-1",
            help = "A positive integer equal to memory plus swap. Specify -1 to enable unlimited swap"
        )]
        memory_swap: i32,

        #[structopt(
            long = "cpu-shares",
            default_value = "0",
            help = "CPU shares (relative weight)"
        )]
        cpu_shares: i32,

        #[structopt(long = "uid", default_value = "0", help = "User id")]
        uid: u32,

        #[structopt(long = "gid", default_value = "0", help = "Group id")]
        gid: u32,
    },
}
