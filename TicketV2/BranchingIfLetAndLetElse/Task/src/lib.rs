pub enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    // TODO: Implement the `radius` method using
    //  either an `if let` or a `let/else`.
    pub fn radius(&self) -> f64 {
        let Shape::Circle { radius } = self
        else { panic!("Radius cannot be used with Shape::Circle") };
        *radius
    }
}
