# -*- mode: ruby -*-
# vi: set ft=ruby :

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

  config.vm.synced_folder '.', '/home/vagrant/rubber-docker'

  config.vm.provider "virtualbox" do |vb|
    vb.memory = "2048"
    vb.cpus = "2"
  end

  config.vm.provision "shell", inline: <<-SHELL
    grep -q `hostname` /etc/hosts || echo 127.0.0.1 `hostname` |sudo tee -a /etc/hosts
  SHELL
end
