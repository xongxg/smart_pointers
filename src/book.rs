struct Author<'a> {
    name: &'a str,
}

impl<'a> Author<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

struct Book<'a> {
    author: Author<'a>,
    title: &'a str,
    publication_date: i32,
}

impl<'a> Book<'a> {
    pub fn new(author: Author<'a>, title: &'a str, publication_date: i32) -> Book<'a> {
        Book {
            author,
            title,
            publication_date,
        }
    }

    pub fn display(&self) {
        println!(
            "{} ({}) by {}",
            self.title, self.publication_date, self.author.name
        );
    }
}

#[test]
fn test_book() {
    let author_name = "Maya Angelou";
    let author = Author::new(&author_name);

    let book_title = "I Know Why the Caged Bird Sings";

    let book = Book::new(author, &book_title, 1969);
    book.display();
    let author_name2 = "Chimamanda Ngozi Adichie";
    let author2 = Author::new(&author_name2);

    let book_title2 = "Americanah";

    let book2 = Book::new(author2, &book_title2, 2013);
    book2.display();
}
