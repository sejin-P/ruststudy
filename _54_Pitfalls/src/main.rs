use tokio::sync::{mpsc, oneshot};
use tokio::task::spawn;
use tokio::time::{sleep, Duration};
use futures::future::join_all;
use std::time::Instant;


#[tokio::main(flavor="current_thread")]
async fn main() {
    // 54.1 Blocking the Executor
    // Most async runtimes only allow IO tasks to run concurrently. This means that CPU blocking tasks
    // will block the executor and prevent other tasks from being executed. An easy workaround is to
    // use async equivalent methods where possible.

    async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
        // std::thread::sleep(std::time::Duration::from_millis(duration_ms));
        sleep(std::time::Duration::from_millis(duration_ms)).await;
        println!(
            "future {id} slept for {duration_ms}ms, finished after {}ms",
            start.elapsed().as_millis(),
        );
    }

    let start = Instant::now();
    let sleep_futures = (1..=10).map(|t| sleep_ms(&start, t, t*10));
    join_all(sleep_futures).await;

    // Run the code and see that the sleeps happen consecutively rather than concurrently.
    // The "current_thread" flavor puts all tasks on a single thread. This makes the effect more obvious, but the bug is still present in the multi-threaded flavor.
    // Switch the std::thread::sleep to tokio::time::sleep and await its result.
    // Another fix would be to tokio::task::spawn_blocking which spawns an actual thread and transforms
    // its handle into a future without blocking the executor.

    // You should not think of tasks as OS threads. They do not map 1 to 1 and most executors will allow
    // many tasks to run on a single OS thread. This is particularly problematic when interacting with
    // other libraries via FFI, where that library might depend on thread-local storage or map to specific
    // OS threads (e.g., CUDA). Prefer tokio::task::spawn_blocking in such situations.












    // 54.2 Pin
    // When you await a future, all local variables (that would ordinarily be stored on a stack frame)
    // are instead stored in the Future for the current async block.
    // If your future has pointers to data on the stack, those pointers might get invalidated. This is unsafe.

    // Therefore, you must guarantee that the address your future points to don't change. That is why
    // we need to pin futures. Using the same future repeatedly in a select! often leads to issues with
    // pinned values.

    // A work item. In this case, just sleep for the given time and respond
    // with a message on the `respond_on` channel.
    #[derive(Debug)]
    struct Work {
        input: u32,
        respond_on: oneshot::Sender<u32>,
    }

    // A worker which listens for work on a queue and performs it.
    async fn worker(mut work_queue: mpsc::Receiver<Work>) {
        let mut iterations = 0;
        loop {
            tokio::select! {
                Some(work) = work_queue.recv() => {
                    sleep(Duration::from_millis(10)).await; // Pretend to work
                    work.respond_on.send(work.input * 10000).expect("failed to send response");
                    iterations += 1;
                }
            }
        }
    }

    // A requester which requests work and waits for it to complete.
    async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> u32 {
        let (tx, rx) = oneshot::channel();
        work_queue.
            send(Work { input, respond_on: tx}).await
            .expect("failed to send on work queue");
        rx.await.expect("failed waiting for response")
    }

    let (tx, rx) = mpsc::channel(10);
    spawn(worker(rx));
    for i in 1..100 {
        let resp = do_work(&tx, i).await;
        println!("work result for iteration {i}: {resp}");
    }
}
