extern crate time;
extern crate zmq;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread::{JoinHandle, spawn};
use time::{get_time, Timespec};
use zmq::{Socket, Context, DONTWAIT};

const EXIT: &'static str = "exit";

#[derive(Clone, Debug)]
pub struct Sensor {
    pub id: String,
    pub addr: String,
    pub filters: HashSet<String>,
    pub queue: Arc<Mutex<VecDeque<(String, Timespec)>>>, // adapt String

}

impl Sensor {
    pub fn new(id: &str, addr: &str, filters: Vec<String>) -> Sensor {
        Sensor {
            id: id.to_string(),
            addr: addr.to_string(),
            filters: filters.into_iter().collect::<HashSet<String>>(),
            queue: Arc::new(Mutex::new(VecDeque::new())),   // TODO ensure ring buffer size!
        }
    }

    pub fn get_addr_str(&self) -> &str {
        &self.addr
    }
}

pub struct Cache {
    pub sensors: HashMap<String, Arc<Sensor>>,
    // sensor-id/data-id -> queue
    // pub queue_map: Arc<Mutex<HashMap<String, VecDeque<(String, Timespec)>>>>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            sensors: init_sensors_info(),
            // sensor-id -> socket
            // socket_map: HashMap::new(),
            // queue_map: Arc::new(Mutex::new(HashMap::new())),s
            // qs: vec![],//init_qs(self.sensors_info, self.queue_map),
        }
    }

    pub fn contains_sensor(&self, id: &str) -> bool {
        self.sensors.contains_key(id)
    }

    pub fn add_sensor(&mut self, id: &str, sensor: Arc<Sensor>) {
        self.sensors.insert(id.to_string(), sensor);
    }

    pub fn print_msg_queues(&self) {
        for (id, sensor) in self.sensors.iter() {

            let q = sensor.queue.lock().unwrap();
            let size = q.len();
            println!("{} contains: ", id);
            for i in 0..size {
                let a = q.get(i).unwrap();
                print_tuple(id.clone(), a.clone());
                println!("");
            }
            println!("");
        }
    }

}

enum SensorThreadCmdType { Add, Remove }

pub struct SensorThreadCmd {
    op: SensorThreadCmdType,
    pub filters: Vec<String>,
}

impl SensorThreadCmd {
    pub fn new_add(filters: Vec<String>) -> SensorThreadCmd{
        SensorThreadCmd {
            op: SensorThreadCmdType::Add,
            filters: filters,
        }
    }
    pub fn new_remove(filters: Vec<String>) -> SensorThreadCmd{
        SensorThreadCmd {
            op: SensorThreadCmdType::Remove,
            filters: filters,
        }
    }
}

// TODO read and parse config file
pub fn init_sensors_info() -> HashMap<String, Arc<Sensor>> {
    // TODO read congig file and get sensors
    // let sensor_clj = Sensor::new("sensor-clj",
    //                                 "tcp://127.0.0.1:5556",
    //                                 vec!["1".to_string(), "2".to_string()]);
    let sensor_java = Sensor::new("sensor-java",
                                    "tcp://127.0.0.1:5555",
                                    vec!["a".to_string()]);

    let mut sensors = HashMap::new();
    // sensors.insert("sensor-clj".to_string(), Arc::new(sensor_clj));
    sensors.insert("sensor-java".to_string(), Arc::new(sensor_java));
    println!("Initialized information for {} sensors.", sensors.len());
    sensors
}

pub fn init_sensor_socket(sensor: &Sensor, ctx: &mut Context) -> Socket {
        let mut socket: Socket = ctx.socket(zmq::SUB).unwrap();
        for filter in &sensor.filters {
            assert!(socket.set_subscribe(filter.as_bytes()).is_ok());
        }
        match socket.connect(sensor.get_addr_str()) {
          Ok(()) => (),
          Err(e) => panic!(e) // TODO panic or tolerate???
        }
    socket
}

pub fn sensor_msg_thread(sensor: Arc<Sensor>, queue: Arc<Mutex<VecDeque<(String, Timespec)>>>)
    -> (JoinHandle<()>, Sender<SensorThreadCmd>) {
    let (tx, rx) = channel::<SensorThreadCmd>();
    let handle = spawn(move || {
        // init socket for sensor subscription
        println!("Started thread for sensor {}", sensor.id);
        let mut ctx = Context::new();
        let mut socket = init_sensor_socket(&sensor, &mut ctx);
        loop {
            // read message from zmq socket
            match socket.recv_string(DONTWAIT) { // TODO adapt to message format (e.g. google protocol buffers); handle in an approriate way!
                Ok(msg) =>  { let time: Timespec = get_time();
                                let mut queue = queue.lock().unwrap();
                                queue.push_back((msg.unwrap(), time));
                            },
                Err(_)  => { }
            }
            // read message from command channel
            match rx.try_recv() {
                Ok(cmd) => { match cmd.op {
                                SensorThreadCmdType::Add => {
                                    for filter in cmd.filters {
                                        match socket.set_subscribe(filter.as_bytes()) {
                                            Ok(_) => println!("Thread \"{}\" added filter: {}",
                                                        sensor.id, filter),
                                            Err(e) => println!("Thread \"{}\" failed to subscribe to {} - error {}",
                                                        sensor.id, filter, e)
                                        }; // further error handling?
                                    }
                                }, // first?
                                SensorThreadCmdType::Remove => {

                                }, // TODO remove
                            }},
                Err(_) => {},
            }
        }
    });
    (handle, tx)
}



// pub fn cache_sensor_msg(id: String, msg: String, time: Timespec, q: &Arc<Mutex<VecDeque<(String, Timespec)>>>) {
//     println!("caching {} for {}.", msg, id);
//     let mut q = q.lock().unwrap();
//     q.push_front((msg, time));
// }

fn print_tuple(id: String, t: (String, Timespec)) {
    let (msg, time) = t;
    print!("id: {}; msg: {}, time: {}.{}", id, msg, time.sec, time.nsec);
}

pub fn print_arc_queues(queues: &HashMap<String, Arc<Mutex<VecDeque<(String, Timespec)>>>>) {
    for (id, q) in queues.iter() {
        let q = q.lock().unwrap();
        let size = q.len();
        println!("{} contains: ", id);
        for i in 0..size {
            let a = q.get(i).unwrap();
            print_tuple(id.clone(), a.clone());
            println!("");
        }
        println!("");
    }
}

#[allow(dead_code)]
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


#[allow(dead_code)]
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

//#[deprecated] // used for one thread reading all sensor zmq sockets
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
