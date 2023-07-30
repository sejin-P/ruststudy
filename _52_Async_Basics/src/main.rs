use futures::executor::block_on;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count is: {i}");
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

fn main() {
    // “Async” is a concurrency model where multiple tasks are executed concurrently by executing each
    // task until it would block, then switching to another task that is ready to make progress.
    // The model allows running a larger number of tasks on a limited number of threads. This is because
    // the per-task overhead is typically very low and operating systems provide primitives for efficiently
    // identifying I/O that is able to proceed.
    //
    // Rust’s asynchronous operation is based on “futures”, which represent work that may be completed
    // in the future. Futures are “polled” until they signal that they are complete.
    //
    // Futures are polled by an async runtime, and several different runtimes are available.

    // Python's asyncio and js's Promise are similar to Rust's future, but they are callback-based.






    // 52.1 Async/Await
    block_on(async_main(10));

    let future: () = async_main(10);

    // You cannot make main async, without additional instructions to the compiler on how to use the returned future.
    // You need an executor to run async code. block_on blocks the current thread until the provided
    // future has run to completion.
    // `.await` asynchronously waits for the completion of another operation. Unlike block_on,
    // `.await` doesn’t block the current thread.
    // .await can only be used inside an async function (or block; these are introduced later).
}
