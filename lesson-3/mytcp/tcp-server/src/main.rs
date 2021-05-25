// 使用标准库std
use std::io::{BufRead, BufReader, Error, Write};
// 使用标准库std
use std::net::{SocketAddr, TcpListener, TcpStream};

// 处理接收到的tcp stream，进行读写操作
fn handler_connection(stream: TcpStream) -> Result<(), Error> {
    // local addr
    println!("[local addr: {:?}]", stream.local_addr().unwrap());
    // remote addr
    let remote_addr = stream.peer_addr().unwrap();
    // 打印连接的客户端地和端口
    println!("[remote addr: {:?}]", remote_addr);
    // tcp可读可写，数据流同时reader和writer是安全的
    let (reader, mut writer) = (&stream, &stream);
    // 存入读缓冲区
    let reader = BufReader::new(reader);
    // 行迭代
    let mut lines = reader.lines();
    // 循环读取直到结束
    while let Some(line) = lines.next() {
        let mut line = line.unwrap();
        // 打印远端地址及收到的消息
        println!("[remote addr: {:?} message] {}", remote_addr, &line);
        // 消息末尾添加换行符
        line.push('\n');
        // 将消息发送给客户端
        writer.write_all(line.as_bytes()).unwrap();
    }
    // 客户端断开连接
    println!("[remote addr: {:?} offline]", remote_addr);
    // 返回成功
    Ok(())
}

// 程序入口
fn main() -> std::io::Result<()> {
    // 如果创建127.0.0.1:8888的tcp listener失败,则创建127.0.0.1:80的tcp listener
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8888)),
        SocketAddr::from(([127, 0, 0, 1], 80)),
    ];
    // 由给定的地址，创建tcp listener，侦听连接
    let listener = TcpListener::bind(&addrs[..]).unwrap();
    // 连接在此listener上等待接收的迭代
    for stream in listener.incoming() {
        // 匹配stream
        match stream {
            // 处理正常情况
            Ok(s) => {
                // 处理接收到的tcp stream，进行读写操作
                handler_connection(s).unwrap_or_else(|error| eprintln!("{:?}", error));
            }
            // 处理异常情况
            Err(e) => {
                // 打印错误信息
                println!("{:#?}", e);
                // 退出本循环，处理下一个循环
                continue;
            }
        };
    }

    // 返回成功
    Ok(())
}
