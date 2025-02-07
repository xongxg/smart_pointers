#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_ref_t() {
    let c = 'Q';
    let ref ref_c = c;
    let ref_c2 = &c;

    assert!(*ref_c == *ref_c2);

    let point = Point { x: 1, y: 2 };
    let _copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };

    println!("{}", _copy_of_x);

    let mut mutable_point = point;
    println!("{:?}", point);

    {
        let Point {
            x: _,
            y: ref mut ref_to_y,
        } = mutable_point;
        println!("{}", ref_to_y);
        *ref_to_y = 3;
    }

    println!("{:?}", mutable_point);

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("{:?}", mutable_tuple);
}

#[test]
fn test_ref_t2() {
    let a = 10;
    let b = &a;
    let ref c = a;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

    let s = Some(String::from("hello"));
    match s {
        Some(ref s) => println!("{}", s),
        None => (),
    }
    println!("{:?}", s);
}

#[test]
fn test_ref_t3() {
    let robot_name = &Some(String::from("Bors"));
    match robot_name {
        &Some(ref name) => println!("{}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);
}

#[derive(Debug)]
struct Foo;

#[derive(Debug)]
struct Bar {
    data: Option<Box<Foo>>,
}

#[derive(Debug)]
enum BarErr {
    Nope,
}

impl Bar {
    // fn borrow(&self) -> Result<&Box<Foo>, BarErr> {
    //     match self.data {
    //         Some(ref data) => Ok(data),
    //         None => Err(BarErr::Nope),
    //     }
    // }
    fn borrow(&self) -> Result<&Box<Foo>, BarErr> {
        self.data.as_ref().ok_or(BarErr::Nope)
        // self.data.as_deref().ok_or(BarErr::Nope)
    }

    fn borrow_mut(&mut self) -> Result<&mut Box<Foo>, BarErr> {
        self.data.as_mut().ok_or(BarErr::Nope)
    }
}

#[test]
fn test_ref_t4() {
    let mut x = Bar {
        data: Some(Box::new(Foo)),
    };
    let mut x2 = Bar { data: None };

    {
        let x = x.borrow();
        println!("{:?}", x);
    }

    {
        let y = x.borrow();
        println!("{:?}", y);
    }
}
