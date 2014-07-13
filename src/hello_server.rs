use std::io::{Listener, Acceptor};
use std::io::net::tcp::TcpListener;

fn main() {
    let mut acceptor = TcpListener::bind("127.0.0.1", 9123).listen().unwrap();
    println!("listening started, ready to accept");
    for opt_stream in acceptor.incoming() {
        spawn(proc() {
            let mut stream = opt_stream.unwrap();
            println!("got a connection from '{}'", stream.peer_name().unwrap());
            stream.write(b"Hello World\r\n").unwrap();
        })
    }
}