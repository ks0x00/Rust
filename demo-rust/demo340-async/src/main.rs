use crate::block_on_test::test_world;

fn main() {
    test_world();
}

mod block_on_test {
    use std::thread;

    // `block_on` blocks the current thread until the provided future has run to
    // completion. Other executors provide more complex behavior, like scheduling
    // multiple futures onto the same thread.
    use futures::executor::block_on;

    pub fn test_world() {
        let future = hello_world(); // Nothing is printed
        // block_on(future); // `future` is run and "hello, world!" is printed
        block_on(future);
        // block_on() blocks the main thread during 2 seconds.
        println!("have a nice day.");
        let future_world = get_new_world();
        let world = block_on(future_world);
        println!("{world}");
    }

    async fn hello_world() {
        println!("hello, world!");
        thread::sleep(std::time::Duration::from_millis(2000));
    }
    async fn get_new_world() -> String {
        "new world".to_owned()
    }
}
