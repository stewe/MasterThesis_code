extern crate time;
extern crate zmq;
extern crate enclave_cache;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
// use std::thread::{JoinHandle, sleep};
use enclave_cache::*;
use time::{get_time, Timespec};
use zmq::{Socket, Context};

const EXIT: &'static str = "exit";

/// returns cached messages: index 0 is newest, len()-1 is oldest
fn get_cached_msgs<'a>(n: usize, q: &Arc<Mutex<VecDeque<(&'a str, &'a str, Timespec)>>>)
    -> Vec<(&'a str, &'a str, Timespec)> {
    let mut cached_msgs = Vec::with_capacity(n);
    match q.lock() {
        Ok(qu) =>  {
            println!("size: {}", qu.len());
            println!("TEST TEST TEST0: {:?}", qu.get(0).unwrap());
            // TODO adapt!!!
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

    static MAX_AGE: i32 = 30000; // 30 seconds TODO init

    let sensors_info = init_sensors_info();
    // Map<ep, Map<filter, Set<caller-id>>>: necessary for dynamic configuration
    let mut filter_subs: HashMap<&str, HashMap<&str, HashSet<&str>>> = HashMap::new();

    // TODO create function for this
    let mut a_subs: HashMap<&str, HashSet<&str>> = HashMap::new();
    a_subs.insert("a", ["clj-requester", "another-service"].iter().cloned().collect());
    filter_subs.insert("sensor-java", a_subs);
    let mut one_subs: HashMap<&str, HashSet<&str>> = HashMap::new();
    one_subs.insert("1", ["clj-requester"].iter().cloned().collect());
    one_subs.insert("2", ["clj-requester"].iter().cloned().collect());
    filter_subs.insert("sensor-clj", one_subs);
    println!("filter_subs: {:?}", filter_subs);

    // sensor-id, socket
    let mut socket_map: HashMap<String, Socket> = HashMap::new();

    // init msg queues --> buggy! queues needs to be locked => coarse grained locking
    // sensor-id/data-id, msg, time
    // let mut queues: HashMap<String, Arc<Mutex<VecDeque<(String, Timespec)>>>> = HashMap::new();
    // for sensor in sensors_info.iter() {
    //     // msg, time
    //     let q: VecDeque<(String, Timespec)> = VecDeque::new();    // TODO with_capacity(n);
    //     queues.insert(sensor.id.to_string(), Arc::new(Mutex::new(q)).clone());
    // }

    // init msg queues  => fine grained locking
    let mut qs: Vec<Arc<Mutex<VecDeque<(String, Timespec)>>>> = vec![];
    // sensor-id/data-id, msg, time
    let queues: Arc<Mutex<HashMap<String, &Arc<Mutex<VecDeque<(String, Timespec)>>>>>> = Arc::new(Mutex::new(HashMap::new()));

    {
        let mut queues = queues.lock().unwrap();
        for _ in 0..sensors_info.len() {
            let q: Arc<Mutex<VecDeque<(String, Timespec)>>> = Arc::new(Mutex::new(VecDeque::new()));
            qs.push(q);
        }
        let mut i = 0;
        for s in sensors_info.iter() {
            queues.insert(s.id.to_string(), &qs[i]);
            i += 1;
        }
    }

    {
        let map = queues.lock().unwrap();
        let java_q = map.get(&"sensor-clj".to_string()).unwrap();
        let java_q = java_q.lock().unwrap();
        for x in java_q.iter() {
            print_tuple("sensor-clj".to_string(), x.clone());
        }
    }

    // TODO init map for managing filter/ep registration


// ----- new try!
    // thread receiving sensor messages
    let queues_one = queues.clone();
    let handle =  thread::spawn(move || {
        // init sockets for sensor subscription
        let mut ctx = Context::new();
        for sensor in sensors_info.iter() {
            let socket = init_sensor_socket(&sensor, &mut ctx);
            socket_map.insert(sensor.id.to_string(), socket);
        }

        loop {
            // read messages from sensors
            let msgs = read_sensor_msgs(&mut socket_map);

            if !msgs.is_empty() {
                // store msgs into queues
                for (id, (msg, time)) in msgs {
                    let queues_one = queues_one.lock().unwrap();
                    // let q = queues_one.get(&id).unwrap();
                    // cache_sensor_msg(id, msg, time, q);
                }
            }
        }
    });

    // print_queues(&queues);
// -----

// let mut threads = vec![];
// // let (msg_tx, msg_rx) = channel();
// let mut command_txs: HashMap<&str, Sender<&str>> = HashMap::new();
// let mut queues = HashMap::new();


    // for sensor in sensors_info.iter(){
    //     let sensor: Sensor = sensor.clone();
    //     let id = sensor.id;
    //
    //     // let msg_tx = msg_tx.clone();
    //     let (command_tx, command_rx): (Sender<&str>, Receiver<&str>) = channel();
    //     command_txs.insert(id, command_tx);
    //
    //     let msg_queue: VecDeque<(&str, &str, Timespec)> = VecDeque::new();
    //     queues.insert(id, Arc::new(Mutex::new(msg_queue)));
    //     let q = queues.get(id).unwrap();
    //     let q = q.clone();  // TODO really???
    //
    //     let handle = thread::spawn(move || {
    //         // TODO try to use only one context, eg by creating sockets first, then passing it to the thread
    //         // mabe compare https://github.com/erickt/rust-zmq/blob/master/examples/msgsend/main.rs
    //         let mut ctx = Context::new();
    //         let mut socket = init_sensor_socket(&sensor, &mut ctx);
    //         println!("Start thread and listen on socket id: {}", id);
    //
    //         loop {
    //             // listen for commands ('exit') TODO: add, remove filter
    //             match command_rx.try_recv() {
    //                 Ok(m) => {match m {
    //                             EXIT => panic!("Thread {} shutdown.", id),
    //                             _ => println!("sensor {} received {}", id, m)
    //                         }},
    //                 Err(..) => {}
    //             }
    //
    //             // listen for messages from sensor
    //             // TODO IMPORTANT!!! think about how to queue and to get_cached_msgs! sensors - filters - queues
    //             let msg = (&mut socket).recv_string(0).unwrap().unwrap(); // TODO adapt to message format (e.g. google protocol buffers); handle in an approriate way!
    //             let time: Timespec = get_time();
    //             let filter = msg.split_whitespace().nth(0).unwrap();
    //             println!("from sensor {} received msg  {} at {:?} with filter {}", id, msg, time, filter);
    //             {
    //                 let mut q = q.lock().unwrap();
    //                 // q.push_front(("filter", msg.as_str().clone(), time)); // TODO remove clone() when channel is removed
    //                 // TODO control size of queue!
    //             }
    //             // msg_tx.send((id, msg)).unwrap(); // TODO remove, since we now have a queue
    //         }
    //     });
    //     threads.push(handle);
    // }

    // thread serving requests
    // TODO create channel and use it also to check whether thread is still alive!
    let queues_two = queues.clone();
    thread::spawn(move || {
        let mut ctx = Context::new();
        let mut socket: Socket = ctx.socket(zmq::REP).unwrap();
        socket.bind("tcp://*:5550").unwrap();

        // example, adapt!
        let mut msg = zmq::Message::new().unwrap();
        loop {
            socket.recv(&mut msg, 0).unwrap();
            let msg_str = msg.as_str().unwrap();
            println!("Received request {}", msg_str);
            // if msg_str.contains("remove")
            // parse and execute
            socket.send_str("World", 0).unwrap();

            // TODO used for debugging
            // print_queues(&queues);
        }
    });

    // let (mut i, mut j) = (0, 0);
    loop {
        println!("reading queue for sensor-clj!");
        {
            let map = queues.lock().unwrap();
            let java_q = map.get(&"sensor-clj".to_string()).unwrap();
            let java_q = java_q.lock().unwrap();
            for x in java_q.iter() {
                print_tuple("sensor-clj".to_string(), x.clone());
            }
        }
        thread::sleep(Duration::from_secs(10));

        // TODO adapt to queue - no receiving of channel messages anymore! ... keep channel for signals?
        // let (_id, _msg) = msg_rx.recv().unwrap();
        // println!("parent received id / msg: {}/{}", _id, _msg);

        // if j==6 {
        //     println!("DEBUG DEBUG DEBUG DEBUG {}", _id);
        //     let q = queues.get(_id).unwrap();
        //     let cached_msgs = get_cached_msgs(j, q);
        //     println!("cached messages: {:?}", cached_msgs);
        // }
        // if _id == "sensor-clj" {
        //     i += 1;
        //     if i==3 {
        //         remove_sensor("sensor-clj", &mut command_txs);
        //     }
        // } else {
        //     j += 1;
        // }

    }
        handle.join().unwrap();

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
