use rust_socketio::{ClientBuilder, Payload, RawClient};
use std::time::Instant;
use structopt::StructOpt;
use std::time::Duration; 
use tokio::time::sleep;


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

    let start_dial = Instant::now();
    let sio_client = ClientBuilder::new(test_url)
      .namespace("/")
      .on("connect", move  |_payload: Payload, _socket: RawClient| {
        let connect_duration = start_dial.elapsed().as_millis();
         println!("Connection established. Duration: {:?}", connect_duration);
      })
      .on_any(|event, payload, _client| {
        if let Payload::String(str) = payload {
          println!("{} {}", String::from(event), str);
        }
      })
     .on("error", |err, _| eprintln!("Error: {:#?}", err))
     .connect()
     .expect("Connection failed");
     
     sio_client.disconnect().expect("Disconnect failed");

     sleep(Duration::from_secs(25)).await;
}