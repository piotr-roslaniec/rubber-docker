# rubber-docker, but in Rust

## Installation

We use `vagrant-vbguest` to automatically provision Virtualbox Guest Additions to new VMs on Vagrant

```bash
vagrant plugin install vagrant-vbguest
# If you have issues with conflicting deps in bundler (bigdecimal-1.3.0 vs bigdecimal-1.3.2), try:
# VAGRANT_DISABLE_STRICT_DEPENDENCY_ENFORCEMENT=1 vagrant plugin install vagrant-vbguest
```

## Development

Download Docker image(s)

```bash
./save_image.sh
```

Start Vagrant VM

```bash
cd vm
vagrant up
vagrant ssh
```

Run the rubber-docker

```bash
./run.sh
```
