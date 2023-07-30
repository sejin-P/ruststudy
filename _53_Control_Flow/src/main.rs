use anyhow::Result;
use futures::future;
use reqwest;
use std::collections::HashMap;
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 53.1 Join
    // A join operation waits until all of a set of futures are ready, and returns a collection of their
    // results. This is similar to Promise.all in JavaScript or asyncio.gather in Python.

    async fn size_of_page(url: &str) -> Result<usize> {
        let resp = reqwest::get(url).await?;
        Ok(resp.text().await?.len())
    }

    let urls: [&str; 4] = [
        "https://google.com",
        "https://httpbin.org/ip",
        "https://play.rust-lang.org/",
        "BAD_URL",
    ];

    let futures_iter = urls.into_iter().map(size_of_page);
    let results = future::join_all(futures_iter).await;
    let page_size_dict: HashMap<&str, Result<usize>> =
        urls.into_iter().zip(results.into_iter()).collect();
    println!("{:?}", page_size_dict);

    // For multiple futures of disjoint types, you can use std::future::join! but you must know how many
    // futures you will have at compile time. This is currently in the futures crate, soon to be stabilised
    // in std::future.

    // The risk of join is that one of the futures may never resolve, this would cause your program
    // to stall.

    // You can also combine join_all with join! for instance to join all requests to an http service
    // as well as a database query. Try adding a tokio::time::sleep to the future, using futures::join!.
    // This is not a timeout(that requires select!, explained in the next chapter), but demonstrates
    // join.











    // 53.2 Select
    // A select operation waits until any of a set of futres is ready, adn responds to that future's
    // result. In js, this is similar to Promise.race. In python, it compares to asyncio.wait(task_set,
    // return_when=asyncio.FIRST_COMPLETED).

    // Similar to a match statement, the body of select! has a numvber of arms, each of the form
    // pattern = future => statement. When the future is ready, the statement is executed with the variables
    // in pattern bound to the future's result.

    #[derive(Debug, PartialEq)]
    enum Animal {
        Cat { name: String },
        Dog { name: String },
    }

    async fn first_animal_to_finish_race(
        mut cat_rcv: Receiver<String>,
        mut dog_rcv: Receiver<String>,
    ) -> Option<Animal> {
        tokio::select! {
            cat_name = cat_rcv.recv() => Some(Animal::Cat { name: cat_name?}),
            dog_name = dog_rcv.recv() => Some(Animal::Dog { name: dog_name?}),
        }
    }

    let (cat_sender, cat_receiver) = mpsc::channel(32);
    let (dog_sender, dog_receiver) = mpsc::channel(32);

    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        cat_sender.send(String::from("Felix")).await.expect("Failed to send cat.");
    });

    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        dog_sender.send(String::from("rex")).await.expect("failed to send dog");
    });

    let winner = first_animal_to_finish_race(cat_receiver, dog_receiver)
        .await
        .expect("failed to receive winner");

    println!("Winner is {winner:?}");
}
