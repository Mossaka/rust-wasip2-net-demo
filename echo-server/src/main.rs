use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        if stream.write_all(&buffer[..bytes_read]).is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::WriteZero, "Failed to write"));
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Echo server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}