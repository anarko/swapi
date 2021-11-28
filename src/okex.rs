use ring::{hmac};
use data_encoding::BASE64;
use reqwest::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use chrono::prelude::{Utc, SecondsFormat};
use json;

static API_KEY: &'static str = "ff24ff67-74e6-4138-92a0-e5c45f4bb065";
static API_SECRET: &'static str = "5A79DDF95C603F354643EFCBE5B346AE";
static PASSPHRASE: &'static str = "a12345678";


pub async fn make_api_request(method : String, end_point : String, body: String) -> Result<()> {

    let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

    let mut signature_content = String::new();
    signature_content.push_str(&timestamp);
    signature_content.push_str(&method);
    signature_content.push_str(&end_point);
    signature_content.push_str(&body);

    let signed_key = hmac::Key::new(ring::hmac::HMAC_SHA256, API_SECRET.as_bytes());
    let signature = hmac::sign(&signed_key, signature_content.as_bytes());
    let base64_signature = BASE64.encode(signature.as_ref());
    println!("{}",signature_content);

    let mut header_map = HeaderMap::new();
    header_map.insert("OK-ACCESS-KEY", HeaderValue::from_str(API_KEY).unwrap());
    header_map.insert("OK-ACCESS-SIGN", HeaderValue::from_str(&base64_signature).unwrap());
    header_map.insert("OK-ACCESS-TIMESTAMP", HeaderValue::from_str(&timestamp).unwrap());
    header_map.insert("OK-ACCESS-PASSPHRASE", HeaderValue::from_str(PASSPHRASE).unwrap());
    header_map.insert("x-simulated-trading", HeaderValue::from_str("1").unwrap());
    header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let mut complete_url = String::from("https://www.okex.com");

    complete_url.push_str(&end_point);


    println!("URL : {}", complete_url.as_str());
    println!("Body : {}", body.as_str());

    let res = client
        .get(complete_url.as_str())
        .headers(header_map)
        .body(body)
        .send()
        .await?;

    let t = res
        .text()
        .await?;

    println!("--> {}", t);


    Ok(())
}