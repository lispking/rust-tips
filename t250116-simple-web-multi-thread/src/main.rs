use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    runtime::Builder,
};

fn main() {
    // Create a runtime with a thread pool
    let runtime = Builder::new_multi_thread()
        .worker_threads(4) // Specify number of worker threads
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("Web server listening on port 8080");

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                handle_connection(socket).await;
            });
        }
    });
}

async fn handle_connection(mut socket: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello, World!";
    let _ = socket.write_all(response.as_bytes()).await;
    let _ = socket.flush().await;
}
