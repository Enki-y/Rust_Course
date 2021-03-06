use std::net::{TcpListener, TcpStream};//引入标准库
use std::thread;
use std::io::{Read, Write};

fn main() {//构建主函数

    let listener = TcpListener::bind("127.0.0.1:9527").unwrap();//绑定本地监听端口

    if let Ok(_stream) = TcpStream::connect("127.0.0.1:9527") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    //构建客户端
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {//模式匹配
            Ok(n) => {
                if n == 0 { 
                    // 连接被关闭
                    break;
                }
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                std::panic::panic_any(err);//生成错误信息
            }
        }
    }
}

