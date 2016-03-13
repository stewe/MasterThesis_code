extern crate time;
extern crate zmq;
extern crate enclave_cache;

use std::collections::{HashMap};
// use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
// use std::thread::{JoinHandle, sleep};
use enclave_cache::*;
use zmq::{Socket, Context};

// ------------------------------------------------------------------------------------
fn main() {
    println!("Cache started.");

    // static MAX_AGE: i32 = 30000; // 30 seconds TODO init

    let cache = Cache::new();
    let mut handles = vec![]; // maybe HashMap

    // threads receive sensor messages
    for sensor in &cache.sensors {
        let sensor = sensor.clone();
        let queue = sensor.queue.clone();
        let handle = sensor_msg_thread(sensor, queue);
        handles.push(handle);
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

    // also: dieser thread liest die queues aus, je nach reques.
    // dazu muss er clones von den sensor.queues besitzen -> arc<hashmap<sensor-id, _arc_queue_clone>>?
    let queue_arcs = cache.sensors.iter()
                                    .fold(HashMap::new(), |mut acc, sensor|
                                    { acc.insert(sensor.id.clone(), sensor.queue.clone()); acc});

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
            print_queues(&queue_arcs);
        }
    });

    // let (mut i, mut j) = (0, 0);
    loop {
        // printing all sensor message queues from root thread -> for debugging only!
        // {
        //     for sensor in &cache.sensors {
        //         println!("messages of sensor {}", sensor.id);
        //         let q = sensor.queue.lock().unwrap();
        //         for x in q.iter() {
        //             print_tuple(sensor.id.clone(), x.clone());
        //             println!("");
        //         }
        //         println!("");
        //     }
        // }
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
        // handle.join().unwrap();

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
