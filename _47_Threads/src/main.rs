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
        thread::sleep(Duration::from_secs(1));
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







    // 48 Channels
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













    // 48.1 Unbounded Channels
    // You get an unbounded and asynchronous channel with mpsc::channel();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("message {i}")).unwrap();
            println!("{thread_id:?}: sent message {i}");
        }
        println!("{thread_id:?} done");
    });

    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }





    println!("{:?}", rx);




    // 48.2 Bounded Channels
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }

    // Calling send will block the current thread until there is space in the channel for the new message. The thread can be blocked indefinitely if there is nobody who reads from the channel.
    // A call to send will abort with an error (that is why it returns Result) if the channel is closed. A channel is closed when the receiver is dropped.
    // A bounded channel with a size of zero is called a “rendezvous channel”. Every send will block the current thread until another thread calls read.











    // 49 Send and Sync
    // How does Rust know to forbid shared access across thread? The answer is in two traits:
    // `Send`: a type T is `Send` if it is safe to move a `T` across a thread boundary.
    // `Sync`: a type T is `Sync` if it is safe to move a &T across a thread boundary.
    // `Send` and `Sync` are unsafe traits. The compiler will automatically derive them for your types
    // as long as they only contain `Send` and `Sync` types. You can also implement them manually
    // when you know it is valid.



    // 49.1 Send
    // The effect of moving ownership to another thread is that destructors will run in that thread.
    // So the question is when you can allocate a value in one thread and deallocate it in another.
    // As an example, a connection to the SQLite library must only be accessed from a single thread.

    // 49.2 Sync
    // T is `sync` if an only if &T is `Send`
    // This statement is essentially a shorthand way of saying that if a type is thread-safe for shared use, it is also thread-safe to pass references of it across threads.
    //
    // This is because if a type is Sync it means that it can be shared across multiple threads without
    // the risk of data races or other synchronization issues, so it is safe to move it to another thread.
    // A reference to the type is also safe to move to another thread, because the data it references
    // can be accessed from any thread safely.












    // 50 Shared State
    // Rust uses the type system to enforce synchronization of shared data. This is primarily done
    // via two types:

    // `Arc<T>`, atomic reference counted T: handles sharing between threads and takes care to dealloc
    // T when the last reference is dropped.

    // `Mutex<T>`: ensures mutually exclusive access to the T value.
}
