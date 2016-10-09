# A Caching Service For The Enclave


## Requirements
### SGX
Ensure your system supports Intel SGX and the drivers are installed. If you want to use the drivers of [*sgx-utils*](https://github.com/jethrogb/sgx-utils/tree/master/isgx), you can install them in the following way.

```bash
make -C /lib/modules/`uname -r`/build M=$PWD
make -C /lib/modules/`uname -r`/build M=$PWD modules_install
cd  /lib/modules/`name -r`/extra
sudo depmod -a
sudo modprobe intel_sgx
```

Test the success of the installation with `lsmod | grep sgx` and `ls /dev/sgx`.

### ZeroMQ

### Rust


## How to build the cache-enclave



## How to start the cache with enclave



## How to start the experiments
Execute *experiments.sh*, optionally with the following parameters:

* **sgx** Runs the experiments with the cache inside an SGX enclave (default without).
* **json** Runs the experiments with JSON instead of the default format Protocol Buffers.
* **debug** Activates verbose logging and enables debugging features for the enclave.
* **latnum** Activates the measurements of latency over of the number of requested values.
* **latsize** Activates the measurements of latency over of the size of the requested values.
* **tpnum** Activates the measurements of throughput over of the number of requested values.
* **tpsize** Activates the measurements of throughput over of the size of the requested values.
* **threadeval** Starts a routine helping to examine the necessary number of requester threads for throughput measurements.




Necessary steps:

git clone https://github.com/jethrogb/rust-core_collections

rust-core_collections/src/fdeda33a9a7326ac4979aee5b0c9657298aebe59/hash/map.rs line 1618:

pub fn new() -> RandomState { //let mut r = rand::thread_rng(); let mut r = rand::XorShiftRng::new_unseeded(); RandomState { k0: r.gen(), k1: r.gen() } }
