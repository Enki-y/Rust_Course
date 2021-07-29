fn main() {
    println!("Hello, world!");
    let light = TrafficLight::Yellow;
    println!("Light is {}", light.time());
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}


impl TrafficLight {
    fn time(&self) -> u8 {
        60
    }
}
