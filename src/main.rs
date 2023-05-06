mod message;
use message::session_establishment::SessionEstablishment;
use message::*;
fn main() {
    // Create a HeartbeatRequest message
    let sess_establishment: SessionEstablishment = SessionEstablishment::new();

    // Create a UDP socket
    let socket = std::net::UdpSocket::bind("127.0.0.1:34254").expect("Could not bind UDP socket");
    // Send a vector of bytes to a remote address
    let remote_address = "127.0.0.9:8805";
    let message: Vec<u8> = sess_establishment.to_bytes();
    socket
        .send_to(&message, remote_address)
        .expect("Could not send message");
}
