// this file is actulally not needed since the framework will be used as a library.
// Thus the main file can be used for development and testing purposes.

extern crate framework;

// use

fn main() {
    println!("Hello World!!11einseinself");
    println!("... nothing to do :/");
    framework::sub("endpoint", &["topic", "filter"]);

    framework::req("tcp://...:1234", &['h', 'e', 'l', 'l', 'o']);
}
