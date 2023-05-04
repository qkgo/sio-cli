use rust_socketio::{ClientBuilder, Payload};
use std::env;
use std::time::Instant;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: socketio_perf_test <test_url>");
        return;
    }
    let test_url = &args[1];

    let start_dial = Instant::now();
    let mut client = ClientBuilder::new(test_url)
        .on("connect", |client, _payload| {
            let connect_time = start_dial.elapsed().as_millis();
            println!("Connected, Dial time: {} ms", connect_time);
            Box::pin(async move {
                // 获取响应数据
                let response = client.transport().response().unwrap();
                let status_code = response.status();
                let headers = response.headers();

                // 输出响应数据
                println!("HttpStatusCode: {}", status_code);
                println!("HeaderLists: {:?}", headers);

                // 断开连接
                client.disconnect().await.unwrap();
            })
        })
        .on("error", |_client, payload| {
            println!("Error: {:?}", payload);
            Box::pin(async {})
        })
        .finish()
        .await
        .unwrap();

    client.connect().await.unwrap();
    sleep(Duration::from_secs(2)).await; // 等待 2 秒以确保结果输出
}
