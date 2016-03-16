extern crate time;
extern crate zmq;
extern crate enclave_cache;

use std::collections::{HashMap};
use std::env::args;
use std::sync::Arc;
use std::sync::mpsc::{Sender};
use std::thread::JoinHandle;

use enclave_cache::*;
use zmq::{Socket, Context};

// ------------------------------------------------------------------------------------
fn main() {
    let debug_mode;
    match args().next() {
        Some(x) => debug_mode = x == "debug".to_string(),
        _ => debug_mode = false,
    }

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
            let op = request.next().unwrap_or("No operator!");

            match op {
                "ADD" => {
                // TODO receive parameters by parsing
                // let sender ... -> track client <-> filter
                    let sensor_id = request.next().unwrap();
                    debug_print(debug_mode, format!("sensor_id: {}", sensor_id));
                    let caller_id = request.next().unwrap();
                    debug_print(debug_mode, format!("caller_id: {}", caller_id));
                    let filters: Vec<String> = request.map(|x| x.to_string()).collect(); //vec!["1".to_string(), "2".to_string()];

                    let addr = "tcp://127.0.0.1:5556";  // TODO how to receive the addr?!

                    let is_active = sensor_threads.contains_key(sensor_id);
                    if is_active {
                    // add filter subscription to already existing thread
                        let tx = &sensor_threads.get(sensor_id).unwrap().1;
                        // let filters = vec!["0".to_string()]; // TODO remove, it's just used for developing and debugging
                        tx.send(SensorThreadCmd::new_add(filters.clone())).unwrap();
                        debug_print(debug_mode, "main thread send add to zmq thread.".to_string()); // TODO DEBUG
                    } else {
                    // start new thread for subscribing to sensor
                        let sensor = Arc::new(Sensor::new(&sensor_id, addr, filters.clone()));
                        cache.add_sensor(&sensor_id, sensor);
                        let sensor = cache.sensors.get(sensor_id).unwrap();
                        let queue = sensor.queue.clone();
                        sensor_threads.insert(sensor_id.to_string(), sensor_msg_thread(sensor.clone(), queue));
                    }
                    cache.add_subscription_to_log(sensor_id.clone(), caller_id, filters);

                },
                "REM" => {
                    let sensor_id = request.next().unwrap();
                    debug_print(debug_mode, format!("sensor_id: {}", sensor_id));
                    let caller_id = request.next().unwrap();
                    debug_print(debug_mode, format!("caller_id: {}", caller_id));
                    let filters: Vec<String> = request.map(|x| x.to_string()).collect(); //vec!["1".to_string(), "2".to_string()];

                    cache.remove_subscription_from_log(sensor_id, caller_id, filters.clone());
                    if !cache.has_subscribers(sensor_id) && sensor_threads.contains_key(sensor_id) {
                        // No more subscribers for the sensor, exit the thread.
                        {
                            let tx = &sensor_threads.get(sensor_id).unwrap().1;
                            tx.send(SensorThreadCmd::new_exit()).unwrap();
                        }
                        cache.remove_sensor(sensor_id);
                        sensor_threads.remove(sensor_id);
                    } else {
                        let filters_to_remove = cache.get_unsubscribed_filters(sensor_id, filters);
                        if !filters_to_remove.is_empty() {
                            // No more subscribers for these filters, unsubscribe from zmq sockets.
                            let tx = &sensor_threads.get(sensor_id).unwrap().1;
                            tx.send(SensorThreadCmd::new_remove(filters_to_remove)).unwrap();
                        }
                    }
                }
                _ => println!("Received unknown request.")
        }

            // parse and execute
            socket.send_str("World", 0).unwrap();

            // TODO used for debugging
            cache.print_msg_queues();
            cache.print_subscriptions();
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
