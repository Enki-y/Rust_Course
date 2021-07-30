fn main() {
    println!("Hello, world!");
    let light = TrafficLight::Green;
    println!("Light is {}", light.time());
}

enum TrafficLight {
    Red,
    Yellow,
    Green
}


impl TrafficLight {
    fn time(&self) -> u8 {
        match self{
            TrafficLight::Red => 20,
            TrafficLight::Green => 30,
            TrafficLight::Yellow => 3
        }
    }
}
