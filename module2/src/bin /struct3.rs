//Topic: Getting used to with Struct
//
//Requirements
//
/*
1- Define a struct Book having two fields author:Author and price:i32,
2- Define a struct Author having two fields one is name and stars,
3- you gonna purchase a book only if author has rating >= 4.9 stars .

*/
#[derive(Debug)]
struct Book {
    author: Author,
    price: i32,
}
#[derive(Debug)]
struct Author {
    name: String,
    stars: f32,
}
fn check_buy(book: Book) {
    if book.author.stars >= 4.9 {
        println!(
            "{:?}'s book can be bought and the price is {:?}",
            book.author.name.trim(),
            book.price
        );
    } else {
        println!("not upto the mark");
    }
}
fn main() {
    let mut buf = String::new();
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    let name = buf;
    let stars = buf1.trim().parse::<f32>().unwrap();
    let price = buf2.trim().parse::<i32>().unwrap();
    let auth = Author {
        name: name,
        stars: stars,
    };
    let book = Book {
        author: auth,
        price: price,
    };
    check_buy(book);
}
