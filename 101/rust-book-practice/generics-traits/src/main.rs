mod traits;

fn main() {
    println!("Hello generics world!");
    println!("largest number with normal function: {}", largest_number());
    println!(
        "largest number with generic function {}",
        largest_number_generics(&vec![2, 1, 5, 43, 12]).unwrap()
    );
}

fn largest_number() -> i32 {
    let number_list = vec![2, 1, 5, 43, 12];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number
        }
    }
    largest
}

fn largest_number_generics(number_list: &[i32]) -> Option<&i32> {
    let mut largest = match number_list.get(0) {
        Some(n) => n,
        None => return None,
    };
    for number in number_list {
        if number > largest {
            largest = number
        }
    }
    Some(largest)
}

fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut largest = match list.get(0) {
        Some(n) => n,
        None => return None,
    };
    for number in list {
        if number > largest {
            largest = number
        }
    }
    Some(largest)
}
