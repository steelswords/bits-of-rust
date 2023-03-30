// This code is largely from the async tutorial @ https://rust-lang.github.io/async-book
use futures::executor::block_on;

async fn do_something() {
    println!("What do you call a condescending bear? A pan-DUH.");
}

async unsafe fn think_of_number() -> u8 {
    static mut number: u8 = 1;
    number += 1;
    number
}

async unsafe fn do_a_flip() {
    for i in 1..think_of_number().await {
        print!("flip! ");
    }
    print!("Tada!\n");
}

async unsafe fn main() {
    let future = do_something(); // Nothing is printed here. It is all packaged in this future
                                 // call.
    println!("Look, ma! A future to an asynchronous function!");
    block_on(future); // 'future' is run. You can think of this like a .join() in traditional
                      // threading.

    let flip1 = do_a_flip();
    let flip2 = do_a_flip();
    futures::join!(flip1, flip2); // Like .await, but activates and waits for multiple
                                  // futures concurrently
}
