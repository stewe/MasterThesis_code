## Caching Service with Intel SGX
* trusted: enclave
* untrusted: application
* interface: ECALL identifier

## Modified crates
* core_rust-crypto
* core_rust-protobuf
* rand
* rustc-serialize

## HowTo
* Build the enclave with *trusted/build.sh*
* Start the caching service with *untrusted/run.sh*
