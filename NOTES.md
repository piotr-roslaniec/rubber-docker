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
