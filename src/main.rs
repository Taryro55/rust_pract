enum Shape {
    Square{side: f32},
    Cube{side: f32},
    Circle{radius: f32},
    Sphere{radius: f32},
}

struct ShapeData {
    shape: Shape,
    area: f32,
    perimeter: Option<f32>,
}

impl ShapeData {
    fn get_area(&self) -> f32 {
        match &self.shape {
            Shape::Square {side} => side * side,
            Shape::Cube { side } => 6.0 * side * side,
            Shape::Circle { radius } => std::f32::consts::PI * radius * radius,
            Shape::Sphere { radius } => 4.0 * std::f32::consts::PI * radius * radius,
        }
    }

    fn get_perimeter(&self) -> f32 {
        match &self.shape {
            Shape::Square {side} => 4.0 * side,
            Shape::Cube { side: _ } => 0.0,
            Shape::Circle { radius } => 2.0 * std::f32::consts::PI * radius,
            Shape::Sphere { radius: _} => 0.0,
        }
    }
    
    fn set_area(&mut self) {
        self.area = match &self.shape {
            Shape::Square {side} => side * side,
            Shape::Cube { side } => 6.0 * side * side,
            Shape::Circle { radius } => std::f32::consts::PI * radius * radius,
            Shape::Sphere { radius } => 4.0 * std::f32::consts::PI * radius * radius,
        }
    }

    fn set_perimeter(&mut self) {
        self.perimeter = match &self.shape {
            Shape::Square {side} => Some(4.0 * side),
            Shape::Cube { side: _ } => None,
            Shape::Circle { radius } => Some(2.0 * std::f32::consts::PI * radius),
            Shape::Sphere { radius: _} => None,
        }
    }
}

fn main() {

    let mut paper = ShapeData{
        shape: Shape::Square { side: (12.0) },
        area: 0.0,
        perimeter: None,
    };
    let mut book = ShapeData{
        shape: Shape::Cube { side: (10.0) },
        area: 0.0,
        perimeter: None,
    };
    let mut coin = ShapeData{
        shape: Shape::Circle { radius: (1.5) },
        area: 0.0,
        perimeter: None,
    };
    let mut planet = ShapeData{
        shape: Shape::Sphere { radius: (10.0) },
        area: 0.0,
        perimeter: None,
    };


    paper.set_area();
    paper.set_perimeter();
    book.set_area();
    book.set_perimeter();
    coin.set_area();
    coin.set_perimeter();
    planet.set_area();
    planet.set_perimeter();


    println!("Area: {:?} & Perimeter: {:?}", paper.area, paper.perimeter);
    println!("Area: {:?} & Perimeter: {:?}", book.area, book.perimeter);
    println!("Area: {:?} & Perimeter: {:?}", coin.area, coin.perimeter);
    println!("Area: {:?} & Perimeter: {:?}", planet.area, planet.perimeter);
}
