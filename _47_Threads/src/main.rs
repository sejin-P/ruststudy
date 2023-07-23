use std::sync::mpsc;
use std::thread;
use std::thread::scope;
use std::time::Duration;

fn main() {
    // 47 Threads
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_secs(5));
    }

    // Threads are all daemon threads, the main thread does not wait for them.
    // Thread panics are independent of each other.
    // Panics can carray a payload, which can be unpacked with downcast_ref.

    // Notice that the threads is stopped before it reaches 10 - the main thread is not waiting.
    // Use let handle = thread::spawn(...) and later handler.join() to wait for the thread to finish.
    // Trigger a panic in the thread, notice how this doesn't affect `main`.
    // Use the `Result` return value from `handle.join()` to get access to the panice payload. This
    // is a good time to talk about Any.

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_secs(1));
        }
    });
    handle.join().expect("should wait this thread");
    println!("i waited!");









    // 47.1 Scoped Threads
    // Normal threads cannot borrow from their environment:
    let s = String::from("Hello");
    // thread::spawn(|| {
    //     println!("Length: {}", s.len());
    // });

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });

    // The reason for that is that when the thread::scope function completes, all the threads are
    // guaranteed to be joined, so they can return borrowed data.
    // Normal Rust borrowing rules apply: you can either borrow mutably by one thread, or immutably
    // by any number of threads.







    // 47.2 Channels
    // Rust channels have two parts: a `Sender<T>` and a `Receiver<T>`. The two parts are connected
    // via the channel, but you only see the end-points.

    // it looks similar with golang chan.
    let (tx, rx) = mpsc::channel();
    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());

    // `mpsc` stands for Multi-Producer, Single-Consumer. `Sender` and `SyncSender` implement
    // Clone (so you can make multiple producers) but `Receiver` does not.
    // `send()` and `recv()` return `Result`. If they return `Err`, it means the counterpart `Sender`
    // or `Receiver` is dropped and the channel is closed.
}
