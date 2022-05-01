use std::net::{IpAddr, SocketAddr};
use tokio_stream::StreamExt;

use f1_2021_telemetry;
use f1_2021_telemetry::packet;

#[tokio::main]
async fn main() {
    let ip_address = IpAddr::from([0, 0, 0, 0]);
    let port = 2021;
    let socket = SocketAddr::new(ip_address, port);

    let mut stream = f1_2021_telemetry::F1_2021::telemetry(socket).unwrap();

    while let Some(packet) = stream.next().await {
        println!("{:?}", packet.header);
    }
}
