use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Option<i32>,
    right_fork: Option<i32>,
    thoughts: mpsc::SyncSender<String>
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&mut self, left: &Arc<Mutex<i32>>, right: &Arc<Mutex<i32>>) {
        let l = left.lock().unwrap();
        let r = right.lock().unwrap();
        self.left_fork = Option::from(*l);
        self.right_fork = Option::from(*r);
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
        self.left_fork = None;
        self.right_fork = None;
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks

    let forks = vec![
        Arc::new(Mutex::new(0)),
        Arc::new(Mutex::new(1)),
        Arc::new(Mutex::new(2)),
        Arc::new(Mutex::new(3)),
        Arc::new(Mutex::new(4))
    ];
    // Create philosophers
    let (tx, rx) = mpsc::sync_channel(10);

    // Make each of them think and eat 100 times

    for idx in 0..5 {
        let tx = tx.clone();
        let mut p = Philosopher {
            name: PHILOSOPHERS[idx].to_string(),
            left_fork: None,
            right_fork: None,
            thoughts: tx,
        };
        let l = forks[idx].clone();
        let r = forks[(idx+1)%5].clone();
        thread::spawn(move || {
            for i in 0..100 {
                p.think();
                p.eat(&l, &r);
            }
        });
    }
    // Output their thoughts

    drop(tx);
    for thought in rx.iter() {
        println!("{thought:?}");
    }
}
