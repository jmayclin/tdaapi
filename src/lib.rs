extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate http;

use hyper::Client;
use hyper::{Body, Method, Request, Response};
use hyper_tls::HttpsConnector;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant /*Sleep*/};

use std::io::Write;
use std::error::Error;
use std::fs;
use std::path;

use http::StatusCode;

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


// need to add bearer token in the Limited Clietn and add it to the request
#[derive(Clone)]
struct Limiter {
    last_call: Arc<Mutex<Instant>>,
}

impl Limiter {
    const DELAY: Duration = Duration::from_secs(1);

    fn new() -> Limiter {
        Limiter {
            last_call: Arc::new(Mutex::new(Instant::now())),            
        }
    }

    fn delay(&self) -> tokio::time::Delay {
        let mut last_call = self.last_call.lock().unwrap();
        *last_call += Limiter::DELAY;
        tokio::time::delay_until(*last_call)
    }
}

#[derive(Clone)]
pub struct API {
    limiter: Limiter,
    access_token: String,
    consumer_key: String,
    last_refresh: Instant,
    refreshing: Arc<Mutex<bool>>,
    client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    // last token refresh epoch, initlize to zero
}

impl API {
    pub fn new() -> API {
        let token = API::get_access_token().unwrap();
        let consumer_key = API::get_consumer_key().unwrap();
        let https = HttpsConnector::new();
        println!("Initilized with \naccess token of {}\nconsumer key of {}", token, consumer_key);

        API {
            limiter: Limiter::new(),
            consumer_key: consumer_key,
            access_token: token,
            last_refresh: Instant::now() - Duration::from_secs(60 * 15),
            refreshing: Arc::new(Mutex::new(false)),
            client: Client::builder().build::<_, hyper::Body>(https),
        }
    }

    fn get_access_token() -> Result<String, std::io::Error> {
        String::from("imanaccesstoken");
        let contents = String::from(fs::read_to_string("access.secret")?.trim());
        Ok(contents)
    }

    fn get_refresh_token() -> Result<String, std::io::Error> {
        let contents = String::from(fs::read_to_string("access.secret")?.trim());
        Ok(contents)
    }


    fn get_consumer_key() -> Result<String, std::io::Error> {
        let contents = String::from(fs::read_to_string("consumer.secret")?.trim());
        Ok(contents)
    }

    pub async fn google(
        &mut self,
    ) -> Result<hyper::StatusCode, Box<dyn std::error::Error + Send + Sync>> {
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
    async fn refresh_access_token(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync> > {
        let refresh_token = API::get_refresh_token();
        if self.last_refresh.elapsed() < Duration::from_secs(60 * 5) {
            panic!("Attempting to refresh token after less than 5 minutes");
        }
        // if you tried to refresh the token less than 15 minutes ago
        //     scream
        let req = Request::builder()
            .method(Method::GET)
            .uri("https://api.tdameritrade.com/v1/oauth2/token")
            .header("User-Agent", "Miss Vanjie")
            .body(Body::from(""))?;
        let resp = self.client.expedite(req).await?;

        // too many retires return the exit
        if resp.status() != http::StatusCode::OK {
            return Err("too many retries".into());
        }

        let body = hyper::body::to_bytes(resp.into_body()).await?;
        println!("The new access token is {:?}", body);

        let path = path::Path::new("access.secret");
        let display = path.display();
        println!("[API] writing access token to {}", display);

        let data = "Some data!";
        let mut f = fs::File::create("access.secret").expect("Unable to create file");
        f.write_all(data.as_bytes()).expect("Unable to write data");


        // write it to the access.secret file 
        // build request
        // send request via override
        // if status isn't 200
        //     scream
        // otherwise set access token file
        // set access token for API
        Ok(())
    }

    async fn refresh_refresh_token(&self) {}

    // 2
    pub async fn price_history(&self) {}

    //3
    pub async fn get_quote(&mut self, symbol: &str) -> Result<f32, Box<dyn Error + Send + Sync>> {
        println!("[{place}] -> {method} : {message}", place = "API", method = "get_quote", message = format!("getting quote of {}", symbol));
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("https://api.tdameritrade.com/v1/marketdata/{ticker}/quotes", ticker = symbol))
            .header("User-Agent", "Miss Vanjie")
            .header("Authorization", format!("Bearer {token}", token = self.access_token))
            .body(Body::from(""))?;
        // I would then build the request
        let resp = self.request(req).await?;
        let body = hyper::body::to_bytes(resp.into_body()).await?;
        println!("{:?}", body);
        Ok(3.2)
    }

    //4
    pub async fn get_quotes(&self) {}

    //5
    pub async fn get_transactions(&self) {}

    async fn request(
        &mut self,
        req: hyper::Request<hyper::Body>,
    ) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
        self.limiter.delay().await;

        let resp = self.client.request(req).await?;
        println!("Status code is {:?}", resp.status());
        match resp.status() {
            http::StatusCode::OK => Ok(resp),
            http::StatusCode::UNAUTHORIZED => {
                self.refresh_access_token().await?;
                //Ok(self.request(req).await?)
                Ok(resp)
            },
            _ => Err("unrecognized error".into()),
        }
        //let body_str = hyper::body::to_bytes(resp.clone().into_body()).await;
        //println!("Body is {:?}", resp.body().await);
    }

    async fn request_internal( &self, req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
        Ok(self.client.request(req).await?)
    }
}
