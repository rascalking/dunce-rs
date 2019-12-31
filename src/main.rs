use dunce::Packet;

fn main() {
    let mut packet = Packet::new_query("www.icann.org");
    println!("{}", packet);

    //let (amt, src) = socket.recv_from(&mut buf).unwrap();
    //let packet = parse_packet(buf, amt).unwrap();
}
