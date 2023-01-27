use std::error::Error;
use std::{fs, thread};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use ThreadPool::ThreadPool;

fn main() ->Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // 绑定端口，启动监听服务
    let listener = TcpListener::bind("127.0.0.1:8030").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        pool.execute(|| {
            println!("aaa");
            handle_connection(stream.unwrap());
        });
    }
    println!("Shutting down");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection Establish");

    let mut buffer = [0; 512];
    // 将stream数据读取到缓冲区数组中
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 400 Not Found\r\n\r\n","404.html")
    };
    // 组装响应数据
    // let contents = fs::read_to_string("./404.html").unwrap();
    // let response = format!("{}{}", status_line, contents);
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}
