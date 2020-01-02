use std::net::{SocketAddr, UdpSocket};

use dunce::packet::Packet;

fn main() {
    let mut query = Packet::new_query("www.icann.org");
    println!("{}", query);

    // TODO: clever "try another local port" loop
    let socket = UdpSocket::bind("0.0.0.0:34254").expect("couldn't bind to address");
    socket.connect("8.8.8.8:53").expect("couldn't connect to dns server");
    socket.send(query.buf()).expect("couldn't send packet");
    let mut buf = vec![0; 512];
    let amt = socket.recv(buf.as_mut_slice()).unwrap();
    println!("received {} bytes", amt);
    buf.truncate(amt);
    let response = Packet::parse(buf);
    println!("{}", response);
}
