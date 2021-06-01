// 定义求和函数，返回Option，当类型溢出时输出None，否则输出变量和值
fn sum_check(x: u32, y: u32) -> Option<u32> {
    // 判断是否会溢出
    if x > u32::MAX - y {
        None
    } else {
        Some(x + y)
    }
}

// 函数入口
fn main() {
    // 不会溢出情况
    let x = 123456789;
    let y = 123456789;
    println!("sum {:?} + {:?} = {:?}", x, y, sum_check(x, y));
    // 会发生溢出情况
    let m = 3123456789;
    let n = 4123456789;
    println!("sum {:?} + {:?} = {:?}", m, n, sum_check(m, n));
}
