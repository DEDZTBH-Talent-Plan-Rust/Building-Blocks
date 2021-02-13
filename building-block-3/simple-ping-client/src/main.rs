use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> Result<(), anyhow::Error> {
    loop {
        print!("C: ");
        io::stdout().flush()?;
        let mut buffer = String::with_capacity(1024);
        io::stdin().read_line(&mut buffer)?;

        let mut stream = TcpStream::connect("127.0.0.1:6379")?;
        stream.write_all(&buffer.as_bytes())?;
        stream.flush()?;
        buffer.clear();
        let mut stream_reader = BufReader::with_capacity(1024, stream);
        stream_reader.read_line(&mut buffer)?;
        println!("S: {}", buffer);
    }
}
