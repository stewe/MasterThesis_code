extern crate zmq;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle, sleep_ms};
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

impl<'a> Clone for Sensor<'a> {
    fn clone(&self) -> Sensor<'a> {
        Sensor {
            id: self.id,
            addr: self.addr,
            filters: self.filters.clone(),
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

fn init_sensor_socket<'a, 'b>(sensor: &'a Sensor<'a>, ctx: &'b mut Context) -> Socket {
        let res: Result<Socket, zmq::Error> = ctx.socket(zmq::SUB);
        let mut socket: Socket = res.unwrap();
        for filter in &sensor.filters {
            assert!(socket.set_subscribe(filter.as_bytes()).is_ok());
        }
        match socket.connect(sensor.addr) {
          Ok(()) => (),
          Err(e) => panic!(e) // TODO panic or tolerate???
        }
    socket
}

// ------------------------------------------------------------------------------------
fn main() {

    const EXIT: &'static str = "exit";

    println!("Cache started.");

    let sensors_info = init_sensors_info();
    // let mut ctx = Context::new();
    // let mut sensor_sockets = init_sensor_sockets(sensors_info, &mut ctx);

    let mut threads = vec![];

    // let data = Arc::new(Mutex::new(&sensor_sockets));
    // let data = data.clone();

    let (msg_tx, msg_rx) = channel();
    let mut command_txs: HashMap<&str, Sender<&str>> = HashMap::new();

    for sensor in sensors_info.iter(){
        let sensor: Sensor = sensor.clone();
        let id = sensor.id.clone(); // TODO necessary?

        let msg_tx = msg_tx.clone();
        let (command_tx, command_rx): (Sender<&str>, Receiver<&str>) = channel();
            command_txs.insert(id, command_tx);

        let handle = thread::spawn(move || {
            let mut ctx = Context::new();
            let mut socket = init_sensor_socket(&sensor, &mut ctx);

    //
    //         // println!("received {}", msg);
    //         // let id = data.lock();//.unwrap().len();
            println!("Start thread and listen on socket id: {}", id);
    //         let socket: &Socket = sensor_rx.recv().unwrap();
    //
    //         // loop {
            for _ in 1..1000 {
                // listen for commands ('exit')
                match command_rx.try_recv() {
                    Ok(m) => {match m {
                                EXIT => {println!("Closing {}", id); panic!("Thread {} shutdown.", id);},
                                _ => println!("sensor {} received {}", id, m)
                            }},
                    Err(..) => {}
                }

                let msg = (&mut socket).recv_string(0).unwrap().unwrap();
                println!("from sensor {} received msg  {}", id, msg);

                msg_tx.send((id, msg)).unwrap();
                // TODO decide whether send to master thread and put there into queue or put into queue here - !!! init queue for each sensor
            }
        });
        threads.push(handle);
    }

    let mut i = 0;
    loop {
        let (_id, _msg) = msg_rx.recv().unwrap();
        println!("parent received id / msg: {}/{}", _id, _msg);
        if _id == "sensor-clj" {
            i += 1;
            if i==3 {
                match command_txs.remove("sensor-clj") {
                    Some(tx) => {
                        // fails if channel is already dead (command_rx dropped or thread down)
                        match tx.send(EXIT) {
                            Ok(_) => println!("parent sent 'exit' to {}", _id),
                            Err(e) => println!("Failed to send 'exit' to {}; error: {}", _id, e),
                        }
                    },
                    None => println!("Couldn't find channel for {}", _id),
                }
            }
        }

    }
    for thread in threads {
        thread.join().unwrap();
    }

}


// let chks: Vec<i64> = msg.split(' ').map(|x| atoi(x)).collect();
// let (_zipcode, temperature, _relhumidity) = (chks[0], chks[1], chks[2]);

// init: subscribe to sensors
// receive messages in new threads
// listen to req socket and reply
// queues for messages
// multiple microservices? keep track of filters / sensors
