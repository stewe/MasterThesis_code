#!/bin/bash
cd /home/stefan/playground/first-steps/trusted/
cargo build-enclave --verbose -H 0x1000000 -S 0x10000 -- --features "enclave_debug" && cd ./target/debug && sgxs-sign --key private.pem -d trusted.sgxs trusted.sig
# cargo build-enclave --verbose -H 0x100000 -S 0x10000 -- --features "json enclave_debug" && cd ./target/debug && sgxs-sign --key private.pem -d trusted.sgxs trusted.sig
# cargo build-enclave --verbose -H 0x100000 -S 0x10000  && cd ./target/debug && sgxs-sign --key private.pem -d trusted.sgxs trusted.sig

exit 0
