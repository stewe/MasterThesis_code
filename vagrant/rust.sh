#!/usr/bin/env bash

if ! [ -x /usr/local/bin/rustc ]
then
        apt-get update
        apt-get dist-upgrade -y
        apt-get install curl
        echo 'loading rustup.sh...'
        curl -sSf https://static.rust-lang.org/rustup.sh -O
        chmod +x rustup.sh
        echo 'executing rustup.sh ...'
        ./rustup.sh -y
        echo 'removing rustup.sh ...'
        rm rustup.sh
else
        echo 'rust is already installed!'
fi
