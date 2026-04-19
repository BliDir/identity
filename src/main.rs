use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

    println!("{} listening on 0.0.0.0:{port}", identity::package_name());

    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let Ok(bytes_read) = stream.read(&mut buffer) else {
        return;
    };

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let response = if request.starts_with("GET /health ") {
        identity::health_response()
    } else {
        "HTTP/1.1 404 Not Found\r\ncontent-length: 0\r\n\r\n"
    };

    let _ = stream.write_all(response.as_bytes());
}
