use std::io;
use std::io::ErrorKind;
use async_trait::async_trait;
use tokio::sync::{mpsc, oneshot};
use tokio::task::spawn;
use tokio::time::{sleep, Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};
use futures::future::join_all;
use std::time::Instant;


#[tokio::main]
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










    // 54.3 Async Traits
    // Async methods in traits are not yet supported in the stable channel.
    // The crate async_trait provides a workaround through a macro:

    #[async_trait]
    trait Sleeper {
        async fn sleep(&self);
    }

    struct FixedSleeper {
        sleep_ms: u64,
    }

    #[async_trait]
    impl Sleeper for FixedSleeper {
        async fn sleep(&self) {
            sleep(Duration::from_millis(self.sleep_ms)).await;
        }
    }

    async fn run_all_sleepers_multiple_times(sleepers: Vec<Box<dyn Sleeper>>, n_times: usize) {
        for _ in 0..n_times {
            println!("running all sleepers..");
            for sleeper in &sleepers {
                let start = Instant::now();
                sleeper.sleep().await;
                println!("slept for {}ms", start.elapsed().as_millis());
            }
        }
    }

    let sleepers: Vec<Box<dyn Sleeper>> = vec![
        Box::new(FixedSleeper { sleep_ms: 50}),
        Box::new(FixedSleeper {sleep_ms: 100}),
    ];
    run_all_sleepers_multiple_times(sleepers, 5).await;

    // async_trait is easy to use, but note that it’s using heap allocations to achieve this.
    // This heap allocation has performance overhead.

    // The challenges in language support for async trait are deep Rust and probably not worth
    // describing in-depth. Niko Matskis did a good job of explaning them in [this post](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/)
    // if you are interested in diggin deeper.
















    // 54.5 Cancellation

    // Dropping a future implies it can nver be polled again. This is called cancellation and it can
    // occur at any await point. Care is needed to ensure the system works correctly even when futures
    // are cancelled. For example, it shouldn't deadlock or lose data.

    struct LinesReader {
        stream: DuplexStream,
    }

    impl LinesReader {
        fn new(stream: DuplexStream) -> Self {
            Self { stream }
        }

        async fn next(&mut self) -> io::Result<Option<String>> {
            let mut bytes = Vec::new();
            let mut buf = [0];
            while self.stream.read(&mut buf[..]).await? != 0 {
                bytes.push(buf[0]);
                if buf[0] == b'\n' {
                    break;
                }
            }
            if bytes.is_empty() {
                return Ok(None)
            }

            let s = String::from_utf8(bytes)
                .map_err(|_| io::Error::new(ErrorKind::InvalidData, "not UTF-8"))?;

            Ok(Some(s))
        }
    }

    async fn slow_copy(source: String, mut dest: DuplexStream) -> std::io::Result<()> {
        for b in source.bytes() {
            dest.write_u8(b).await?;
            tokio::time::sleep(Duration::from_millis(10)).await
        }
        Ok(())
    }
    
    async fn do_main() -> std::io::Result<()> {
        let (client, server) = tokio::io::duplex(5);
        let handle = tokio::spawn(slow_copy("hi\nthere\n".to_owned(), client));

        let mut lines = LinesReader::new(server);
        let mut interval = tokio::time::interval(Duration::from_millis(60));
        loop {
            tokio::select! {
            _ = interval.tick() => println!("tick!"),
            line = lines.next() => if let Some(l) = line? {
                print!("{}", l)
            } else {
                break
            },
        }
        }
        handle.await.unwrap()?;
        Ok(())
    }
    

    do_main().await.expect("TODO: panic message");
    // The comipler doesn't help with cancellation safety. You need to read API documentation and
    // consider what state your async fn holds.

    // Unlike panic and ?, cancellation is part of normal control flow (vs error-handling).


    // LinesReader can be made cancellation-safe by makeing buf part of the struct:

    // Interval::tick is cancellation-safe because it keeps track of whether a tick has been ‘delivered’.
    //
    // AsyncReadExt::read is cancellation-safe because it either returns or doesn’t read data.
    //
    // AsyncBufReadExt::read_line is similar to the example and isn’t cancellation-safe. See its documentation for details and alternatives.
}
