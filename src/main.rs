use crate::lib::cli::{Opt, Subcommand};
use crate::lib::core::Container;
use structopt::StructOpt;
mod lib;

fn main() {
    let opt = Opt::from_args();
    match opt.subcommand {
        Some(Subcommand::Run {
            image_name,
            command,
            memory,
            memory_swap,
            cpu_shares,
            uid,
            gid,
        }) => {
            let container = Container::new(
                image_name,
                opt.image_dir,
                opt.container_dir,
                command,
                memory,
                memory_swap,
                cpu_shares,
                uid,
                gid,
            );
            container.run();
        }
        _ => (),
    }
}
