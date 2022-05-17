use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use web_server::ThreadPool;

fn main() {
    let url = "127.0.0.1:7878";
    let listener = TcpListener::bind(url).unwrap();

    let single_threat = false;

    println!("Init on http://{}", url);

    if single_threat {
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream, true);
        }
    } else {
        let threads_number = 8;
        let pool = ThreadPool::new(threads_number);
        let max_requests = 6;

        for stream in listener.incoming().take(max_requests) {
            let stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream, false);
            });
        }

        println!("Shutting down.");
    }
}

fn handle_connection(mut stream: TcpStream, log: bool) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    if log {
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\nGreet: Hi Backend Tribe!\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
