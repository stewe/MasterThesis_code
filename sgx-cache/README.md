Necessary steps:

git clone https://github.com/jethrogb/rust-core_collections

rust-core_collections/src/fdeda33a9a7326ac4979aee5b0c9657298aebe59/hash/map.rs line 1618: 

pub fn new() -> RandomState {
        //let mut r = rand::thread_rng();
        let mut r = rand::XorShiftRng::new_unseeded();
        RandomState { k0: r.gen(), k1: r.gen() }
    }

Cargo.toml:

[replace]
"rand:0.3.14" = {git = "https://github.com/jethrogb/rand.git", branch = "core", features = ["core_io", "box", "vec"], default-features = false }

[dependencies]
rand = { version = "0.3", default-features = false, features = ["box", "vec"], optional = true }
