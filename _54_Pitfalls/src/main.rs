use tokio;
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
        tokio::time::sleep(std::time::Duration::from_millis(duration_ms)).await;
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
}
