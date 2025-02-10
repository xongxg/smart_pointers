use std::mem;

struct Inner {
    data: String,
}

struct Outer {
    inner: Inner,
}

impl Drop for Outer {
    fn drop(&mut self) {
        println!("Dropping Outer");
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        println!("Dropping Inner");
    }
}

#[test]
fn test_drop() {
    let outer = Outer {
        inner: Inner {
            data: String::from("Some data"),
        },
    };

    println!("Outer and Inner created.");
}

struct CustomResource {
    name: String,
}

impl Drop for CustomResource {
    fn drop(&mut self) {
        println!("Releasing CustomResource: {}", self.name);
    }
}

#[test]
fn test_drop_custom_resource() {
    let resource = CustomResource {
        name: String::from("resource"),
    };

    println!("Custom resource created.");
    mem::drop(resource);
}
