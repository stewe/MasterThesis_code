extern crate zmq;

fn main() {
    let mut ctx = zmq::Context::new();

    let mut socket = match ctx.socket(zmq::REQ) {
      Ok(socket) => { socket },
      Err(e) => { panic!(e) }
    };

    match socket.connect("tcp://127.0.0.1:1234") {
      Ok(()) => (),
      Err(e) => panic!(e)
    }

    match socket.send_str("hello world!", 0) {
      Ok(()) => (),
      Err(e) => panic!(e)
    }
}



// init: subscribe to sensors
// receive messages in new thread
// listen to req socket and reply
// queues for messages
// multiple microservices? keep track of filters / sensors
