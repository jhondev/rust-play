struct Person {
    name: String,
}

pub fn congratulations() {
    let p = Person {
        name: String::from("Jake"),
    };
    congratulate(&p);
}

fn congratulate(person: &Person) {
    println!("Congratulations borrowing, {}!!", person.name);
}

// **********************************

pub fn pluralizing() {
    let s = String::from("book");
    println!(
        "I have one {}, you have two {}. borrowing",
        s,
        pluralize(&s)
    )
}

fn pluralize(s: &String) -> String {
    format!("{}s", s)
}

// ***********************************
pub fn mut_borrow() {
    let mut list = vec![1, 2, 3];
    *list.first_mut().expect("empty list") += 1;

    let list_first = list.first();
    let list_last = list.last();

    println!(
        "The first element is {:?} and the last is {:?}",
        list_first, list_last
    );
}

// **************************************
pub struct Player {
    score: i64,
}

impl Player {
    pub fn set_score(&mut self, new_score: i64) {
        self.score = new_score
    }

    pub fn score(&self) -> i64 {
        self.score
    }

    pub fn new() -> Self {
        Player { score: 0 }
    }
}

pub fn temp_var() {
    let mut player1 = Player::new();
    // new rust versions dont need temp var to calculate borrowed result
    player1.set_score(player1.score() + 1)
}
