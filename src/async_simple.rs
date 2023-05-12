// This code is largely from the async tutorial @ https://rust-lang.github.io/async-book
use futures::executor::block_on;
use rand::Rng;
use std::thread;

async fn do_something() {
    println!("What do you call a condescending bear? A pan-DUH.");
}

async fn think_of_number() -> u8 {
    rand::thread_rng().gen()
}

async fn do_a_flip(squeak: &str) {
    for _i in 1..think_of_number().await {
        print!("{}! ", squeak);
        thread::sleep_ms(1);

    }
    print!("Tada!\n");
}

async fn async_main() {
    let future = do_something(); // Nothing is printed here. It is all packaged in this future
                                 // call.
    println!("Look, ma! A future to an asynchronous function!");
    //block_on(future); // 'future' is run. You can think of this like a .join() in traditional
                      // threading.

    let flip1 = do_a_flip("flip");
    let flip2 = do_a_flip("flop");
    let flip3 = do_a_flip("flap");
    let flip4 = do_a_flip("flup");
    let flip5 = do_a_flip("floup");
    futures::join!(future, flip1, flip2, flip3, flip4, flip5); // Like .await, but activates and waits for multiple
                                  // futures concurrently
}

fn main() {
    block_on(async_main());
}
