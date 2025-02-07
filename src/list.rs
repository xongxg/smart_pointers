#[derive(Debug)]
struct Item(i32, Option<Box<Item>>);

#[derive(Debug)]
struct List(Option<Box<Item>>);

impl List {
    fn new() -> List {
        List(None)
    }

    // fn append(&mut self, value: i32) {
    //     let mut current = &mut self.0;
    //
    //     while let Some(ref mut item) = current {
    //         current = &mut item.1;
    //     }
    // }
}
