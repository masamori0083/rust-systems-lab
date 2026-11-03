//! プロキシのhalf-closeを検証するため、EOFを受け取ってから応答する実験用サーバー。

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 転送先サーバーが、プロキシからの接続を待ち受ける入り口を作る
    let result = TcpListener::bind("127.0.0.1:8001");
    match result {
        Ok(listener) => {
            println!("待ち受け中: 127.0.0.1:8001");

            // 接続は1件だけ受け付ける。このstreamの相手はプロキシ
            let connection = listener.accept();

            match connection {
                Ok((mut stream, peer_addr)) => {
                    println!("接続相手: {peer_addr}");

                    // 受信したデータは保存せず、同じバッファを繰り返し使う
                    let mut buffer = [0u8; 1024];
                    loop {
                        let received = stream.read(&mut buffer);
                        match received {
                            Ok(0) => {
                                // 相手の送信終了を示すEOFを受け取ったので、読み取りを終える
                                println!("EOFを受信しました");
                                break;
                            }
                            Ok(n) => {
                                // データを受信しても、EOFが来るまでは応答しない
                                println!("受信: {n} バイト");
                            }
                            Err(error) => {
                                // 読み取り失敗はEOFとは区別し、応答せず終了する
                                println!("読み取りエラー: {error}");
                                return;
                            }
                        }
                    }

                    // EOFでループを抜けた後に、初めて固定のバイト列を返す
                    // 相手が送信だけを終了していれば、応答は引き続き受け取れる
                    let sent = stream.write_all(b"EOF received\n");
                    match sent {
                        Ok(()) => {
                            println!("応答を書き込みました");
                        }
                        Err(error) => {
                            println!("書き込みエラー: {error}");
                        }
                    }

                    // この分岐を抜けるとstreamが破棄され、接続を閉じる
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
