// 使用标准库std
use std::io::{self, BufRead, BufReader, Write};
// 使用标准库std
use std::net::TcpStream;
// 使用标准库std
use std::str;

// 程序入口
fn main() -> std::io::Result<()> {
    // 连接tcp server 127.0.0.1:8888，返回stream
    let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    // 打印远端地址
    println!("[remote addr: {:?}]", stream.peer_addr().unwrap());

    // 循环发送或接收消息
    loop {
        // 定义变量存放标准输入
        let mut input = String::new();
        // 按行读取标准输入，会读入换行符
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        // 如果输入exit或空字符串，退出客户端
        if input == "exit\n" || input == "\n" {
            // 结束循环
            break;
        }
        // 打印标准输入字符串内容
        println!("[input message] {:?}", input);
        // 将标准输入字符串发送给tcp server
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        // 存入读缓冲区
        let mut reader = BufReader::new(&stream);
        // 定义向量接收tcp server发来的消息
        let mut buffer: Vec<u8> = Vec::new();
        // 将信息写入向量
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        // 打印从tcp server获得的数据
        println!(
            "[server message] {:?}",
            str::from_utf8(&buffer).expect("Could not write buffer as string")
        );
    }
    // 返回成功
    Ok(())
}
