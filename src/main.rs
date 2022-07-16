fn main() {
  // 声明枚举 TrafficLight的三个类型
  let red = TrafficLight::Red;
  let green = TrafficLight::Green;
  let yellow = TrafficLight::Yellow;
  // 三个枚举变量都调用notify方法打印对应的时间
  notify(&red);
  notify(&green);
  notify(&yellow);
}
// 定义一个枚举 TrafficLight
enum TrafficLight {
  Red,
  Green,
  Yellow,
}
// 定义一个trait Summary
pub trait Summary {
  // 必须实现time方法
  fn time(&self) -> u8;
}
// 为枚举 TrafficLight实现一个trait Summary
impl Summary for TrafficLight {
  //定义方法time
  fn time(&self) -> u8 {
    // 模式匹配三种颜色的灯输出三种时间
    match self {
      TrafficLight::Red => 120,
      TrafficLight::Green => 60,
      TrafficLight::Yellow => 10,
    }
  }
}
// 定义notify方法打印时间
pub fn notify<T: Summary>(item: &T) {
  println!("light is: {}", item.time());
}