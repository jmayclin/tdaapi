use tdaapi::{API};

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
    let mut handles = Vec::new();
    for sym in symbols {
        let mut api_c = api.clone();
        let handle = tokio::spawn(async move {
            let symbol = sym;
            let ask_price = api_c.quote(symbol).await.unwrap();
            println!("The ask price for {} is {}", symbol, ask_price);
        });
        handles.push(handle);
    }

    for h in handles {
        h.await;
    }

}
