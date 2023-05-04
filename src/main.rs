use clap::{App, Arg};
use rust_socketio::{ClientBuilder, Payload, RawClient};
use std::time::Instant;
use tokio::runtime::Builder;

async fn measure_connect_time(path: &str) -> Result<std::time::Duration, String> {
    let builder = ClientBuilder::new(path);

    let start_time = Instant::now();
    let client = builder.connect().await.map_err(|e| e.to_string())?;

    let connect_duration = start_time.elapsed();
    Ok(connect_duration)
}

fn main() {
    let matches = App::new("Socket.io Performance Test")
        .version("0.1")
        .author("Your Name <your.email@example.com>")
        .about("Measures Socket.io connection time")
        .arg(
            Arg::new("path")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Error building runtime");

    let result = rt.block_on(measure_connect_time(path));

    match result {
        Ok(duration) => {
            println!("Connect time: {:?}", duration);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
