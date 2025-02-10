use std::ptr::addr_of_mut;

#[derive(Debug)]
struct Item(i32, Option<Box<Item>>);

#[derive(Debug)]
struct List(Option<Box<Item>>);

impl List {
    fn new() -> List {
        List(None)
    }

    fn append(&mut self, value: i32) {
        let mut current = &mut self.0;

        while let &mut Some(ref mut next_item) = current {
            current = &mut next_item.1;
        }

        let item = Item(value, None);
        *current = Some(Box::new(item));
    }
}

#[test]
fn test_append() {
    let mut list = List::new();
    list.append(1);
    list.append(2);

    println!("{:?}", list);
}
