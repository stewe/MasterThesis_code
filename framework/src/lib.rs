//! # Microservice framework for sensor subscription and cache requesting
//! some more explanation?

mod sensor;
mod cache;
mod manager;

mod messaging;

pub use manager::{sub};
pub use messaging::{req};



//#[test]
fn it_works() {
    assert!(true); // test fails

}
