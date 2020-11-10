extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use tdaapi::{API};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut api = API::new();
    let handle = tokio::spawn(async move {
        let res = api.get_quote("AAPL").await;
    });
    handle.await;

    /*
    let mut handles = Vec::new();
    for _ in 0..5 {
        let api_c = api.clone();
        let h = 
        tokio::spawn(async move {
            api_c.google().await;
        });

        handles.push(h);
    }

    for h in handles {
        h.await;
    }
    */

}
