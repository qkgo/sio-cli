use rust_socketio::{ClientBuilder, Payload, RawClient, TransportType, Event};
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


// https://github.com/1c3t3a/rust-socketio/blob/cb107ba6770fabb109dce173b74ab36ebe039045/socketio/src/client/client.rs#L163
#[tokio::main]
fn main() {
    let opt = Opt::from_args();
    let test_url = opt.path;

    let start_dial = Instant::now();
    let sio_client = ClientBuilder::new(test_url)
      .reconnect(true)
      .max_reconnect_attempts(100)
      .reconnect_delay(100, 100)
      // .transport_type(TransportType::Websocket)
      .namespace("/")
      
      .on(Event::Connect, move |_payload: Payload, _socket: RawClient| {
        let connect_duration = start_dial.elapsed().as_millis();
         println!("Connection established. Duration: {:?}", connect_duration);
        // connect_num_clone.fetch_add(1, Ordering::SeqCst);
        // let r = socket.emit_with_ack(
            // "message",
            // json!(""),
            // Duration::from_millis(100),
            // |_, _| {},
        // );
        // assert!(r.is_ok(), "should emit message success");
       }) 
      // .on("connect", move  |_payload: Payload, _socket: RawClient| {
        // let connect_duration = start_dial.elapsed().as_millis();
        //  println!("Connection established. Duration: {:?}", connect_duration);
      // })
      
      .on(Event::Close, move |_, _| {
        let connect_duration = start_dial.elapsed().as_millis();
         println!("Connection closed. Duration: {:?}", connect_duration);
        // close_num_clone.fetch_add(1, Ordering::SeqCst);
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
}