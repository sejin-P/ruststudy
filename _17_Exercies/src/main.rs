#![allow(unused_variables, dead_code)]
use std::ops::Add;

// test for my question
struct Person {
    age: u32,
}

struct User {
    name: String,
    age: u32,
    weight: f32,
    person: Person,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User { name: name, age: age, weight: weight, person: Person{age: 1} }
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn age(&self) -> u32 {
        return self.age;
    }

    pub fn weight(&self) -> f32 {
        return self.weight;
    }

    // copy를 구현하지 않았기 때문에 borrowing만 가능.
    pub fn person(&mut self) -> &mut Person {
        return &mut self.person;
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

fn main() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    let mut age = bob.age();
    let p = bob.person();
    p.age = 5;
    age = 15;
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_weight() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.weight(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {x, y}
    }

    fn magnitude(&self) -> f64 {
        let sqr = i32::pow(self.x, 2) + i32::pow(self.y, 2);
        return f64::from(sqr).sqrt();
    }

    fn dist(&self, p1: Point) -> f64 {
        let distanceSqr = i32::pow(self.x-p1.x, 2) + i32::pow(self.y-p1.y, 2);
        return f64::from(distanceSqr).sqrt();
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn new() -> Self {
        Polygon{points: Vec::new()}
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    fn left_most_point(&self) -> Option<Point> {
        let len = self.points.len();
        if len == 0 {
            return None;
        }

        let mut leftMostPoint = self.points[0];
        for point in &self.points[1..len] {
            if point.x < leftMostPoint.x {
                leftMostPoint = *point;
            }
        }

        return Some(leftMostPoint);
    }

    fn length(&self) -> i32 {
        let mut s = 0;
        for point in &self.points {
            s += point.x + point.y;
        }

        return s;
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    fn new(center: Point, radius: i32) -> Self {
        Circle{center, radius }
    }

    fn length(&self) -> i32 {
        self.radius
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(&self) -> i32 {
        match self {
            Shape::Polygon(poly) => poly.length(),
            Shape::Circle(circle) => circle.length(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.points.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(Shape::Polygon(poly)),
            Shape::from(Shape::Circle(Circle::new(Point::new(10, 20), 5))),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![85, 5]);
    }
}



