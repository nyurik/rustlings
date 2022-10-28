// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

#[must_use]
fn foo() -> String {
    String::from("foo")
}

// fn book<'a>(title: &'a str) -> Book<'a> {
//     Book { author: &foo(), title: &title }
//
// }

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &foo(), title: &title };

    println!("{} by {}", book.title, book.author);
}
