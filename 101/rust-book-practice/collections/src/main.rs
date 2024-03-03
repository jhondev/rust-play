fn main() {
    reading_vectors();
}

fn creating_vector() {
    let v: Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3];
    v1.push(4);
}

fn reading_vectors() {
    let v = vec![1, 2, 3, 4, 5];
    let third = v[2];
    println!("The third element is {}.", third);

    match v.get(3) {
        Some(third) => println!("The third element is {}.", third),
        None => println!("There's no third element."),
    }
}

fn multiple_types_in_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}

fn string_collection() {
    let data = "initial contents";
    let s = data.to_string();
    let s = String::from("initial contents");
}

fn reading_string() {
    let s = "Здравствуйте";
    let c = s.chars();
}

fn creating_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec!["Blue", "Yellow"];
    let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let zipped_scores = teams.iter().zip(initial_scores.iter());
    let scores: HashMap<_, _> = zipped_scores.collect();
}
