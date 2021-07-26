

mod traffic_light;
use traffic_light::*;

mod summation;
use summation::*;

mod area;
use area::*;

fn main(){
    //traffic_light homework
    let light_red = TrafficLight::Red;
    println!("light is : {:?}, itme : {}", light_red, light_red.time());

    let light_green = TrafficLight::Green;
    println!("light is : {:?}, itme : {}", light_green, light_green.time());

    let light_yellow = TrafficLight::Yellow;
    println!("light is : {:?}, itme : {}", light_yellow, light_yellow.time());

    //summation homework
    let data = [1, 2, 3, u32::MAX];
    if let Some(sum) = summation(&data[..]) {
        println!("data sum is : {}", sum);
    }else{
        println!("data sum is None");
    }

    //area homework
    println!("circle area : {}", calc_area(&Circle{
        radius: 1.0
    }));
    println!("circle area : {}", calc_area(&Triangle{
        high: 1.0,
        bottom: 2.0
    }));
    println!("circle area : {}", calc_area(&Square{
        side: 2.0
    }));
}