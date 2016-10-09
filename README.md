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
Ensure [ZeroMQ](http://zeromq.org/intro:get-the-software) is installed on your system.

### Rust
You need nightly Rust. The caching service was developed and tested with version 1.11.0-nightly (801d2682d 2016-07-06). You can install it in the following way:

```bash
curl -sfO https://static.rust-lang.org/rustup.sh
chmod +c rustup.sh
sudo ./rustup.sh --channel=nightly --date=2016-07-07
```

If you use another version, you need to modify *rust-core_collections*. In the file *src/your_version/hash/map.rs* change the Random Number Generator in the function `RandomState::new` to

```Rust
impl RandomState {
    /// Constructs a new `RandomState` that is initialized with random keys.
    #[inline]
    #[allow(deprecated)] // rand
    pub fn new() -> RandomState {
        let mut r = rand::XorShiftRng::new_unseeded();
        RandomState { k0: r.gen(), k1: r.gen() }
    }
}
```


## How to build the cache-enclave



## How to start the cache with enclave



## How to start the experiments
Execute *experiments.sh*. Unless at least one of the parameters *latnum*, *latsize* *tpnum* and *tpsize* are passed, all four measurements are processed. The following optional parameters are available:

* **sgx** Runs the experiments with the cache inside an SGX enclave (default without).
* **json** Runs the experiments with JSON instead of the default format Protocol Buffers.
* **debug** Activates verbose logging and enables debugging features for the enclave.
* **latnum** Activates the measurements of latency over of the number of requested values.
* **latsize** Activates the measurements of latency over of the size of the requested values.
* **tpnum** Activates the measurements of throughput over of the number of requested values.
* **tpsize** Activates the measurements of throughput over of the size of the requested values.
* **threadeval** Starts a routine helping to examine the necessary number of requester threads for throughput measurements.



rust-core_collections/src/fdeda33a9a7326ac4979aee5b0c9657298aebe59/hash/map.rs line 1618:

pub fn new() -> RandomState { //let mut r = rand::thread_rng(); let mut r = rand::XorShiftRng::new_unseeded(); RandomState { k0: r.gen(), k1: r.gen() } }
