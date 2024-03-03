use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    let t = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify_with_trait_bound_syntax(t);
    println!("Breaking news ! {}", item.summarize())
}

pub fn notify_with_trait_bound_syntax<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify_multiple_bounds<T: Summary + Display>(item: T) {
pub fn notify_multiple_bounds(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

// fn some_function<T, U>(t: T, u: U) -> i32
// where T: Display + Clone,
// U: Clone + Debug
// {
