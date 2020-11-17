use tdaapi::{API};

use std::time;
use std::future;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let api = API::new();
    let symbols = vec![
        "AAPL",
        "ACB",
        "GE",
        "CCL",
        "MSFT",
        "TSLA"
    ];
    //let months = 2;
    let mut handles = Vec::new();
    for sym in symbols {
        let mut api_c = api.clone();
        let handle = tokio::spawn(async move {
            let symbol = sym;
            let history = api_c.history(symbol, 2).await.unwrap();
            let mut average: f64 = history.iter().map(|candle| candle.close).sum();
            average /= history.len() as f64;
            println!("The ask price for {} is {:?}", symbol, history);
            println!("The average price is {}", average);
        });
        handles.push(handle);
    }

    for h in handles {
        h.await;
    }

}
