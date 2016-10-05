#!/bin/bash
path="$(pwd)/`dirname '$BASH_SOURCE'`"
echo $path
cd $path
if [ ! -e "target/debug/private.pem" ]; then
    mkdir -p target && mkdir -p target/debug
    cp private.pem target/debug/
fi

cargo build-enclave --verbose -H 0x1000000 -S 0x10000 -- --features "enclave_debug" && cd ./target/debug && sgxs-sign --key private.pem -d trusted.sgxs trusted.sig
# cargo build-enclave --verbose -H 0x100000 -S 0x10000 -- --features "json enclave_debug" && cd ./target/debug && sgxs-sign --key private.pem -d trusted.sgxs trusted.sig
# cargo build-enclave --verbose -H 0x100000 -S 0x10000  && cd ./target/debug && sgxs-sign --key private.pem -d trusted.sgxs trusted.sig

exit 0
