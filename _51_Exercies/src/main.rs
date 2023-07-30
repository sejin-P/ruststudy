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

// struct Philosopher {
//     name: String,
//     // ANCHOR_END: Philosopher
//     left_fork: Arc<Mutex<Fork>>,
//     right_fork: Arc<Mutex<Fork>>,
//     thoughts: mpsc::SyncSender<String>,
// }
//
// // ANCHOR: Philosopher-think
// impl Philosopher {
//     fn think(&self) {
//         self.thoughts
//             .send(format!("Eureka! {} has a new idea!", &self.name))
//             .unwrap();
//     }
//     // ANCHOR_END: Philosopher-think
//
//     // ANCHOR: Philosopher-eat
//     fn eat(&self) {
//         // ANCHOR_END: Philosopher-eat
//         println!("{} is trying to eat", &self.name);
//         let left = self.left_fork.lock().unwrap();
//         let right = self.right_fork.lock().unwrap();
//
//         // ANCHOR: Philosopher-eat-end
//         println!("{} is eating...", &self.name);
//         thread::sleep(Duration::from_millis(10));
//     }
// }

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
        let mut l = Arc::clone(&forks[idx]);
        let mut r = Arc::clone(&forks[(idx+1)%5]);

        if idx == 4 {
            std::mem::swap(&mut l, &mut r);
        }
        thread::spawn(move || {
            for i in 0..100 {
                p.think();
                p.eat(&l, &r);
            }
        });
    }
    // Output their thoughts

    drop(tx);
    for thought in rx {
        println!("thought");
        println!("{thought:?}");
    }

    // ANCHOR_END: Philosopher-eat-end
    // let (tx, rx) = mpsc::sync_channel(10);
    //
    // let forks = (0..PHILOSOPHERS.len())
    //     .map(|_| Arc::new(Mutex::new(Fork)))
    //     .collect::<Vec<_>>();
    //
    // for i in 0..forks.len() {
    //     let tx = tx.clone();
    //     let mut left_fork = Arc::clone(&forks[i]);
    //     let mut right_fork = Arc::clone(&forks[(i + 1) % forks.len()]);
    //
    //     // To avoid a deadlock, we have to break the symmetry
    //     // somewhere. This will swap the forks without deinitializing
    //     // either of them.
    //     if i == forks.len() - 1 {
    //         std::mem::swap(&mut left_fork, &mut right_fork);
    //     }
    //
    //     let philosopher = Philosopher {
    //         name: PHILOSOPHERS[i].to_string(),
    //         thoughts: tx,
    //         left_fork,
    //         right_fork,
    //     };
    //
    //     thread::spawn(move || {
    //         for _ in 0..100 {
    //             philosopher.eat();
    //             philosopher.think();
    //         }
    //     });
    // }
    //
    // drop(tx);
    // for thought in rx {
    //     println!("{thought}");
    // }
}
