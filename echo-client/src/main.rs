use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to the server on 127.0.0.1:8080");

    let mut input = String::new();
    loop {
        input.clear();
        print!("Enter message: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            break;
        }

        stream.write_all(input.as_bytes())?;

        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;
        println!("Echoed: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
    Ok(())
}