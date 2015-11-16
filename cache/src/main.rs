extern crate zmq;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;
use zmq::{Socket, Context};

// TODO remove when no longer needed
fn atoi(s: &str) -> i64 {
    s.parse().unwrap()
}

struct Sensor<'a> {
    id: &'a str,
    addr: &'a str,
    filters: Vec<String>,

}

impl<'a> Sensor<'a> {
    fn new(id: &'a str, addr: &'a str, filters: Vec<String>) -> Sensor<'a> {
        Sensor {
            id: id,
            addr: addr,
            filters: filters,
        }
    }
}

fn init_sensors_info<'a>() -> Vec<Sensor<'a>> {
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

// currently fails if cannot connect to one of the sensors
fn init_sensors<'a>(sensors: Vec<Sensor<'a>>, ctx: &'a mut Context) -> HashMap<&'a str, Socket> {
    let mut sockets: HashMap<&'a str, Socket> = HashMap::new();
    for sensor in sensors {
        let res: Result<Socket, zmq::Error> = ctx.socket(zmq::SUB);
        let mut socket: Socket = res.unwrap();
        for filter in sensor.filters {
            assert!(socket.set_subscribe(filter.as_bytes()).is_ok());
        }
        match socket.connect(sensor.addr) {
          Ok(()) => (),
          Err(e) => panic!(e) // TODO panic or tolerate???
        }
        sockets.insert(sensor.id, socket);
    }
    println!("Initialized {} sockets to subscribeers.", sockets.len());
    sockets
}

// ----------------------------
fn main() {
    println!("Cache started.");

    let sensors_info = init_sensors_info();
    println!("{} sensor used.", &sensors_info.len());
    let mut ctx = Context::new();
    let mut sensor_sockets = init_sensors(sensors_info, &mut ctx);
    println!("{} sockets used.", &sensor_sockets.len());

    let mut threads = vec![];

    // let data = Arc::new(Mutex::new(&sensor_sockets));
    // let data = data.clone();

    // TODO try using channels!


    let (tx, rx) = channel();

    for (id, sensor) in sensor_sockets.iter_mut(){
        let handle = thread::spawn(move || {
            let msg = rx.recv().unwrap();
            println!("received {}", msg);
            // let id = data.lock();//.unwrap().len();
            // println!("id: {}", id);
            // println!("Start thread for sensor {}.", id);

    //         // loop {
    //         //         let msg = socket.recv_string(0).unwrap().unwrap();
    //         //         let chks: Vec<i64> = msg.split(' ').map(|x| atoi(x)).collect();
    //         //         // match id
    //         //         let (_zipcode, temperature, _relhumidity) = (chks[0], chks[1], chks[2]);
    //         //         println!("from sensor {} received zipcode {}, temperature {}, relhumidity {}.", id, _zipcode, temperature, _relhumidity);
                }
        );
        threads.push(handle);
        tx.send(10).unwrap();
    }

    for thread in threads {
        // thread.join();
    }

}



// init: subscribe to sensors
// receive messages in new threads
// listen to req socket and reply
// queues for messages
// multiple microservices? keep track of filters / sensors
