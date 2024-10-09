use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).unwrap();
    let mut index = 0;

    for book in books {
        write!(file, "{}|{}|{}", book.title, book.author, book.year).unwrap();

        index += 1;

        if index != books.len() {
            write!(file, "\n").unwrap();
        }
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut books: Vec<Book> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split('|').collect();
        let book = Book {
            title: String::from(tokens[0]),
            author: String::from(tokens[1]),
            year: (*tokens[2]).parse::<u16>().unwrap(),
        };

        books.push(book);
    }
    
    return books;
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}