mod concurrency;

fn main() {
    println!("\n*******************CONCURRENCY**\n");
    concurrency::threading();
    concurrency::channels();
    concurrency::shared_state();

    println!("\nDone!\n");
}
