use rust_socketio::{ClientBuilder, Payload, RawClient};
use std::time::Instant;
use structopt::StructOpt;
use tokio::runtime::Runtime;

#[derive(StructOpt, Debug)]
#[structopt(name = "socketio-perf-test")]
struct Opt {
    #[structopt(short, long, help = "Test path")]
    path: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let test_url = opt.path;

    let dial_start = Instant::now();
    let sio_client = ClientBuilder::new(test_url)
        .namespace("/admin")
        .on("connect", |payload: Payload, socket: RawClient| {
            let connect_duration = dial_start.elapsed();
            println!("Connection established. Duration: {:?}", connect_duration);
            Ok::<(), E>(());
        })
        .connect()
        .await;

    match sio_client {
        Ok(_) => {
            let handshake_duration = dial_start.elapsed();
            println!("Handshake completed. Duration: {:?}", handshake_duration);
        }
        Err(err) => {
            println!("Failed to establish a connection: {:?}", err);
        }
    }
}