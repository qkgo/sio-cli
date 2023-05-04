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
    // let test_url = &args[1];
    
    // let opt = Opt::from_args();
    // let test_url = format!("{}{}", opt.domain, opt.test_path);

    let opt = Opt::from_args();
    let url = opt.path;

    let mut rt = Runtime::new().unwrap();

    // test_url.as_str()
    rt.block_on(async {
        let dial_start = Instant::now();
        let socket = SocketBuilder::new(test_url)
            .on("connect", |payload: Payload, socket: RawClient| {
                let connect_duration = dial_start.elapsed();
                println!("Connection established. Duration: {:?}", connect_duration);
                Ok(());
            })
            .on("error",  |err, _| {
                println!("Error: {}", err);
                Ok(());
            })
            .connect();

        match socket {
            Ok(socket) => {
                let handshake_duration = dial_start.elapsed();
                println!("Handshake completed. Duration: {:?}", handshake_duration);

                let http_status = socket.http_status().unwrap_or(0);
                let headers = socket.http_headers();

                println!("HTTP Status Code: {}", http_status);
                println!("Headers:");

                for (key, value) in headers {
                    println!("  {}: {}", key, value);
                }

                socket.disconnect().await.expect("Failed to disconnect");
            }
            Err(err) => {
                println!("Failed to establish a connection: {:?}", err);
            }
        }
    });


    // sleep(Duration::from_secs(2)).await;
}
