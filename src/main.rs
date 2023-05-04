use rust_socketio::{SocketBuilder, ClientBuilder, Payload, RawClient};
use serde_json::json;
use std::env;
use std::time::Instant;
use tokio::time::sleep;
use std::time::Duration;
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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: socketio_perf_test <test_url>");
        return;
    }

    let opt = Opt::from_args();
    let test_url = opt.path;

    let mut rt = Runtime::new().unwrap();

    // test_url.as_str()
    rt.block_on(async {
        let dial_start = Instant::now();
        let sioClient = ClientBuilder::new(test_url)
            .namespace("/admin")
            .on("connect", |payload: Payload, socket: RawClient| {
                let connect_duration = dial_start.elapsed();
                println!("Connection established. Duration: {:?}", connect_duration);
                Ok::<(), E>(());
            })
            .on("error", |err, _| eprintln!("Error: {:#?}", err))
            .connect();

        match sioClient {
            Ok(sioClient) => {
                let handshake_duration = dial_start.elapsed();
                println!("Handshake completed. Duration: {:?}", handshake_duration); 
            }
            Err(err) => {
                println!("Failed to establish a connection: {:?}", err);
            }
        }
    });


    // sleep(Duration::from_secs(2)).await;
}
