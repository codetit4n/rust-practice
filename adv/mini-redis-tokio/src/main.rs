use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listner.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_owned());
        connection.write_frame(&response).await.unwrap();
    }
}

// continue from https://tokio.rs/tokio/tutorial/spawning#concurrency
