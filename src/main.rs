extern crate termion;

// use termion::{color, style};

use std::process;
use std::thread;
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
  let server_addr = "127.0.0.1:4040";
  if let Ok(mut stream) = TcpStream::connect(server_addr) {
    println!("Connected!");

    let mut input_stream = stream.try_clone().unwrap();

    thread::spawn(move || {
      let mut client_buffer = [0u8; 1024];

      loop {
        match input_stream.read(&mut client_buffer) {
          Ok(n) => {
            if n == 0 {
              process::exit(0);
            }

            io::stdout().write(&client_buffer).unwrap();
            io::stdout().flush().unwrap();
            client_buffer = [0u8; 1024];
          },
          Err(error) => {
            println!("Failure: {}", error.to_string());
          }
        }
      }
    });

    let output_stream = &mut stream;
    let mut user_buffer = String::new();

    loop {
      io::stdin().read_line(&mut user_buffer).unwrap();

      output_stream.write(user_buffer.as_bytes()).unwrap();
      output_stream.flush().unwrap();
      user_buffer = String::new();
    }
  } else {
    println!("Could not connect to server: {}", server_addr);
  }
}
