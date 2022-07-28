//use anyhow::Result;
use std::net::{TcpStream};
use std::io::{Read, Write, Error};
//use std::str::from_utf8;

pub async fn process(text: &str, _lang: &str) -> Result<String, Error> {
    let mut data: String = "".to_string();
    let res: Result<String,Error>;
    match TcpStream::connect("127.0.0.1:9001") {
        Ok(mut stream) => {
            println!("Successfully connected to zh tokenizer server in port 9001");

            stream.write(text.as_bytes()).unwrap();
            println!("Sent text, awaiting reply...");

            match stream.read_to_string(&mut data) {
                Ok(_) => {
                    res = Ok(data)},
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    res = Err(e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            res = Err(e);
        }
    }
    res
}
