#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate error_chain;

mod borrowing;
mod error_handling;
mod lifetimes;
mod ownership;

fn main() {
    println!("\n***ownership***\n");
    ownership::simple();
    ownership::transferred();
    ownership::clonning();
    ownership::pluralizing();

    println!("\n***borrowing***\n");
    borrowing::pluralizing();
    borrowing::congratulations();
    borrowing::mut_borrow();
    borrowing::temp_var();

    println!("\n***error handling***\n");
    error_handling::handle_result();
    error_handling::box_error();
    if let Err(e) = error_handling::create_document("filename") {
        println!("{}", e)
    }
    if let Err(e) = error_handling::create_document_quick_error("filename") {
        println!("{}", e)
    }
    if let Err(e) = error_handling::create_document_error_chain("filename") {
        println!("{}", e)
    }
    if let Err(e) = error_handling::create_document_failure_derive("filename") {
        println!("{}", e)
    }

    println!("\n***lifetimes***\n");
    lifetimes::concrete_lifetimes();
    lifetimes::generic_lifetimes();
    println!("generic {:?}", lifetimes::simulate_game("fork", "break"));
    println!(
        "longer book {}",
        lifetimes::title_of_longer(
            &lifetimes::Book {
                title: "Book 1".to_owned(),
                pages: 2
            },
            &lifetimes::Book {
                title: "Book 2".to_owned(),
                pages: 5
            }
        )
    );
    println!("static lftms {:?}", lifetimes::return_first_two_static());

    println!("\nEnd");
}
