use std::net::TcpStream;

const MQTT_BROKER: &str = "test.mosquitto.org";
const MQTT_PORT: u16 = 1883;

fn main() -> std::io::Result<()> {
    let _stream = TcpStream::connect((MQTT_BROKER, MQTT_PORT))?;
    println!("Connected to MQTT broker!");
    connect_packet("Test");
    Ok(())
}

fn connect_packet(client_id: &str) -> Vec<u8> {
    let mut packet = vec![
        16, // MQTT control packet type for CONNECT 
        0,  // Remaining length
        0, 4, // Length of protocol name
        b'M', b'Q', b'T', b'T', // Protocol name
        4,    // protocol level - 3.1.1
        2,    // Clean Session
        0, 60, // Keep alive for 60 sec
    ];
    // Add client id length and client id and set remaining length
    let client_id_bytes = client_id.as_bytes();
    packet.extend_from_slice(&(client_id_bytes.len() as u16).to_be_bytes());
    packet.extend_from_slice(client_id_bytes);
    packet[1] = (packet.len() - 2) as u8;
    println!("{:?}", packet);
    packet
}
