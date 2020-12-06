// 为枚举交通信号等实现一个trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
#[derive(Debug)]
enum TrafficLight{
    Red,
    Green,
    Yellow,
}


// std::cmp::PartialEq
impl TrafficLight{
    fn time(&self) -> u8  {
        match &self{
            TrafficLight::Yellow => 60,
            TrafficLight::Green => 80,
            TrafficLight::Red => 100,
        }
       
    }
}

fn main() {
    let yellow = TrafficLight::Yellow;
    println!("{}",yellow.time());

    let green = TrafficLight::Green;
    println!("{}",green.time());

    let red = TrafficLight::Red;
    println!("{}",red.time());
}
