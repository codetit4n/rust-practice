use core::panic;
use std::{collections::HashMap, sync::Arc};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
//type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
//
//fn new_sharded_db(num_shards: usize) -> ShardedDb {
//    let mut db = Vec::with_capacity(num_shards);
//    for _ in 0..num_shards {
//        db.push(Mutex::new(HashMap::new()));
//    }
//    Arc::new(db)
//}
//
#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("listening...");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listner.accept().await.unwrap();

        let db = db.clone();

        println!("accepted...");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as Vec<u8>
                let mut db = db.lock().unwrap();

                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

// continue from https://tokio.rs/tokio/tutorial/channels
