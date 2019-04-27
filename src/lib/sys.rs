use libc;
use nix::{NixPath, Result};
use nix::errno::Errno;

pub fn pivot_root<P1: ?Sized + NixPath, P2: ?Sized + NixPath>(
    new_root: &P1, put_old: &P2) -> Result<()> {
    let res = new_root.with_nix_path(|new_root| {
        put_old.with_nix_path(|put_old| {
            unsafe {
                libc::syscall(libc::SYS_pivot_root, new_root.as_ptr(), put_old.as_ptr())
            }
        })
    })??;

    Errno::result(res).map(drop)
}
