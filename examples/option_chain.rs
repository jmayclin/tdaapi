use tdaapi::{API, QuoteStock};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let api = API::new();
    let symbols = vec![
        "AAPL",
        //"ACB",
        //"GE",
        //"CCL",
        //"MSFT",
        //"TSLA"
    ];
    let mut handles = Vec::new();
    for sym in symbols {
        let mut api_c = api.clone();
        let handle = tokio::spawn(async move {
            let symbol = sym;
            let options = api_c.get_option_chain(symbol).await.unwrap();
            println!("The options are {:?}", options);
        });
        handles.push(handle);
    }

    for h in handles {
        h.await;
    }

}
