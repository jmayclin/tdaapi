extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate tokio;

use hyper::Client;
use hyper::{Body, Method, Request, Response};
use hyper_tls::HttpsConnector;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant /*Sleep*/};

use std::error::Error;
use std::fs;
use std::io::Write;
use std::path;

use serde::{Deserialize, Serialize};

//mod structs;
//use structs::AccessTokenResponse;

use http::StatusCode;

#[derive(Serialize, Debug)]
pub struct AccessTokenRequest {
    grant_type: String,
    refresh_token: String,
    client_id: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessTokenResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
    refresh_token_expires_in: u64,
}

#[derive(Deserialize, Debug)]
struct QuoteStock {
    //Equity:
    symbol: String,
    description: String,
    bidPrice: f64,
    bidSize: f64,
    bidId: String,
    askPrice: f64,
    askSize: f64,
    askId: String,
    lastPrice: f64,
    lastSize: f64,
    lastId: String,
    openPrice: f64,
    highPrice: f64,
    lowPrice: f64,
    closePrice: f64,
    netChange: f64,
    totalVolume: f64,
    quoteTimeInLong: f64,
    tradeTimeInLong: f64,
    mark: f64,
    exchange: String,
    exchangeName: String,
    marginable: bool,
    shortable: bool,
    volatility: f64,
    digits: f64,
    WkHigh: f64,
    WkLow: f64,
    peRatio: f64,
    divAmount: f64,
    divYield: f64,
    divDate: String,
    securityStatus: String,
    regularMarketLastPrice: f64,
    regularMarketLastSize: f64,
    regularMarketNetChange: f64,
    regularMarketTradeTimeInLong: f64,
}

#[derive(Deserialize, Debug)]
struct QuoteOption {
    symbol: String,
    description: String,
    bidPrice: f64,
    bidSize: f64,
    askPrice: f64,
    askSize: f64,
    lastPrice: f64,
    lastSize: f64,
    openPrice: f64,
    highPrice: f64,
    lowPrice: f64,
    closePrice: f64,
    netChange: f64,
    totalVolume: f64,
    quoteTimeInLong: f64,
    tradeTimeInLong: f64,
    mark: f64,
    openInterest: f64,
    volatility: f64,
    moneyIntrinsicValue: f64,
    multiplier: f64,
    strikePrice: f64,
    contractType: String,
    underlying: String,
    timeValue: f64,
    deliverables: String,
    delta: f64,
    gamma: f64,
    theta: f64,
    vega: f64,
    rho: f64,
    securityStatus: String,
    theoreticalOptionValue: f64,
    underlyingPrice: f64,
    uvExpirationType: String,
    exchange: String,
    exchangeName: String,
    settlementType: String,
}

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
        println!(
            "Initilized with \naccess token of {}\nconsumer key of {}",
            token, consumer_key
        );

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
        let contents = String::from(fs::read_to_string("refresh.secret")?.trim());
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
    async fn refresh_access_token(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let refresh_token = API::get_refresh_token();
        if self.last_refresh.elapsed() < Duration::from_secs(60 * 5) {
            panic!("Attempting to refresh token after less than 5 minutes");
        }

        let refresh_body = AccessTokenRequest {
            client_id: "OD8T1O14POUWY00BJJPGQBIPWPQ8PNWZ".to_owned(),
            grant_type: String::from("refresh_token"),
            refresh_token: API::get_refresh_token()?,
        };

        println!(
            "The body of the request is {}",
            serde_json::to_string(&refresh_body)?
        );

        let req1 = Request::builder()
            .method(Method::POST)
            .uri("https://api.tdameritrade.com/v1/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept-Encoding", "gzip")
            .header("Accept-Language", "en-US")
            .header(
                "User-Agent",
                "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:82.0) Gecko/20100101 Firefox/82.0",
            )
            .body(Body::from(format!(
                "grant_type=refresh_token&refresh_token={refresh_token}&client_id={consumer_key}",
                consumer_key = API::get_consumer_key()?,
                refresh_token = API::get_refresh_token()?
            )))?;

        let req2 = Request::builder()
            .method(Method::POST)
            .uri("https://api.tdameritrade.com/v1/oauth2/token")
            .header("Content-Type", "application/json")
            .header("Accept-Language", "en-US")
            .header("User-Agent", "Miss Vanjie")
            .body(Body::from(serde_json::to_string(&refresh_body)?))?;

        let req3 = Request::builder()
            .method(Method::POST)
            .uri("https://api.tdameritrade.com/v1/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", "Miss Vanjie")
            .body(Body::from("grant_type=refresh_token&refresh_token=Pe%2F4TMiknedQhJpwHqrGZJIYw1zH3zj3yJdDBC64qDce%2FRCEBC%2BmbkIdAY%2B%2BbbLi6ro%2BuKYHTebb23AOg7To8KJfXKTgyIL4I2xWC9s%2BPCwvSieHgL3s%2BfJ8htZJA3GZDt8FZbWch8icoLb0EnJUyKj0JJ3tu4Tz%2BRG78JXhazW8ztMBCuDXvSQLoV%2BsiMJ29fgV9nznSzaL2QvjkAb9NVnuC5t0mAadSsbKkt8zL3DuUoTPP8AAQ%2FTH8r4sPB94gNjgloiGZQ5aaH1kpr%2BpKL1x%2FmCJ4hcnQ%2BABVyI75UAkauSoEQLkKC9BMGFVCjY6kVSvo3foewv2jrcKLWVM%2FofZF0JMGOVRoATbexhvBhAQ7cEP1i4W3aVXKV307XqryNrRsV7ktHt04D%2FY2Aez9bMq9B0hnj0vt5hIEjiba3B4BsqnVS%2Fk4%2F9txEQ100MQuG4LYrgoVi%2FJHHvlL8DU5J9puGlPd74mZ7M41EZmCQX%2B0gpmEFd7hs6cPGxbvYB%2B%2BC2%2FTSk%2FwcMl4L6i6AFMVXr9Q7WcvxO4AsyHsW38Z42fnHHzYzg3COG42xG%2FnxXwFeId%2FzSwr31ZsZc7qKmia1Slmx%2B0MdVTEAhEByQnyPHT7%2FJmHxoNvUsy83IVowDkV9Sslkk5h4HbIdKCFd0Pjo6kkOmq7deHoV6jLS0EtBeDsfY7430ETZ3a91945njCgDqwUdHoUwRrXm61YiBAWX7ygZylQNTshT1JSH8Bm1GlJjXMuMDxyE17yMf1I76oLDQ37cw4G6V1G3IiHoQidw0N6KEfPI3QnkGLg%2BFCT8ziVtvoN26H7qwXgctBTVK5ZvI94pfm%2B7fK7%2F0HeJxTdQ%2BYzVFAPMwGKvcK4wQqAxH3HXHuaRWJ%2F%2F%2B34B7UoH2PKKuvAe%2BWdyE%3D212FD3x19z9sWBHDJACbC00B75E&access_type=&code=&client_id=OD8T1O14POUWY00BJJPGQBIPWPQ8PNWZ&redirect_uri="))?;
        let resp = self.client.request(req3).await?;

        let res_status = resp.status();
        println!("Headers:\n{:#?}", resp.headers());
        let body = hyper::body::to_bytes(resp.into_body()).await?;
        println!("The body of the response is {:?}", body);
        // too many retires return the exit
        if res_status != http::StatusCode::OK {
            return Err("Failed to refresh access token".into());
        }
        let auth_response: AccessTokenResponse = serde_json::from_slice(&body)?;
        println!("The new access token response is {:?}", auth_response);
        println!("The actual access token is {}", auth_response.access_token);

        let path = path::Path::new("access.secret");
        let display = path.display();
        println!("[API] writing access token to {}", display);

        let data = "Some data!";
        let mut f = fs::File::create("access.secret").expect("Unable to create file");
        f.write_all(auth_response.access_token.as_bytes())
            .expect("Unable to write data");

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
        println!(
            "[{place}] -> {method} : {message}",
            place = "API",
            method = "get_quote",
            message = format!("getting quote of {}", symbol)
        );
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!(
                "https://api.tdameritrade.com/v1/marketdata/{ticker}/quotes",
                ticker = symbol
            ))
            .header("User-Agent", "Miss Vanjie")
            .header(
                "Authorization",
                format!("Bearer {token}", token = self.access_token),
            )
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
        // req2 = req.clone()
        self.limiter.delay().await;
        while *self.refreshing.lock().unwrap() {
            println!(
                "[{place}] -> {method} : {message}",
                place = "API",
                method = "request",
                message = "reschedule request to to current refresh"
            );
            self.limiter.delay().await;
        }

        let resp = self.client.request(req).await?;
        println!("Status code is {:?}", resp.status());
        match resp.status() {
            http::StatusCode::OK => Ok(resp),
            http::StatusCode::UNAUTHORIZED => {
                self.refresh_access_token().await?;
                Err("need to refresh code".into())
            }
            _ => Err("unrecognized error".into()),
        }
    }

    /*fn clone_req(req: &hyper::Request<hyper::Body>) -> hyper::Request<hyper::Body> {
        let mut new_req = hyper::Request::new(req.method().clone(), req.uri().clone());
        new_req.headers_mut().extend(req.headers().iter());
        new_req.set_body(req.body()); // <------- here the error occur!
        new_req
    }*/
}
