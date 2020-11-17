use hyper::Client;
use hyper::{Body, Method, Request, Response, Uri};
use hyper_tls::HttpsConnector;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant /*Sleep*/};

use std::error::Error;
use std::fs;
use std::io::Write;
use std::path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::structs::*;

use http::StatusCode;


// move this into a different module
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
            "[new] Initilized with \naccess token of {}\nconsumer key of {}",
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

    // structs in seperate module
    // tdaapi module
    // 1
    pub async fn refresh_access_token(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.last_refresh.elapsed() < Duration::from_secs(60 * 5) {
            panic!("Attempting to refresh token after less than 5 minutes");
        } else {
            self.last_refresh = Instant::now();
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

        let req = Request::builder()
            .method(Method::POST)
            .uri("https://api.tdameritrade.com/v1/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", "Miss Vanjie")
            .body(Body::from("grant_type=refresh_token&refresh_token=Pe%2F4TMiknedQhJpwHqrGZJIYw1zH3zj3yJdDBC64qDce%2FRCEBC%2BmbkIdAY%2B%2BbbLi6ro%2BuKYHTebb23AOg7To8KJfXKTgyIL4I2xWC9s%2BPCwvSieHgL3s%2BfJ8htZJA3GZDt8FZbWch8icoLb0EnJUyKj0JJ3tu4Tz%2BRG78JXhazW8ztMBCuDXvSQLoV%2BsiMJ29fgV9nznSzaL2QvjkAb9NVnuC5t0mAadSsbKkt8zL3DuUoTPP8AAQ%2FTH8r4sPB94gNjgloiGZQ5aaH1kpr%2BpKL1x%2FmCJ4hcnQ%2BABVyI75UAkauSoEQLkKC9BMGFVCjY6kVSvo3foewv2jrcKLWVM%2FofZF0JMGOVRoATbexhvBhAQ7cEP1i4W3aVXKV307XqryNrRsV7ktHt04D%2FY2Aez9bMq9B0hnj0vt5hIEjiba3B4BsqnVS%2Fk4%2F9txEQ100MQuG4LYrgoVi%2FJHHvlL8DU5J9puGlPd74mZ7M41EZmCQX%2B0gpmEFd7hs6cPGxbvYB%2B%2BC2%2FTSk%2FwcMl4L6i6AFMVXr9Q7WcvxO4AsyHsW38Z42fnHHzYzg3COG42xG%2FnxXwFeId%2FzSwr31ZsZc7qKmia1Slmx%2B0MdVTEAhEByQnyPHT7%2FJmHxoNvUsy83IVowDkV9Sslkk5h4HbIdKCFd0Pjo6kkOmq7deHoV6jLS0EtBeDsfY7430ETZ3a91945njCgDqwUdHoUwRrXm61YiBAWX7ygZylQNTshT1JSH8Bm1GlJjXMuMDxyE17yMf1I76oLDQ37cw4G6V1G3IiHoQidw0N6KEfPI3QnkGLg%2BFCT8ziVtvoN26H7qwXgctBTVK5ZvI94pfm%2B7fK7%2F0HeJxTdQ%2BYzVFAPMwGKvcK4wQqAxH3HXHuaRWJ%2F%2F%2B34B7UoH2PKKuvAe%2BWdyE%3D212FD3x19z9sWBHDJACbC00B75E&access_type=&code=&client_id=OD8T1O14POUWY00BJJPGQBIPWPQ8PNWZ&redirect_uri="))?;
        let resp = self.client.request(req).await?;

        // too many retires return the exit
        if resp.status() != http::StatusCode::OK {
            return Err("Failed to refresh access token".into());
        }

        let body = hyper::body::to_bytes(resp.into_body()).await?;
        let auth_response = serde_json::from_slice(&body);
        let auth_response: AccessTokenResponse = match auth_response {
            Ok(response) => response,
            Err(error) => panic!("Problem parsing response {:?}", error),
        };
        //println!("The new access token response is {:?}", auth_response);
        //println!("The actual access token is {}", auth_response.access_token);

        let mut f = fs::File::create("access.secret").expect("Unable to create file");
        //println!("Created file");
        f.write_all(auth_response.access_token.as_bytes())
            .expect("Unable to write data");
        self.access_token = auth_response.access_token;
        println!("[refresh_access_token] Wrote accesstoken to fle?");
        Ok(())
    }

    async fn refresh_refresh_token(&self) {}

    // 2
    pub async fn history(
        &mut self,
        symbol: &str,
        period: u32,
    ) -> Result<Vec<Candle>, Box<dyn Error + Send + Sync>> {
        println!("[history] Requestion history for {}", symbol);

        let uri = format!(
            "https://api.tdameritrade.com/v1/marketdata/{ticker}/pricehistory?apikey={apikey}&periodType={period_type}&period={period}&frequencyType={frequency_type}&frequency={frequency}",
            ticker = symbol,
            apikey = API::get_consumer_key()?,
            period_type = "month",
            period = period,
            frequency_type = "daily",
            frequency = "1",
        ).parse::<Uri>()
        .unwrap();

        let mut headers = hyper::HeaderMap::new();
        headers.insert(
            "User-Agent",
            hyper::header::HeaderValue::from_str("Miss Vanjie")?,
        );

        let resp = self.request(
            Method::GET,
            uri,
            headers,
            String::from(""),
        ).await?;
        let body = hyper::body::to_bytes(resp.into_body()).await?;

        // condense this
        let history = serde_json::from_slice(&body);
        let history: PriceHistory = match history {
            Ok(response) => response,
            Err(error) => panic!("Problem parsing response {:?}", error),
        };

        Ok(history.candles)
    }

    //3
    pub async fn quote(&mut self, symbol: &str) -> Result<QuoteStock, Box<dyn Error + Send + Sync>> {
        println!(
            "[quote] : getting quote of symbol {}",
            symbol
        );
        let uri = format!(
            "https://api.tdameritrade.com/v1/marketdata/{ticker}/quotes",
            ticker = symbol
            )
            .parse::<Uri>()
            .unwrap();

        let mut headers = hyper::HeaderMap::new();
        headers.insert(
            "User-Agent",
            hyper::header::HeaderValue::from_str("Miss Vanjie")?,
        );

        let resp = self
            .request(Method::GET, uri, headers, String::from(""))
            .await?;
        let body = hyper::body::to_bytes(resp.into_body()).await?;

        // condense this
        // swtich these error messages to fatal failures
        let stock_quote = serde_json::from_slice(&body);
        let stock_quote: std::collections::HashMap<String, QuoteStock> = match stock_quote {
            Ok(response) => response,
            Err(error) => panic!("Problem parsing response {:?}", error),
        };

        println!("Stock quote for {} is {:?}", symbol, stock_quote);
        let quote = stock_quote[symbol].clone();
        Ok(quote)
    }

    //4
    pub async fn get_quotes(&self) {}

    //5
    pub async fn get_transactions(&self) {}

    //6
    pub async fn get_option_chain(&mut self, symbol: &str) -> Result<Vec<(f64, f64)>, Box<dyn Error + Send + Sync>> {
        println!("[get_option_chain] option chain for {}", symbol);

        let uri = format!(
            "https://api.tdameritrade.com/v1/marketdata/chains?apikey={apikey}&symbol={ticker}&contractType=CALL&strikeCount=6&includeQuotes=TRUE&strategy=SINGLE&toDate=2020-11-21",
            ticker = symbol,
            apikey = API::get_consumer_key()?,
        ).parse::<Uri>()
        .unwrap();

        let mut headers = hyper::HeaderMap::new();
        headers.insert(
            "User-Agent",
            hyper::header::HeaderValue::from_str("Miss Vanjie")?,
        );

        let resp = self.request(
            Method::GET,
            uri,
            headers,
            String::from(""),
        ).await?;
        let sc = resp.status();
        let body = hyper::body::to_bytes(resp.into_body()).await?;

        match sc {
            http::StatusCode::OK => println!("Successful"),
            _ => println!("Something has gone terribly wrong {:?}", body),
        };

        println!("The body of the request is {:?}", body);
        // condense this
        let chain: Value = serde_json::from_slice(&body)?;
        println!("{}", serde_json::to_string_pretty(&chain).unwrap());

        let chain: OptionChain = match serde_json::from_slice(&body) {
            Ok(response) => response,
            Err(error) => panic!("Problem parseing response {:?}", error),
        };

        let date = chain.callExpDateMap.keys().next().unwrap();
        println!("The expiration date of these is {}", date);

        assert_eq!(chain.callExpDateMap.len(), 1);
        let price = chain.underlying.last;
        let inner = chain.callExpDateMap.get(date).unwrap();
        let options: Vec<(f64, f64)> = chain.callExpDateMap
            .get(date).unwrap().iter()
            .map(|(strike, options)| {
                assert_eq!(options.len(), 1);
                (strike.parse::<f64>().unwrap(), options[0].last)
            })
            .collect();


        Ok(options)

    }

    fn build_request(
        &self,
        method: &Method,
        uri: &Uri,
        headers: &hyper::HeaderMap,
        content: &str,
    ) -> Result<Request<Body>, Box<dyn Error + Send + Sync>> {
        let mut req = Request::builder()
            .method(method)
            .uri(uri)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .body(Body::from(content.to_string()))?;
        let req_headers = req.headers_mut();
            for (key, value) in headers.iter() {
                req_headers.insert(key, value.clone());
            }
        Ok(req)

    }

    async fn request(
        &mut self,
        method: Method,
        uri: Uri,
        headers: hyper::HeaderMap,
        content: String,
    ) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {        

        self.limiter.delay().await;
        while *self.refreshing.lock().unwrap() {
            println!(
                "[request] : {message}",
                message = "reschedule request to to current refresh"
            );
            self.limiter.delay().await;
        }

        let request = self.build_request(&method, &uri, &headers, &content)?;
        let resp = self.client.request(request).await?;

        println!("Status code is {:?}", resp.status());
        match resp.status() {
            http::StatusCode::OK => Ok(resp),
            http::StatusCode::UNAUTHORIZED => {
                self.refresh_access_token().await?;
                let retry = self.build_request(&method, &uri, &headers, &content)?;
                // it seems like this error handling is suspect
                Ok(self.client.request(retry).await?)
            }
            _ => Err("unrecognized error".into()),
        }
    }
}

