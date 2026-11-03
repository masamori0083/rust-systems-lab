use std::error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

fn main() {
    // プロキシが通信を待ち受ける入り口を作る
    let result = TcpListener::bind("127.0.0.1:9000");

    match result {
        Ok(listener) => {
            // クライアントからの接続を受け付ける
            let connection = listener.accept();
            match connection {
                Ok((mut stream, peer_addr)) => {
                    // プロキシと転送先サーバーとの通信口を作る
                    let upstream_result = TcpStream::connect("127.0.0.1:8001");

                    match upstream_result {
                        Ok(mut upstream) => {
                            println!("{upstream:?}");

                            // クライアントに応答を書き込むために、同じ接続のハンドルを複製する
                            let mut client_writer = match stream.try_clone() {
                                Ok(socket) => socket,
                                Err(error) => {
                                    println!("{error}");
                                    return;
                                }
                            };

                            // 転送先サーバーからの応答を読み取るために、同じ接続のハンドルを複製する
                            let mut upstream_reader = match upstream.try_clone() {
                                Ok(socket) => socket,
                                Err(error) => {
                                    println!("{error}");
                                    return;
                                }
                            };

                            let request_thread_result =
                                std::thread::Builder::new().spawn(move || {
                                    let mut buffer = [0u8; 1024];
                                    loop {
                                        // クライアントから読む
                                        let received = stream.read(&mut buffer);

                                        match received {
                                            Ok(0) => {
                                                // クライアントの送信終了を受け、転送先への送信も終了する
                                                let shutdown = upstream.shutdown(Shutdown::Write);
                                                match shutdown {
                                                    Ok(()) => {}
                                                    Err(error) => {
                                                        println!("{error}");
                                                        break;
                                                    }
                                                }
                                                break;
                                            }
                                            Ok(n) => {
                                                // 転送先へ書く
                                                let sent = upstream.write_all(&buffer[..n]);
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
                                });

                            let mut add_buffer = [0u8; 1024];

                            // 追加スレッドの起動結果を確認する
                            let request_thread = match request_thread_result {
                                Ok(handle) => handle,
                                Err(error) => {
                                    println!("{error}");
                                    return;
                                }
                            };

                            // 転送先から届く応答を、最後までクライアントに返す
                            loop {
                                // 転送先からの応答を読む
                                let received = upstream_reader.read(&mut add_buffer);
                                match received {
                                    // EOFなら終わり
                                    Ok(0) => {
                                        let shutdown = client_writer.shutdown(Shutdown::Write);
                                        match shutdown {
                                            Ok(()) => {}
                                            Err(error) => {
                                                println!("{error}");
                                                break;
                                            }
                                        }
                                        break;
                                    }
                                    // 書き込みする
                                    Ok(n) => {
                                        let sent = client_writer.write_all(&add_buffer[..n]);
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
                            // この位置で追加スレッドの終了を待つ
                            let joined = request_thread.join();
                            match joined {
                                Ok(()) => {}
                                Err(_) => {
                                    println!("pannic happen")
                                }
                            }
                        }

                        // 追加スレッドの終了を待
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
        Err(error) => {
            println!("{error}");
        }
    }
}
