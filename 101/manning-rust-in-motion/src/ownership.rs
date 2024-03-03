pub fn simple() {
    let a = String::from("hello");
    println!("I say, {} ownership simple!", a); // a will get out of scope after this line and the memory will be deallocated
}

pub fn transferred() {
    let a = String::from("hello");
    say(a);
    // print!("{}", a); will cause error
}

pub fn clonning() {
    let a = String::from("hello");
    say(a.clone());
    println!("I say, {} ownership clonned!", a);
}

pub fn pluralizing() {
    let s = String::from("book");
    println!("I have one {}, you have two {}", s, pluralize(s.clone()))
}

fn say(s: String) {
    println!("I say, {} ownership transferred!", s);
}

fn pluralize(s: String) -> String {
    s + "s"
}
