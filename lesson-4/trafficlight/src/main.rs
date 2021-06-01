// 引入标准库时间类型
use std::time::Duration;

// 定义交通灯枚举类型
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义trait，打印交通灯时间
pub trait TrafficTime {
    fn light_time(&self) -> Duration;
}

// 为TrafficLight实现trait
impl TrafficTime for TrafficLight {
    fn light_time(&self) -> std::time::Duration {
        // 模式匹配
        match &self {
            // 打印不同时间
            TrafficLight::Red => Duration::new(30, 0),
            TrafficLight::Yellow => Duration::from_secs(3),
            TrafficLight::Green => Duration::from_secs(60),
        }
    }
}

// 函数入口
fn main() {
    // 定义枚举变量
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;
    //打印对应的交通灯时间
    println!("red light time: {:?}", red_light.light_time());
    println!("yellow light time: {:?}", yellow_light.light_time());
    println!("green light time: {:?}", green_light.light_time());
}
