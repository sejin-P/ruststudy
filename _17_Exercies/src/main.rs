#![allow(unused_variables, dead_code)]

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
    pub fn person(&self) -> &Person {
        return &self.person;
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    let mut age = bob.age();
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