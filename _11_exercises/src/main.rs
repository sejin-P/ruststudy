struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        todo!("Initialize and return a `Library` value")
    }

    //fn len(self) -> usize {
    //    todo!("Return the length of `self.books`")
    //}

    //fn is_empty(self) -> bool {
    //    todo!("Return `true` if `self.books` is empty")
    //}

    //fn add_book(self, book: Book) {
    //    todo!("Add a new book to `self.books`")
    //}

    //fn print_books(self) {
    //    todo!("Iterate over `self.books` and each book's title and year")
    //}

    //fn oldest_book(self) -> Option<&Book> {
    //    todo!("Return a reference to the oldest book (if any)")
    //}
}

// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
fn main() {
    let library = Library::new();

    //println!("The library is empty: {}", library.is_empty());
    //
    //library.add_book(Book::new("Lord of the Rings", 1954));
    //library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    //
    //println!("The library is no longer empty: {}", library.is_empty());
    //
    //
    //library.print_books();
    //
    //match library.oldest_book() {
    //    Some(book) => println!("The oldest book is {}", book.title),
    //    None => println!("The library is empty!"),
    //}
    //
    //println!("The library has {} books", library.len());
    //library.print_books();
}