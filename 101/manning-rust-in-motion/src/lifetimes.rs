//*******************************************CONCRETE LIFETIMES */
/* A reference's lifetime must be contained within the lifetime of the value being referenced.
so always point to a valid value */
pub fn concrete_lifetimes() {
    let list = vec![100, 34, 72, 55];
    let slice = &list;
    let first_two = get_first_two(slice);
    println!("first 2 are {:?}", first_two);
    println!("lists is {:?}", slice);
    println!("again, first 2 are {:?}", first_two);
}

fn get_first_two(borrowed_list: &Vec<i32>) -> &[i32] {
    &borrowed_list[..2]
}

// fn concrete_lifetime_error() {
//     let first_two = {
//         let list = vec![100, 34, 72, 55];
//         &list
//     };

//     print!("First 2 are {:?}", first_two);
// }

// fn concrete_lifetime_moved_error() {
//     let list1 = vec![100, 34, 72, 55];
//     let list2 = list1;
//     let slice = &list1[..2];

//     print!("First 2 are {:?}", slice);
// }

// ????
// fn borrow_error() -> &str {
//   let n = String::from("name");
//   &n
// }

//*******************************************GENERIC LIFETIMES */
/* missing lifetime specifier
this function's return type contains a borrowed value,
but the signature does not say whether it is borrowed from `home` or `away` */
// This is a generic lifetime that relates the two input reference's lifetimes to the output reference's lifetime.
// We need to tell the compiler about this relationship.
// fn simulate_game(home: &str, away: &str) -> &str {
//     if away == "break" {
//         println!("is not home btw");
//     }
//     home
// }
// solution
pub fn simulate_game<'a>(home: &'a str, away: &str) -> &'a str {
    if away == "break" {
        println!("is not home btw");
    }
    home
}

// what if we use multiple lifetime annotation
// this parameter and the return type are declared with different lifetimes
// fn simulate_game2<'a, 'b>(home: &'a str, away: &'b str) -> &'a str {
//     if rand::random() {
//         away
//     } else {
//         home
//     }
// }

pub fn generic_lifetimes() {
    let home = "ve";
    let away = "break";
    let result = simulate_game2(home, away);
    println!("generic lifetimes: {}", result);
}

fn simulate_game2<'a>(home: &str, away: &str) -> &'a str {
    if away == "break" {
        "arand"
    } else if home == "continue" {
        "home"
    } else {
        "unknown"
    }
}

//* */
pub struct Book {
    pub title: String,
    pub pages: i32,
}
pub fn title_of_longer<'a>(book1: &'a Book, book2: &'a Book) -> &'a str {
    if book1.pages > book2.pages {
        &book1.title
    } else {
        &book2.title
    }
}

//* static lifetimes */
static LIST: [i32; 4] = [100, 34, 72, 55];
pub fn return_first_two_static() -> &'static [i32] {
    &LIST[..2]
}

// TODO: overconstrained examples
