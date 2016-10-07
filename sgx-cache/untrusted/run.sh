#!/bin/bash
trap 'kill "$cache_pid" 2>/dev/null' SIGTERM # SIGTERM (kill pid)

LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu/

scriptdir=`dirname "$BASH_SOURCE"`
cd $scriptdir || exit 

enclave_name="sgx_pubsub_cache_enclave"
debug=n
build=n
le='../sgx-utils-files/le.sgxs'
le_prod='../sgx-utils-files/le_prod_css.bin'

for var in "$@"
  do
    case "$var" in
      "log=debug"|"debug") debug=y
      ;;
      "build") build=y
      ;;
      *)
      ;;
    esac
done

if [ $debug = "y" ]; then 
    echo "Running / Building enclave in debug mode." 
    enclave='../trusted/target/debug/'$enclave_name'.sgxs'
    sig='../trusted/target/debug/'$enclave_name'.sig'
    mode=debug
else
    enclave='../trusted/target/release/'$enclave_name'.sgxs'
    sig='../trusted/target/release/'$enclave_name'.sig'
    mode=release
fi

if [ $build = "y" ]; then
    if [ $mode = "debug" ]; then param=debug; fi
    if [ ! -e $enclave ]; then ../trusted/build.sh $param; fi
    cargo build --release
    exit
fi

if [ ! -e $enclave ]; then echo "Enclave doesn't exist!"; exit ; fi
if [ ! -e $sig ]; then echo "Signature doesn't exist!"; exit ; fi
if [ ! -e $le ]; then echo "The file le.sgxs doesn't exist!"; exit ; fi
if [ ! -e $le_prod ]; then echo "The file le_prod_css.bin doesn't exist!"; exit; fi

./target/release/sgx-pubsub-cache-untrusted-app $enclave $sig $le $le_prod $@ &
cache_pid=$!
wait "$cache_pid"
exit 0
