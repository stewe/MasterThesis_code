extern crate time;
extern crate zmq;
extern crate enclave_cache;

use std::collections::{HashMap};
use std::sync::Arc;
use std::sync::mpsc::{Sender};
use std::thread::JoinHandle;

use enclave_cache::*;
use zmq::{Socket, Context};

// ------------------------------------------------------------------------------------
fn main() {
    println!("Cache started.");

    // static MAX_AGE: i32 = 30000; // 30 seconds TODO init

    let mut cache = Cache::new();
    let mut sensor_threads: HashMap<String, (JoinHandle<()>, Sender<SensorThreadCmd>)> = HashMap::new(); // maybe HashMap

    // threads receive sensor messages
    for (id, sensor) in &cache.sensors {
        let sensor = sensor.clone();
        let queue = sensor.queue.clone();
        let (handle, tx) = sensor_msg_thread(sensor, queue);
        sensor_threads.insert(id.to_string(), (handle, tx));
    }

    // // Map<ep, Map<filter, Set<caller-id>>>: necessary for dynamic configuration
    // let mut filter_subs: HashMap<&str, HashMap<&str, HashSet<&str>>> = HashMap::new();
    //
    // // TODO create function for this
    // let mut a_subs: HashMap<&str, HashSet<&str>> = HashMap::new();
    // a_subs.insert("a", ["clj-requester", "another-service"].iter().cloned().collect());
    // filter_subs.insert("sensor-java", a_subs);
    // let mut one_subs: HashMap<&str, HashSet<&str>> = HashMap::new();
    // one_subs.insert("1", ["clj-requester"].iter().cloned().collect());
    // one_subs.insert("2", ["clj-requester"].iter().cloned().collect());
    // filter_subs.insert("sensor-clj", one_subs);
    // println!("filter_subs: {:?}", filter_subs);

    // TODO init map for managing filter/ep registration


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

    // main thread: receive requests / API handling
        let mut ctx = Context::new();
        let mut socket: Socket = ctx.socket(zmq::REP).unwrap();
        socket.bind("tcp://*:5550").unwrap();


        // TODO parse request approriately
        let mut msg = zmq::Message::new().unwrap();
        loop {
            socket.recv(&mut msg, 0).unwrap();
            let msg_str = msg.as_str().unwrap();
            println!("Received request {}", msg_str);
            let mut request = msg_str.split(' ');
            let op = request.next().unwrap();

            match op {
                "ADD" => {
                // TODO receive parameters by parsing
                // let sender ... -> track client <-> filter
                let id = request.next().unwrap();
                let addr = "tcp://127.0.0.1:5556";  // TODO how to receive the addr?!
                let filters = request.map(|x| x.to_string()).collect(); //vec!["1".to_string(), "2".to_string()];

                let is_active = sensor_threads.contains_key(id);
                if is_active {
                    // add filter subscription to already existing thread
                    let tx = &sensor_threads.get(id).unwrap().1;
                    let filters = vec!["0".to_string()]; // TODO remove, it's just used for developing and debugging
                    tx.send(SensorThreadCmd::new_add(filters)).unwrap();
                    println!("main thread send add to zmq thread."); // TODO DEBUG
                } else {
                    // start new thread for subscribing to sensor
                    let sensor = Arc::new(Sensor::new(&id, addr, filters));
                    cache.add_sensor(&id, sensor);
                    let sensor = cache.sensors.get(id).unwrap();
                    let queue = sensor.queue.clone();
                    sensor_threads.insert(id.to_string(), sensor_msg_thread(sensor.clone(), queue));
                }
            },
            _ => println!("Received unknown request.")
        }

            // parse and execute
            socket.send_str("World", 0).unwrap();

            // TODO used for debugging
            cache.print_msg_queues();
        }



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
