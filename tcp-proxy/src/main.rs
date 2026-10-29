use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let result = TcpListener::bind("127.0.0.1:9000");

    match result {
        Ok(listener) => {
            let connection = listener.accept();
            match connection {
                Ok((mut stream, peer_addr)) => {
                    let mut buffer = [0u8; 1024];
                    loop {
                        let received = stream.read(&mut buffer);
                        match received {
                            Ok(0) => {
                                break;
                            }
                            Ok(n) => {
                                let sent = stream.write_all(&buffer[..n]);
                                match sent {
                                    Ok(()) => {}
                                    Err(error) => {
                                        println!("{error}");
                                        break;
                                    }
                                }
                            }
                            Err(error) => {
                                println!("{error}");
                                break;
                            }
                        }
                    }
                }
                Err(error) => {
                    println!("{error}");
                }
            }
        }
        Err(error) => {
            println!("{error}");
        }
    }
}
