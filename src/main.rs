use rust_socketio::{ClientBuilder, Payload, RawClient};
use std::time::Instant;
use structopt::StructOpt;
use tokio::runtime::Runtime;
use std::time::Duration; 


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

  
    let sio_client = ClientBuilder::new(test_url)
      .namespace("/")
      .on("connect", |payload: Payload, socket: RawClient| {
         let connect_duration = dial_start.elapsed();
         println!("Connection established. Duration: {:?}", connect_duration);
      })
     .on("error", |err, _| eprintln!("Error: {:#?}", err))
     .connect()
     .expect("Connection failed");
 
  

     sio_client.disconnect().expect("Disconnect failed")


     sleep(Duration::from_secs(25)).await; // 等待 2 秒以确保结果输出
}