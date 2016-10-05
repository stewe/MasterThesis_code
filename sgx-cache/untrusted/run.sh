#!/bin/bash
trap 'kill "$cache_pid" 2>/dev/null' SIGTERM # SIGINT (ctrl+c) SIGKILL (kill pid)

LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu/

scriptdir=`dirname "$BASH_SOURCE"`
cd $scriptdir || exit 

enclave='../trusted/target/debug/trusted.sgxs'
sig='../trusted/target/debug/trusted.sig'
le='../sgx-utils-files/le.sgxs'
le_prod='../sgx-utils-files/le_prod_css.bin'
if [ ! -e $enclave ]; then echo "Enclave doesn't exist!"; exit ; fi
if [ ! -e $sig ]; then echo "Signature doesn't exist!"; exit ; fi
if [ ! -e $le ]; then echo "The file le.sgxs doesn't exist!"; exit ; fi
if [ ! -e $le_prod ]; then echo "The file le_prod_css.bin doesn't exist!"; exit; fi

cargo build --release  || exit
./target/release/sgx-first-steps-untrusted $enclave $sig $le $le_prod $@ &
cache_pid=$!
wait "$cache_pid"
exit 0
