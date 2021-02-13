use anyhow::Error;
use shellwords::MismatchedQuotes;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    for stream in listener.incoming() {
        let stream = stream?;
        println!("Connection established with {:?}", stream);
        match handle_connection(stream) {
            Ok(_) => {}
            Err(e) => eprintln!("{:?}", e),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), anyhow::Error> {
    let mut stream_reader = BufReader::with_capacity(1024, &stream);
    let mut request = String::new();
    stream_reader.read_line(&mut request)?;
    let response: String = match shellwords::split(&request) {
        Ok(args) => match args.len() {
            0 => "ERR No command\n".to_owned(),
            1 => {
                if args[0].to_uppercase() == "PING" {
                    "\"PONG\"\n".to_owned()
                } else {
                    format!("ERR Unknown or disabled command '{}'\n", args[0])
                }
            }
            2 => {
                if args[0].to_uppercase() == "PING" {
                    let mut striped_arg: &str = &args[1];
                    let pattern = &['\'', '\"'][..];
                    if let Some(s) = striped_arg.strip_prefix(pattern) {
                        striped_arg = s;
                    }
                    if let Some(s) = striped_arg.strip_suffix(pattern) {
                        striped_arg = s;
                    }
                    format!("\"{}\"\n", striped_arg)
                } else {
                    format!("ERR Unknown or disabled command '{}'\n", args[0])
                }
            }
            _ => "ERR wrong number of arguments for 'ping' command\n".to_owned(),
        },
        Err(e) => format!("ERR {:?}", e), // THIS IS NOT STANDARD!
    };

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
