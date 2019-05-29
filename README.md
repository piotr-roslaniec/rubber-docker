# rubber-docker

This repo contains [Docker From Scratch Workshop] implemented in Rust.

## Installation

Make sure you have Vagrant installed.
We use `vagrant-vbguest` to automatically provision Virtualbox Guest Additions to new VMs on Vagrant.

```bash
vagrant plugin install vagrant-vbguest
# If you have issues with conflicting deps in bundler (bigdecimal-1.3.0 vs bigdecimal-1.3.2), try:
# VAGRANT_DISABLE_STRICT_DEPENDENCY_ENFORCEMENT=1 vagrant plugin install vagrant-vbguest
```

## Development

Make images, create Vagrant VM and enter it:

```bash
./vagrant_remake.sh
```

If VM already exists:

```bash
vagrant up
vagrant ssh
```

Once inside VM, run `rubber-docker` with some default settings:

```bash
./rdocker.sh
```
