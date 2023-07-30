use futures::executor::block_on;
use tokio::time;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count is: {i}");
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

async fn count_to_with_tokio(count: i32) {
    for i in 1..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
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










    // 52.2 Futures
    // Future is a trait, implemented by objects that represent an operation that may not be complete yet.
    // A future can be polled, and poll returns a Poll.

    // The .await keyword, applied to a Future, causes the current async function to pause until that
    // Future is ready, and then evaluates to its output.

    // Context와 Pin은 나중에 설명.
    // Briefly:
    // Context allows a Future to schedule itself to be polled again when an event occurs.
    // Pin ensures that the Future isn't moved in memory, so that pointers into that future remain valid.
    // This is required to allow references to remain valid after an .await.







    // 52.3 Runtimes
    // A runtime provides support for performing operations asynchronously (a reactor) and is responsible
    // for executing futures (an executor). Rust does not have a 'built-in' runtime, but several options
    // are available.

    // [Tokio](https://tokio.rs/): performant, with a well-developed ecosystem of functionality like
    // [Hyper](https://hyper.rs/) for HTTP or [Tonic](https://github.com/hyperium/tonic) for gRPC.

    // [async-std](https://async.rs/): aims to be a "std for async", and includes a basic runtime in
    // async::task.

    // [smol](https://docs.rs/smol/latest/smol/): simple and lightweight.





    // 52.3.1 Tokio

    // Tokio provides:

    // A multi-threaded runtime for executing asynchronous code.
    // An asynchronous version of the standard library.
    // A large ecosystem of libraries.

    // #[tokio::main]
    // async fn main() {
    //     tokio::spawn(count_to_with_tokio(10));
    //
    //     for i in 1..5 {
    //         println!("Main task: {i}");
    //         time::sleep(time::Duration::from_millis(5)).await;
    //     }
    // }








    // 52.4 Tasks is in _52_4_Tasks

}


