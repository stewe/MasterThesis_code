#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;
extern crate time;
extern crate zmq;
extern crate enclave_cache;

extern crate zmq_rs;
extern crate zmq_ffi;

use std::collections::{HashMap};
use std::sync::Arc;
use std::sync::mpsc::{Sender};
use std::thread::JoinHandle;

use enclave_cache::*;
use zmq::{Socket, Context};

// use zmq_rs;


// ------------------------------------------------------------------------------------
fn main() {
    // env_logger::init().unwrap();
    simple_logger::init().unwrap();
    info!("Cache started.");

    let mut cache = CacheApp::new();
    let mut sensor_threads: HashMap<String, (JoinHandle<()>, Sender<SensorThreadCmd>)> = HashMap::new();
    // let mut pushed_data =
    let mut ctx = Context::new();


    // threads receiving sensor messages
    for (id, sensor) in &cache.sensors {
        let sensor = sensor.clone();
        let queue = sensor.queue.clone();
        let (handle, tx) = sensor_msg_thread(sensor, queue, &mut ctx);
        sensor_threads.insert(id.to_string(), (handle, tx));
    }


    // main thread: receive requests / API handling
        let mut socket: Socket = ctx.socket(zmq::REP).unwrap();
        socket.bind("tcp://*:5550").unwrap();


        // TODO parse request approriately
        let mut msg = zmq::Message::new().unwrap();
        loop {
            socket.recv(&mut msg, 0).unwrap();
            let msg_str = msg.as_str().unwrap();
            debug!("Received request {}", msg_str);
            let mut request = msg_str.split(' ');
            let op = request.next().unwrap_or("No operator!");

            let mut resp = String::new();
            match op {
                "ADD" => {
                // TODO receive parameters by parsing
                    let sensor_id = request.next().unwrap();
                    debug!("sensor_id: {}", sensor_id);
                    let caller_id = request.next().unwrap();
                    debug!("caller_id: {}", caller_id);
                    let filters: Vec<String> = request.map(|x| x.to_string()).collect();

                    let addr = "tcp://127.0.0.1:5556";  // TODO how to receive the addr?!

                    let is_active = sensor_threads.contains_key(sensor_id);
                    if is_active {
                    // add filter subscription to already existing thread
                        let tx = &sensor_threads.get(sensor_id).unwrap().1;
                        tx.send(SensorThreadCmd::add(filters.clone())).unwrap();
                        debug!("Main thread send add to zmq thread.");
                    } else {
                    // start new thread for subscribing to sensor
                        let sensor = Arc::new(Sensor::new(&sensor_id, addr, filters.clone(), cache.get_expiration()));
                        cache.add_sensor(&sensor_id, sensor);
                        let sensor = cache.sensors.get(sensor_id).unwrap();
                        let queue = sensor.queue.clone();
                        sensor_threads.insert(sensor_id.to_string(), sensor_msg_thread(sensor.clone(), queue, &mut ctx));
                    }
                    cache.add_subscription_to_log(sensor_id.clone(), caller_id, filters);
                    // TODO error handling!
                    resp = "Ok".to_string();

                },
                "REM" => {
                    let sensor_id = request.next().unwrap();
                    debug!("sensor_id: {}", sensor_id);
                    let caller_id = request.next().unwrap();
                    debug!("caller_id: {}", caller_id);
                    let filters: Vec<String> = request.map(|x| x.to_string()).collect();

                    cache.remove_subscription_from_log(sensor_id, caller_id, filters.clone());
                    if !cache.has_subscribers(sensor_id) && sensor_threads.contains_key(sensor_id) {
                        // No more subscribers for the sensor, exit the thread.
                        {
                            let tx = &sensor_threads.get(sensor_id).unwrap().1;
                            tx.send(SensorThreadCmd::exit()).unwrap();
                        }
                        cache.remove_sensor(sensor_id);
                        sensor_threads.remove(sensor_id);
                    } else {
                        let filters_to_remove = cache.get_unsubscribed_filters(sensor_id, filters);
                        if !filters_to_remove.is_empty() {
                            // No more subscribers for these filters, unsubscribe from zmq sockets.
                            let tx = &sensor_threads.get(sensor_id).unwrap().1;
                            tx.send(SensorThreadCmd::remove(filters_to_remove)).unwrap();
                        }
                    }
                    // TODO error handling!
                    resp = "Ok".to_string();
                }
                "GETPM" => {
                    // let duration = request.next().unwrap();
                    // debug!("duration: {}", duration);
                    // let filter_one = request.next().unwrap();
                    // debug!("filter_one: {}", filter_one);
                    // let amount_one = request.next().unwrap();
                    // debug!("amount_one: {}", amount_one);
                    // let filter_two = request.next().unwrap();
                    // debug!("filter_two: {}", filter_two);
                    // let amount_two = request.next().unwrap();
                    // debug!("amount_two: {}", amount_two);
                    // this would become to complex, do it when the message format and parser are ready
                    let duration = 60000; // milliseconds of interest
                    let mut filter_per_sensors = Vec::new();
                    filter_per_sensors.push(("sensor-java".to_string(), vec!(("a".to_string(), 4 as usize))));
                    filter_per_sensors.push(("sensor-clj".to_string(), vec!(("1".to_string(), 2 as usize))));

                    let result = cache.get_published_msgs(duration, filter_per_sensors);
                    resp = format!("{:?}", result);
                    // TODO serialize!
                    debug!("{:?}", result);
                    // TODO error handling!

                }
                // next case
                _ => info!("Received unknown request.")
        }

            // parse and execute
            socket.send_str(&resp, 0).unwrap();

            // TODO used for debugging
            cache.print_msg_queues();
            cache.log_subscriptions();
        }

}


// let chks: Vec<i64> = msg.split(' ').map(|x| atoi(x)).collect();
// let (_zipcode, temperature, _relhumidity) = (chks[0], chks[1], chks[2]);
