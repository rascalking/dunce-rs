use std::net::{SocketAddr, UdpSocket};

use dunce::packet::Packet;

fn main() {
    let mut packet = Packet::new_query("www.icann.org");
    println!("{}", packet);

    // TODO: clever "try another local port" loop
    let socket = UdpSocket::bind("0.0.0.0:34254").expect("couldn't bind to address");
    socket.connect("8.8.8.8:53").expect("couldn't connect to dns server");
    socket.send(packet.buf()).expect("couldn't send packet");
    let mut buf = [0; 512];
    let amt = socket.recv(&mut buf).unwrap();
    println!("received {}", amt);
}
