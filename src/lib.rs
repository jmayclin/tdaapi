extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use tokio::time::{Duration, Instant, /*Sleep*/};
use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::{Body, Method, Request};
use std::sync::{Arc, Mutex};

use std::fs;

/*
#[derive(Clone)]
struct Client {
    name: String,
}

impl Client {
    fn new() -> Client {
        Client{name: String::from("interior client")}
    }

    async fn request(&self) {
        // can return response error
        // or error from being locked out
        println!("Making an async call from inner client");
        tokio::time::sleep(Duration::from_millis(10)).await;
        println!("Finished making an async call from inned client");
    }
} */

#[derive(Clone)]
struct LimitedClient {
    //last_call: tokio::time::Interval,
    last_call: Arc<Mutex<Instant>>,
    client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl LimitedClient {
    const DELAY: Duration = Duration::from_millis(1000);

    fn new() -> LimitedClient{
        let https = HttpsConnector::new();

        LimitedClient {            
            last_call: Arc::new(Mutex::new(Instant::now())),
            //ticker: tokio::time::interval(Duration::new(1,0)),
            client: Client::builder().build::<_, hyper::Body>(https),
        }
    }

    pub async fn request(&self, req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
        println!("Calling from limited client");
        self.get_delay().await;
        let resp = self.client.request(req).await?;
        println!("Finished calling from limited client");
        Ok(resp)
    }

    fn get_delay(&self) -> tokio::time::Delay {
        let mut last_call = self.last_call.lock().unwrap();
        *last_call += LimitedClient::DELAY;
        tokio::time::delay_until(*last_call)
    }
}

#[derive(Clone)]
pub struct API {
    client: LimitedClient,
    access_token: String,
}

impl API {
    pub fn new() -> API {
        let token = API::get_access_token().unwrap();
        println!("Initilized with access token of {}", token);
        API {
            client: LimitedClient::new(),
            access_token: token,
            //ticker: tokio::time::interval(Duration::new(1,0)),
        }
    }

    fn get_access_token() -> Result<String, std::io::Error>
    {
        String::from("imanaccesstoken");
        let contents = String::from(fs::read_to_string("access.secret")?.trim());
        Ok(contents)
    }

    fn get_refresh_token() -> Result<String, std::io::Error> {
        let contents = String::from(fs::read_to_string("access.secret")?.trim());
        Ok(contents)    
    }

    pub async fn google(&self) -> Result<hyper::StatusCode, Box<dyn std::error::Error + Send + Sync>>
    {
        println!("Calling from API");
        let req = Request::builder()
            .method(Method::GET)
            .uri("http://www.google.com")
            .header("User-Agent", "Miss Vanjie")
            .body(Body::from(""))?;
        // I would then build the request
        let resp = self.request(req).await?;
        println!("The response was {:?}", resp.status());
        Ok(resp.status())
    }

    // structs in seperate module
    // tdaapi module
    
    // 1
    async fn refresh_access_token(&mut self) {

    }

    // 2
    pub async fn price_history(&self) {}

    //3 
    pub async fn get_quote(&self) {} 

    //4
    pub async fn get_quotes(&self) {}

    //5
    pub async fn get_transactions(&self) {}



    async fn request(&self, req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
        let resp = self.client.request(req).await?;
        println!("Status code is {:?}", resp.status());

        Ok(resp)
        
    }



}