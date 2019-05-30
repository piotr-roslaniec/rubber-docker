# Notes

## Chroot & mount

[(Stackoverflow) chroot before pivot_root causes busy error](https://unix.stackexchange.com/questions/457040/chroot-before-pivot-root-causes-busy-error>)

`mount` & `pivot_root` example

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

[Part 4 - UTS Namespace](https://windsock.io/uts-namespace/>)
[The Curious Case of Pid Namespaces](https://hackernoon.com/the-curious-case-of-pid-namespaces-1ce86b6bc900)

## Networking

[Network namespaces](https://blogs.igalia.com/dpino/2016/04/10/network-namespaces/)
