use std::cmp::Ordering;
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy)]
struct Person {
    age: i32,
    name: &'static str,
}
impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        (self.age).cmp(&(other.age))
    }
}
fn older(p1: Person, p2: Person) {
    if p1 > p2 {
        println!("{} tem mais anos de vida do que {}", p1.name, p2.name);
    } else {
        println!("{} tem mais anos de vida do que {}", p2.name, p1.name);
    }
}
fn main() {
    let p1 = Person {
        age: 6,
        name: "Nicolas",
    };
    let p2 = Person {
        age: 31,
        name: "Ana",
    };
    let p3 = Person {
        age: 34,
        name: "Kinash",
    };
    older(p1, p2);
    older(p2, p3);
    older(p1, p3);
}
