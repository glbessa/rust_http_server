use std::process;
use std::thread;
use std::net::{TcpListener, TcpStream};

use rust_http_server::ThreadPool;

const HOST: &str = "127.0.0.1";
const PORT: usize = 5050;
const NUM_THREADS: usize = 4;

fn main() {
    let listener = match TcpListener::bind(format!("{}:{}", HOST, PORT)) {
        Ok(l) => l,
        Err(e) => {
            println!("{}", e.to_string());
            process::exit(-1);
        }
    };
    let pool = ThreadPool::new(NUM_THREADS);


    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("Error when receiving a connection!");
                continue;
            }
        };

        /*
        thread::spawn(|| {
            handle_connection(stream);
        });
        */

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    //println!("Connected!");
    todo!();
}