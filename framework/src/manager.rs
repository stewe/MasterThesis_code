use sensor::Sensor;
use cache::Cache;

pub struct manager {
    sensors: Vec<Sensor>,   // map from ep to sensor?
    cache: Cache,
}

pub fn sub (ep: &'static str, filter: &[&str]) {
    println!("ep: {}; filters: ", ep);
    for f in filter {
        println!("{}, ", f);
    }
}

// pub fn sub_max_age (ep: String, filter: &[&str], max_age: i32) {}
// pub fn sub_max_ages (ep: String, filter: &[String], max_ages: &[i32]) {}
// pub sub_and_add ( ep-sensor, ep-cache, filter[], max-age[] )
// pub fn unsub (ep: String, filter: &[String]) {}
