struct Person {
    name: String,
    age: i32,
}
impl Person {
    fn new(name: String, age: i32) -> Person {
        let p: Person = Person {name, age };
        return p;
    }
}
fn main() {
    let p1: Person = Person::new(String::from("Asrith"),21);
    println!("{},{}",p1.name,p1.age);
}
