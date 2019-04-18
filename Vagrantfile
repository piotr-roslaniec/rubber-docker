# -*- mode: ruby -*-
# vi: set ft=ruby :

# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.

VM_USER = 'vagrant'
HOST_USER = "#{ENV['USERNAME'] || ENV['USER'] || `whoami | tr -d '\n'`}"

Vagrant.configure("2") do |config|

  # set auto_update to false, if you do NOT want to check the correct
  # additions version when booting this machine
  config.vbguest.auto_update = true

  # do NOT download the iso file from a webserver
  config.vbguest.no_remote = false

  config.vm.box = "bento/ubuntu-18.04"

  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
  SHELL

  config.vm.synced_folder '.', '/home/' + VM_USER + '/rubber-docker'

  config.vm.provider "virtualbox" do |vb|
    vb.memory = "2048"
    vb.cpus = 2
  end

  config.vm.provision "shell", inline: <<-SHELL
    grep -q `hostname` /etc/hosts || echo 127.0.0.1 `hostname` |sudo tee -a /etc/hosts
  SHELL
end
