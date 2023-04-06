#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Chudi";
    let age = 21;
    let chudi = Person { name, age };

    // Pretty print
    println!("{:#?}", chudi);
}
