
pub trait Area {
    fn area(&self) -> f32;
}

pub struct Circle{
    pub radius: f32
}

impl Area for Circle{
    fn area(&self) -> f32{
        return 3.14 * self.radius * self.radius;
    }
}

pub struct Triangle{
    pub high: f32,
    pub bottom: f32
}

impl Area for Triangle{
    fn area(&self) -> f32{
        return self.bottom * self.high / 2.0;
    }
}

pub struct Square{
    pub side: f32
}

impl Area for Square{
    fn area(&self) -> f32{
        return self.side * self.side;
    }
}

pub fn print_area<T: Area>(graph: &T){
    println!("type : {} , area : {}", std::any::type_name::<T>(), graph.area());
}