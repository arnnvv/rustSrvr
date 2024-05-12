use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";

    let end_point: String = HOST.to_owned() + ":" + PORT;
    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at port {}", PORT);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let content = "ANNVV SAYS HII";

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
}
