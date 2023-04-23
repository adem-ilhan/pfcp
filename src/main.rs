mod message;
use message::elements::*;
use message::heartbeat::HeartbeatRequest;
use message::AssociationSetup::AssociationSetup;
use message::SessionEstablishment::SessionEstablishment;
use message::*;
fn main() {
    println!("Hello, world!");
    // Create a HeartbeatRequest message

    // Create a HeartbeatRequest message
    let heartbeat_request: SessionEstablishment = SessionEstablishment::new();

    // Create a UDP socket
    let socket = std::net::UdpSocket::bind("127.0.0.1:34254").expect("Could not bind UDP socket");
    // Send a vector of bytes to a remote address
    let remote_address = "127.0.0.9:8805";
    let message: Vec<u8> = heartbeat_request.to_bytes();
    println!("{:?}", message);
    socket
        .send_to(&message, remote_address)
        .expect("Could not send message");
    // print!("{}", heartbeat_request.header.lenght);

    // let a: NodeID = NodeID::new(NODE_ID_TYPE_IPV4, vec![172, 0, 0, 1]);
    // println!("{:?}", a);
}
