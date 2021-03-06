#!/bin/bash
scriptdir=`dirname "$BASH_SOURCE"`
cd $scriptdir

debug=n
heap=0x1000000	# enclave heap size in Byte (hex)
stack=0x10000	# enclave stack size in Byte (hex)

for var in "$@"
  do
    case "$var" in
     "debug") debug=y
      ;;
      *) echo "invalid argument: $var"
        exit 22
      ;;
    esac
done

name="sgx_pubsub_cache_enclave"

if [ $debug = "y" ]; then
    if [ ! -e "target/debug/private.pem" ]; then
        mkdir -p target && mkdir -p target/debug
        cp private.pem target/debug/
        cargo build-enclave --verbose -H $heap -S $stack -- --features "enclave_debug" && cd ./target/debug && sgxs-sign --key private.pem -d $name.sgxs $name.sig
        # cargo build-enclave --verbose -H 0x100000 -S 0x10000 -- --features "json enclave_debug" && cd ./target/debug && sgxs-sign --key private.pem -d $name.sgxs $name.sig
    fi
else
    if [ ! -e "target/release/private.pem" ]; then
        mkdir -p target && mkdir -p target/release
        cp private.pem target/release/
        cargo build-enclave -H $heap -S $stack -- --release && cd ./target/release && sgxs-sign --key private.pem -d $name.sgxs $name.sig
    fi
fi

exit 0
