extern crate time;
extern crate zmq;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread::{JoinHandle, spawn};
use time::{get_time, Timespec};
use zmq::{Socket, Context, DONTWAIT};

pub fn debug_print(mode: bool, s: String) {
    if mode { println!("{}", s) };
}

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
    // Sensor-ID -> {filter -> Set(Subscriber)}
    subscribers: HashMap<String, HashMap<String, HashSet<String>>>
}

impl Cache {
    pub fn new() -> Cache {
        let cache_values = init_sensors_info();
        Cache {
            sensors: cache_values.0,
            subscribers: cache_values.1,
        }
    }

    pub fn contains_sensor(&self, id: &str) -> bool {
        self.sensors.contains_key(id)
    }

    pub fn add_sensor(&mut self, id: &str, sensor: Arc<Sensor>) {
        self.sensors.insert(id.to_string(), sensor);
    }

    pub fn remove_sensor(&mut self, id: &str) {
        self.sensors.remove(id);
    }

    pub fn add_subscription_to_log(&mut self, sensor_id: &str, caller_id: &str, filters: Vec<String>) {
        let filter_map = self.subscribers.entry(sensor_id.to_string()).or_insert(HashMap::new());
        for f in filters {
            let set = filter_map.entry(f).or_insert(HashSet::new());
            set.insert(caller_id.to_string());
        }
    }

    pub fn remove_subscription_from_log(&mut self, sensor_id: &str, caller_id: &str, filters: Vec<String>) {
        let remove_sensor;
        {
            let mut filter_map = match self.subscribers.get_mut(sensor_id) {
                Some(v) => v,
                None => return
            };
            for f in filters {
                let remove_filter;
                {
                    let set = filter_map.get_mut(&f);
                    match set {
                        Some(v) => { v.remove(caller_id); remove_filter = v.is_empty(); },
                        None => remove_filter = true,
                    };
                }
                if remove_filter { filter_map.remove(&f); }
            }
            remove_sensor = filter_map.is_empty();
            }
        if remove_sensor {
            self.subscribers.remove(sensor_id);
        }
    }

    pub fn has_subscribers(&self, sensor_id: &str) -> bool {
        self.subscribers.contains_key(sensor_id)
    }

    pub fn get_unsubscribed_filters(&self, sensor_id: &str, filters: Vec<String>) -> Vec<String> {
        match self.subscribers.get(sensor_id) {
            Some(v) => filters.iter().cloned().filter(|x| !v.contains_key(x)).collect(),
            None => vec![]
        }
    }

    pub fn print_subscriptions(&self) {
        println!("subscriptions: {:?}", self.subscribers);
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

enum SensorThreadCmdType { Add, Remove, Exit }

pub struct SensorThreadCmd {
    op: SensorThreadCmdType,
    pub filters: Option<Vec<String>>,
}

impl SensorThreadCmd {
    pub fn new_add(filters: Vec<String>) -> SensorThreadCmd {
        SensorThreadCmd {
            op: SensorThreadCmdType::Add,
            filters: Some(filters),
        }
    }

    pub fn new_remove(filters: Vec<String>) -> SensorThreadCmd {
        SensorThreadCmd {
            op: SensorThreadCmdType::Remove,
            filters: Some(filters),
        }
    }

    pub fn new_exit() -> SensorThreadCmd {
        SensorThreadCmd {
            op: SensorThreadCmdType::Exit,
            filters: None,
        }
    }
}

// TODO read and parse config file
pub fn init_sensors_info() -> (HashMap<String, Arc<Sensor>>, HashMap<String, HashMap<String, HashSet<String>>>) {
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

    let mut subs = HashMap::new();
    let mut filtermap = HashMap::new();
    let mut aset = HashSet::new();
    aset.insert("config".to_string());  // !!! subs from config file get caller_id "config"
    filtermap.insert("a".to_string(), aset);
    subs.insert("sensor-java".to_string(), filtermap);
    (sensors, subs)
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

        // TODO try to use only one context, eg by creating sockets first, then passing it to the thread
        // mabe compare https://github.com/erickt/rust-zmq/blob/master/examples/msgsend/main.rs
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
                                    for filter in cmd.filters.unwrap() {
                                        match socket.set_subscribe(filter.as_bytes()) {
                                            Ok(_) => println!("Thread \"{}\" added filter: {}",
                                                        sensor.id, filter),
                                            Err(e) => println!("Thread \"{}\" failed to subscribe to {} - error {}",
                                                        sensor.id, filter, e)
                                        }; // further error handling?
                                    }
                                }, // first?
                                SensorThreadCmdType::Remove => {
                                    for filter in cmd.filters.unwrap() {
                                        match socket.set_unsubscribe(filter.as_bytes()) {
                                            Ok(_) => println!("Thread \"{}\" removed filter: {}",
                                                        sensor.id, filter),
                                            Err(e) => println!("Thread \"{}\" failed to unsubscribe from {} - error {}",
                                                        sensor.id, filter, e)
                                        }; // further error handling?
                                    }
                                },
                                SensorThreadCmdType::Exit => panic!("No more subscrptions for sensor {}, closing connection.", sensor.id),
                            }},
                Err(_) => {},
            }
        }
    });
    (handle, tx)
}

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
