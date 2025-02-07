struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn new(name: &'a str) -> Person<'a> {
        Person { name }
    }

    fn get(&self) -> &'a str {
        println!("Hello, my name is {}.", self.name);
        self.name
    }
}

#[test]
fn test_person() {
    let person = Person::new("Dave");
    println!("{}", person.get());
}