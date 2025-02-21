trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Student + Programmer {
    fn git_username(&self) -> String;
}

struct Tom;

impl Student for Tom {
    fn university(&self) -> String {
        todo!()
    }
}

impl Person for Tom {
    fn name(&self) -> String {
        todo!()
    }
}

impl Programmer for Tom {
    fn fav_language(&self) -> String {
        todo!()
    }
}

impl CompSciStudent for Tom {
    fn git_username(&self) -> String {
        todo!()
    }
}

enum Animal {
    Cat(Cat),
    Dog(Dog),
}
struct Cat;
struct Dog;

impl Dog {
    fn get_noise(&self) -> String {
        "woof".to_owned()
    }
}
impl Cat {
    fn get_noise(&self) -> String {
        "meow".to_owned()
    }
}

impl Animal {
    fn get_noise(&self) -> String {
        match self {
            Animal::Cat(c) => c.get_noise(),
            Animal::Dog(d) => d.get_noise(),
        }
    }
}

mod traits {
    trait Animal {
        fn get_noise(&self) -> String;
    }

    struct Cat;
    impl Animal for Cat {
        fn get_noise(&self) -> String {
            "meow".to_owned()
        }
    }

    struct Dog;
    impl Animal for Dog {
        fn get_noise(&self) -> String {
            "woof".to_owned()
        }
    }

    fn make_noise(animal: &dyn Animal) {
        let noise = animal.get_noise();
        println!("{}", noise)
    }

    #[test]
    fn test() {
        let cat = Cat;
        make_noise(&cat);
        let dog = Dog;
        make_noise(&dog);
    }
}

mod composition {
    struct Animal {
        name: String,
    }

    trait Name {
        fn get_name(&self) -> String;
    }

    impl Name for Animal {
        fn get_name(&self) -> String {
            self.name.to_owned()
        }
    }

    struct Cat {
        animal_base: Animal,
    }

    impl Name for Cat {
        fn get_name(&self) -> String {
            self.animal_base.get_name()
        }
    }

    impl Cat {
        fn new(name: &str) -> Self {
            Self {
                animal_base: Animal {
                    name: name.to_owned(),
                },
            }
        }
    }

    #[test]
    fn test() {
        let cat = Cat::new("cat");
        println!("my cat name is {}.",cat.get_name());
    }
}
