pub struct Book {
    sban: u32,
    name: String,
    authors: Vec<String>,
    price: f32
}

impl Book {
    pub fn new(sban: u32, name: String, authors: String, price: f32) -> Book {
        let author_split: Vec<&str> = authors.split(',')
                                             .collect();

        let auth: Vec<String> = author_split.iter()
                                            .map(|s| {
                                                let s: String = s.to_string();
                                                s
                                            })
                                            .collect();

        Book {
            sban: sban,
            name: name,
            authors: auth,
            price: price
        }
    }

    pub fn add_author(&mut self, author: String) {
        self.authors.push(author);
    }
}


fn main() {
    let mut book = Book::new(123, "Rust".to_string(),
                             "Thulasiram,Aditya".to_string(), 21.0);

    book.add_author("Gopi".to_string());

    println!("Authors of book {} are {:?}", book.name, book.authors);
}