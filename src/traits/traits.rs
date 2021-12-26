use std::detect::__is_feature_detected::sha;

pub fn run() {
    let circle = Circle {
        x: 1.0,
        y: 1.0,
        radius: 5.0,
    };

    print_area(circle);
}


struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// Trait is like an interface
trait HasArea {
    fn area(&self) -> f64;

    // can have default functions for a trait
    fn is_valid(&self) -> bool {
        true
    }
}

trait HasCircumference {
    fn circumference(&self) -> f64;
}

// implementation of trait for a struct, it can also be a primitive type - although not recommended
impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// separate implementation for different trait
impl HasCircumference for Circle {
    fn circumference(&self) -> f64 {
        2 * std::f64::consts::PI * self.radius
    }
}

// function that is bound to a trait
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

// function that is bound to multiple traits
fn print_area_circumference<T: HasArea + HasCircumference>(shape: T) {
    println!("This shape has an area of {} and circumference of {}", shape.area(), shape.circumference());
}
