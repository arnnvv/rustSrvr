use std::net::TcpListener;

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";

    let end_point: String = HOST.to_owned() + ":" + PORT;
    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at port {}", PORT);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
    }
}
