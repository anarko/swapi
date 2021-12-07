use ring::{hmac};
use data_encoding::BASE64;
use reqwest::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use chrono::prelude::{Utc, SecondsFormat};
//use json;
use serde_json;
use serde_json::json;

static API_KEY: &'static str = "ff24ff67-74e6-4138-92a0-e5c45f4bb065";
static API_SECRET: &'static str = "5A79DDF95C603F354643EFCBE5B346AE";
static PASSPHRASE: &'static str = "a12345678";


pub async fn make_api_request(method : reqwest::Method, end_point : Option<String>, body: Option<String>) -> Result<serde_json::Value> {

    let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let mut request_body = String::new();
    if body != None {
       request_body = body.unwrap();
    }
    let path = end_point.unwrap();

    let mut signature_content = String::new();
    signature_content.push_str(&timestamp);
    signature_content.push_str(&method.to_string().to_uppercase());
    signature_content.push_str(&path);
    signature_content.push_str(&request_body);

    let signed_key = hmac::Key::new(ring::hmac::HMAC_SHA256, API_SECRET.as_bytes());
    let signature = hmac::sign(&signed_key, signature_content.as_bytes());
    let base64_signature = BASE64.encode(signature.as_ref());
    //println!("{}",signature_content);

    let mut header_map = HeaderMap::new();
    header_map.insert("OK-ACCESS-KEY", HeaderValue::from_str(API_KEY).unwrap());
    header_map.insert("OK-ACCESS-SIGN", HeaderValue::from_str(&base64_signature).unwrap());
    header_map.insert("OK-ACCESS-TIMESTAMP", HeaderValue::from_str(&timestamp).unwrap());
    header_map.insert("OK-ACCESS-PASSPHRASE", HeaderValue::from_str(PASSPHRASE).unwrap());
    header_map.insert("x-simulated-trading", HeaderValue::from_str("1").unwrap());
    header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let mut complete_url = String::from("https://www.okex.com");

    complete_url.push_str(&path);

/*
    println!("URL : {}", complete_url.as_str());
    println!("Body : {}", request_body.as_str());
*/
    let r  = client.request(method, complete_url.as_str())
        .headers(header_map)
        .body(request_body)
        .send()
        .await?;
/*
    let json = json::parse(&r.text().await?).unwrap();
*/
    let json = serde_json::from_str(&r.text().await?).unwrap();
    //let json = json!(&r.text().await?);
    Ok(json)
}

pub async fn get_order_book(ticket: String) -> Result<serde_json::Value> {

    let mut path = String::from("/api/v5/market/books?sz=400&instId=");
    path.push_str(&ticket.as_str());
    let ret = make_api_request(reqwest::Method::GET,Some(path),None).await?;
    Ok(ret)
}


pub async fn get_pair_fee(inst_id: String) -> Result<serde_json::Value> {

    let mut path = String::from("/api/v5/account/trade-fee?instType=SPOT&instId=");
    path.push_str(&inst_id.as_str());
    let ret = make_api_request(reqwest::Method::GET,Some(path),None).await?;

    Ok(ret)
}

pub async fn get_instruments() -> Result<serde_json::Value> {

    let path = String::from("/api/v5/public/instruments?instType=SPOT");
    let ret = make_api_request(reqwest::Method::GET,Some(path),None).await?;

    let data = ret.get("data").unwrap_or(&json!(null));
    Ok(data.clone())
}

pub async fn place_order(order:String) -> Result<serde_json::Value> {

    let path = String::from("/api/v5/trade/order");
    let ret = make_api_request(reqwest::Method::POST,Some(path),Some(order)).await?;
    Ok(ret)
}


pub async fn get_order(inst_id:String,cl_ord_id:String) -> Result<serde_json::Value> {

    let path = String::from(format!("/api/v5/trade/order?instId={}&clOrdId={}",inst_id,cl_ord_id));

    let ret = make_api_request(reqwest::Method::GET,Some(path),None).await?;
    Ok(ret)
}
