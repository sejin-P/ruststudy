use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use reqwest::{blocking::Client, Url};
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("bad http response: {0}")]
    BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand {
    url: Url,
    extract_links: bool,
}

fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
    println!("Checking {:#}", command.url);
    let response = client.get(command.url.clone()).send()?;
    if !response.status().is_success() {
        println!("error!");
        return Err(Error::BadResponse(response.status().to_string()));
    }

    let mut link_urls = Vec::new();
    if !command.extract_links {
        return Ok(link_urls);
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    for href in href_values {
        match base_url.join(href) {
            Ok(link_url) => {
                link_urls.push(link_url);
            }
            Err(err) => {
                println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
            }
        }
    }
    Ok(link_urls)
}

fn main() {
    let thread_count = 4;

    let (tx, rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();

    let rx = Arc::new(Mutex::new(rx));
    for i in 0..thread_count {
        let i = i;
        let rx = rx.clone();
        let result_tx = result_tx.clone();
        thread::spawn(move || {
            let client = Client::builder().connect_timeout(Duration::from_secs(2)).build().unwrap();
            loop {
                let l = rx.lock().unwrap();
                let url = l.recv();
                match url {
                    Ok(url) => {
                        let command = CrawlCommand{url, extract_links: true};
                        match visit_page(&client, &command) {
                            Ok(links) => {
                                result_tx.send((links, i)).unwrap();
                            }
                            Err(err) => println!("Could not extract links: {err:#}"),
                        }
                    },
                    Err(_) => {
                        println!("No more links to crawl");
                        break;
                    },
                }

            }

            println!("Thread {} is done", i);

        });
    }

    let urls = vec![
        Url::parse("https://www.google.com").unwrap(),
        Url::parse("https://www.naver.com").unwrap(),
        Url::parse("https://www.facebook.com").unwrap(),
        Url::parse("https://m.kakaobank.com/").unwrap(),
        Url::parse("https://www.google.co.in").unwrap(),
    ];

    for url in urls {
        tx.send(url).unwrap();
    }
    drop(tx);
    drop(result_tx);

    for result in result_rx {
        println!("Got link: {:?}", result);
    }
}