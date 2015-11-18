extern crate zmq;

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
// use std::thread::{JoinHandle, sleep_ms};
use zmq::{Socket, Context};

const EXIT: &'static str = "exit";

// TODO remove when no longer needed
// fn atoi(s: &str) -> i64 {
//     s.parse().unwrap()
// }

#[derive(Clone)]
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

// impl<'a> Clone for Sensor<'a> {
//     fn clone(&self) -> Sensor<'a> {
//         Sensor {
//             id: self.id,
//             addr: self.addr,
//             filters: self.filters.clone(),
//         }
//     }
// }

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

/// returns cached messages: index 0 is newest, len()-1 is oldest
fn get_cached_msgs(n: usize, q: &Arc<Mutex<VecDeque<String>>>) -> Vec<String> {
    let mut cached_msgs = Vec::with_capacity(n);
    match q.lock() {
        Ok(qu) =>  {
            println!("size: {}", qu.len());
            println!("TEST TEST TEST0: {}", qu.get(0).unwrap());
            for i in 0..n {
                match qu.get(i) {
                    // Some(s) => cached_msgs.insert(n-i-1, s.clone()),
                    Some(s) => cached_msgs.push(s.clone()),
                    None => {},
                }
            }
        },
        Err(_) => println!("Couldn't access lock."), // TODO do sth. useful?
    }
    cached_msgs
}

fn remove_sensor(id: &str, command_txs: &mut HashMap<&str, Sender<&str>>) {
    match command_txs.remove(id) {
        Some(tx) => {
            // fails if channel is already dead (command_rx dropped or thread down)
            match tx.send(EXIT) {
                Ok(_) => println!("parent sent 'exit' to {}", id),
                Err(e) => println!("Failed to send 'exit' to {}; error: {}", id, e),
            }
        },
        None => println!("Couldn't find channel for {}", id),
    }
}

// ------------------------------------------------------------------------------------
fn main() {
    println!("Cache started.");

    let sensors_info = init_sensors_info();
    let mut threads = vec![];
    let (msg_tx, msg_rx) = channel();
    let mut command_txs: HashMap<&str, Sender<&str>> = HashMap::new();
    let mut queues = HashMap::new();

    for sensor in sensors_info.iter(){
        let sensor: Sensor = sensor.clone();
        let id = sensor.id;

        let msg_tx = msg_tx.clone();
        let (command_tx, command_rx): (Sender<&str>, Receiver<&str>) = channel();
        command_txs.insert(id, command_tx);

        let msg_queue: VecDeque<String> = VecDeque::new();
        queues.insert(id, Arc::new(Mutex::new(msg_queue)));
        let q = queues.get(id).unwrap();
        let q = q.clone();

        let handle = thread::spawn(move || {
            let mut ctx = Context::new();
            let mut socket = init_sensor_socket(&sensor, &mut ctx);
            println!("Start thread and listen on socket id: {}", id);

            loop {
                // listen for commands ('exit')
                match command_rx.try_recv() {
                    Ok(m) => {match m {
                                EXIT => panic!("Thread {} shutdown.", id),
                                _ => println!("sensor {} received {}", id, m)
                            }},
                    Err(..) => {}
                }

                // listen for messages from sensor
                // TODO IMPORTANT!!! think about how to queue and to get_cached_msgs! sensors - filters - queues 
                let msg = (&mut socket).recv_string(0).unwrap().unwrap(); // TODO adapt to message format (e.g. google protocol buffers); handle in an approriate way!
                println!("from sensor {} received msg  {}", id, msg);
                {
                    let mut q = q.lock().unwrap();
                    q.push_front(msg.clone()); // TODO remove clone() when channel is removed
                    // TODO control size of queue!
                }
                msg_tx.send((id, msg)).unwrap(); // TODO remove, since we now have a queue
            }
        });
        threads.push(handle);
    }

    // thread for requests
    // todo create channel and use it also to check whether thread is still alive!
    thread::spawn(|| {
        let mut ctx = Context::new();
        let mut socket: Socket = ctx.socket(zmq::REP).unwrap();
        socket.bind("tcp://*:5550").unwrap();

        // example, adapt!
        let mut msg = zmq::Message::new().unwrap();
        loop {
            socket.recv(&mut msg, 0).unwrap();
            println!("Received request {}", msg.as_str().unwrap());
            // parse and execute
            socket.send_str("World", 0).unwrap();
        }
    });

    let (mut i, mut j) = (0, 0);
    loop {
        // TODO adapt to queue - no receiving of channel messages anymore! ... keep channel for signals?
        let (_id, _msg) = msg_rx.recv().unwrap();
        // println!("parent received id / msg: {}/{}", _id, _msg);

        if j==6 {
            println!("DEBUG DEBUG DEBUG DEBUG {}", _id);
            let q = queues.get(_id).unwrap();
            let cached_msgs = get_cached_msgs(j, q);
            println!("cached messages: {:?}", cached_msgs);
        }
        if _id == "sensor-clj" {
            i += 1;
            if i==3 {
                remove_sensor("sensor-clj", &mut command_txs);
            }
        } else {
            j += 1;
        }

    }
    // for thread in threads {
    //     thread.join().unwrap();
    // }

}


// let chks: Vec<i64> = msg.split(' ').map(|x| atoi(x)).collect();
// let (_zipcode, temperature, _relhumidity) = (chks[0], chks[1], chks[2]);

// init: subscribe to sensors
// receive messages in new threads
// listen to req socket and reply
// queues for messages
    // idea: ringbuffer (maybe with extrasize) with get(number) that does not remove elements
    // -> spsc, conflicts with head element?
// multiple microservices? keep track of filters / sensors
