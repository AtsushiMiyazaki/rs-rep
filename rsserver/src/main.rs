use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(x) => request_handler(x).unwrap(),
            Err(e) => println!("failed to accept incoming request: {:?}", e),
        };
    };
}

fn request_handler(mut stream: TcpStream) -> Result<(), &'static str> {
    let mut buf = [0; 1024];

    stream.read(&mut buf).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    match buf {
        _x if buf.starts_with(get) => stream.write(&handle_get()).unwrap(),
        _ => {
            eprintln!("{:?}", String::from_utf8_lossy(&buf));
            stream.write(&handle_error()).unwrap()
        },
    };

    Ok(())
}

fn handle_get() -> Vec<u8> {
    let contents = fs::read_to_string("hello.html").unwrap();

    let res = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    res.as_bytes().to_vec()
}

fn handle_error() -> Vec<u8> {
    let contents = fs::read_to_string("error.html").unwrap();

    let res = format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    res.as_bytes().to_vec()
}
