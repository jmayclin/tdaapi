extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use tdaapi::{API};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let api = API::new();
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

}
