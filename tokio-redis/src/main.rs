use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Rem;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

#[tokio::main]
async fn main() {
    let map = sharded_db(5);

    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr)
        .await
        .expect("unable to start TCP listener");

    println!("Listening at {addr}");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = map.clone();
        tokio::spawn(async move { process(socket, db).await });
    }
}

fn sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
async fn process(socket: TcpStream, db: ShardedDb) {
    use mini_redis::Command::{self, Get, Set};

    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let command = Command::from_frame(frame).unwrap();
        println!("Received command: {:?}", command);
        let res = match command {
            Set(cmd) => {
                let key = cmd.key().to_string();
                get_db_shard(&db, &key)
                    .lock()
                    .unwrap()
                    .insert(key, cmd.value().clone());
                Frame::Simple("OK".to_string())
            }

            Get(cmd) => {
                let key = cmd.key().to_string();
                if let Some(value) = get_db_shard(&db, &key).lock().unwrap().get(key.as_str()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("not implemented {:?}", cmd),
        };

        conn.write_frame(&res).await.unwrap();
    }
}

fn get_db_shard<'a, 'b>(db: &'a ShardedDb, key: &'b String) -> &'a Mutex<HashMap<String, Bytes>> {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let key_index: usize = hasher.finish().rem(db.len() as u64) as usize;
    &db[key_index]
}
