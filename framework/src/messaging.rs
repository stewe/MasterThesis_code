// publisher: zmq_socket (context, ZMQ_PUB); zmq_bind (socket, "tcp://*:5556"); zmq_send
// subscriber: zmq_socket (context, ZMQ_SUB); zmq_connect (socket, "tcp://*:5556"); zmq_setsockopt (socket, ZMQ_SUBSCRIBE, filter, strlen (filter)); zmq_recv
pub fn sub (endpoint: &str, filters: &[&str]) {}

// think about parameters:
pub fn unsub (endpoint: &str, filters: &[&str]) {} // zmq_setsockopt (subscriber, ZMQ_UNSUBSCRIBE, filter, strlen (filter));

// zmq_socket (context, ZMQ_REP/ZMQ_REQ); zmq_bind (socket, "tcp://*:5555"); zmq_send / zmq_recv
pub fn req (endpoint: &str, msg: &[char]) {

    let v: Vec<char> = From::from(msg);
    let s: String = v.into_iter().collect();
    println!("TEST: {}", s);
}

pub fn recv () -> ! {
    panic!();
}


// does the zmq enclave support multiple zmq-eps? how does the queue dispatch??? does each ep has an own queue?
// how to assign received msgs to senders?
