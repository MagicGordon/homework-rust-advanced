pub trait Light {
    fn time(&self) -> u8;
}

#[derive(Debug, Copy, Clone)]
pub enum TrafficLight{
    Red,
    Green,
    Yellow,
}

impl Light for TrafficLight{
    fn time(&self) -> u8{
        match self{
            TrafficLight::Red=> 60,
            TrafficLight::Green=> 30,
            TrafficLight::Yellow=> 10,
        }
    }
}