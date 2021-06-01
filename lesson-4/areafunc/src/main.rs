// 定义trait
trait Geometry {
    // 定义trait行为实现的方法
    fn area(&self) -> f64;
}

// 三角形
struct Triangle {
    length: f64, // 长边
    height: f64, // 高
}

// 为三角形实现trait
impl Geometry for Triangle {
    // 计算面积
    fn area(&self) -> f64 {
        self.length * self.height / 2.0
    }
}

// 圆形
struct Circle {
    diameter: f64,
}

// 为圆形实现trait
impl Geometry for Circle {
    // 计算面积
    fn area(&self) -> f64 {
        let pi = std::f64::consts::PI;
        pi * self.diameter * self.diameter / 4.0
    }
}

// 长方形
struct Rectangle {
    length: f64, // 长
    width: f64,  // 宽
}

// 为长方形实现trait
impl Geometry for Rectangle {
    // 计算面积
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

// 输入泛型形状（泛型约束），打印面积
fn notify(item: impl Geometry) {
    println!("Geometry area is: {:?}", item.area());
}

// 函数入口
fn main() {
    // 定义三角形实例
    let tri = Triangle {
        length: 6.0,
        height: 3.0,
    };
    // 打印三角形面积
    notify(tri);
    // 定义圆形实例
    let cir = Circle { diameter: 10.0 };
    // 打印圆形面积
    notify(cir);
    // 定义长方形实例
    let rec = Rectangle {
        length: 5.0,
        width: 4.0,
    };
    // 打印长方形面积
    notify(rec);
}
