use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;
use std::io;

use pool::ThreadPool;

pub fn listen_addr() {
    let linstener = TcpListener::bind("127.0.0.1:8088").unwrap();
    let pool = ThreadPool::new(4);
    let mut counter = 0;

    for stream in linstener.incoming(){
        if counter == 4 {
            println!("Exit! byte");
            break;
        }

        counter += 1;

        let stream = stream.unwrap();

        pool.execute( || {
            handle_conn(stream)
        });
    }
}

fn handle_conn(mut stream: TcpStream){
    let mut buffer = [0;512];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET";

    if buffer.starts_with(get) {
        send_http_response(load_html("views/hello.html"), &mut stream);
    }else{
        send_http_response(load_html("views/404.html"), &mut stream);
    }
}

fn send_http_response(content:Result<String,io::Error>, stream:&mut TcpStream) {
        let response = match content {
            Ok(content) => content,
            Err(e) => format!("HTTP/1.1 500 FAIL\r\n\r\n{}",e),
        };
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
}

fn load_html(file:&str) -> Result<String,io::Error> {
    let mut file = File::open(file)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(format!("HTTP/1.1 200 OK\r\n\r\n{}", content))
}