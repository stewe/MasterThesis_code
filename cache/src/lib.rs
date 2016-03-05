extern crate time;
extern crate zmq;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use time::{get_time, Timespec};
use zmq::{Socket, Context, DONTWAIT};


#[derive(Clone)]
pub struct Sensor<'a> {
    pub id: &'a str,
    pub addr: &'a str,
    pub filters: Vec<String>,

}

impl<'a> Sensor<'a> {
    pub fn new(id: &'a str, addr: &'a str, filters: Vec<String>) -> Sensor<'a> {
        Sensor {
            id: id,
            addr: addr,
            filters: filters,
        }
    }
}

// TODO read and parse config file
pub fn init_sensors_info<'a>() -> Vec<Sensor<'a>> {
    // TODO read congig file and get sensors
    let sensor_clj = Sensor::new("sensor-clj",
                                    "tcp://127.0.0.1:5556",
                                    vec!{"1".to_string(), "2".to_string()});
    let sensor_java = Sensor::new("sensor-java",
                                    "tcp://127.0.0.1:5555",
                                    vec!{"a".to_string()});
    let sensors = vec!{sensor_clj, sensor_java};
    println!("Initialized information for {} sensors.", sensors.len());
    sensors
}

pub fn init_sensor_socket<'a, 'b>(sensor: &'a Sensor<'a>, ctx: &'b mut Context) -> Socket {
        let mut socket: Socket = ctx.socket(zmq::SUB).unwrap();
        for filter in &sensor.filters {
            assert!(socket.set_subscribe(filter.as_bytes()).is_ok());
        }
        match socket.connect(sensor.addr) {
          Ok(()) => (),
          Err(e) => panic!(e) // TODO panic or tolerate???
        }
    socket
}

pub fn read_sensor_msgs(sockets: &mut HashMap<String, Socket>) -> HashMap<String, (String, Timespec)> {
    let mut result = HashMap::new();
    for (id, socket) in sockets.iter_mut() {
        match socket.recv_string(DONTWAIT) { // TODO adapt to message format (e.g. google protocol buffers); handle in an approriate way!
            Ok(msg) =>  { let time: Timespec = get_time();
                match result.insert(id.clone(), (msg.unwrap(), time)) { Some(_) => { }, None => { } } },
            Err(_)  => { }
        }
    }
    // let filter = msg.split_whitespace().nth(0).unwrap();
    result
}

pub fn cache_sensor_msg(id: String, msg: String, time: Timespec, q: &Arc<Mutex<VecDeque<(String, Timespec)>>>) {
    println!("caching {} for {}.", msg, id);
    let mut q = q.lock().unwrap();
    q.push_front((msg, time));
}

pub fn print_tuple(id: String, t: (String, Timespec)) {
    let (msg, time) = t;
    print!("id: {}; msg: {}, time: {}.{}", id, msg, time.sec, time.nsec);
}

pub fn print_queues(queues: &HashMap<String, Arc<Mutex<VecDeque<(String, Timespec)>>>>) {
    for (id, q) in queues.iter() {
        let q = q.lock().unwrap();
        let size = q.len();
        print!("{} contains: ", id);
        for i in 0..size {
            let a = q.get(i).unwrap();
            print_tuple(id.clone(), a.clone());
        }
        println!("\n");
    }
}
