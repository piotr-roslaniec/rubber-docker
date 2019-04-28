# Notes

<https://unix.stackexchange.com/questions/457040/chroot-before-pivot-root-causes-busy-error>

Mounting example

```bash
root@vagrant:~# unshare -m
root@vagrant:~# mount --bind / /mnt
root@vagrant:~# cd /mnt
root@vagrant:/mnt# mount --bind . mnt
root@vagrant:/mnt# cd mnt
root@vagrant:/mnt/mnt# mount --bind /proc proc
root@vagrant:/mnt/mnt# chroot /mnt
root@vagrant:/# cd /mnt
root@vagrant:/mnt# pivot_root . mnt
root@vagrant:/mnt#
```

## Namespaces

CLONE_NEWNS will create with a copy of the namespace of the parent.
CLONE_NEWUTS will create the process in a new UTS namespace, whose identifiers are initialized by duplicating the identifiers from the UTS namespace of the calling process.
CLONE_NEWPID will create the process in a new PID namespace.

<https://windsock.io/uts-namespace/>

<https://hackernoon.com/the-curious-case-of-pid-namespaces-1ce86b6bc900>