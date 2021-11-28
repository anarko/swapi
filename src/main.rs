use warp::{Filter};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use json;

mod okex;

type SpotSwaps = HashMap<Uuid, SpotSwap>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SwapQuery {
    pair: String,
    quantity: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SwapUUID {
    swpa_uuid: String,
}

#[derive( Debug, Deserialize, Serialize, Clone)]
struct SpotSwap {
    swap_uuid: Uuid,
    pair: String,
    quantity: f64,
    estimated_price : f64,
    time_satmp : f64
}

#[derive(Clone)]
struct Swapi {
  swap_list: SpotSwaps
}

impl Swapi {
    fn new() -> Self {
        Swapi {
           swap_list: HashMap::new(),
        }
    }
}

fn query_swap_price(swap_query: SwapQuery) -> SpotSwap {

    let swap_uuid = Uuid::new_v4();

    println!("{} {}", swap_query.pair,swap_query.quantity);

    let proposed_swap = SpotSwap {
       estimated_price : 0.00,
       pair : swap_query.pair,
       quantity : swap_query.quantity,
       swap_uuid : swap_uuid,
       time_satmp : 2.0,
    };

    return proposed_swap;
}


async fn get_swap_price(
        swap_query : SwapQuery,
    mut swapi: Swapi
    ) -> Result<impl warp::Reply, warp::Rejection> {

        let proposed_swap = query_swap_price(swap_query);
        swapi.swap_list.insert(proposed_swap.swap_uuid, proposed_swap.clone());

        Ok(warp::reply::json(
            &proposed_swap.clone()
        ))

}

async fn make_swap(
    swap_uuid: SwapUUID,
    swapi: Swapi
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut result = HashMap::new();
        let r = swapi.swap_list.clone();

        for (key,value) in r.iter() {
            result.insert(key, value);
        }

        Ok(warp::reply::json(
            &result
        ))

}

fn get_json_filter() -> impl Filter<Extract = (SwapQuery,), Error = warp::Rejection> + Clone {
    // filtramos que el contenido del body recibido sea un JSON que contiene
    // los datos que necesitamos para el request y que su tamaño no sea exesivo
    warp::body::content_length_limit(1024).and(warp::body::json())
}

fn post_json_filter() -> impl Filter<Extract = (SwapUUID,), Error = warp::Rejection> + Clone {
    // filtramos que el contenido del body recibido sea un JSON que contiene
    // los datos que necesitamos para el request y que su tamaño no sea exesivo
    warp::body::content_length_limit(1024).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let swapi = Swapi::new();
    let filter = warp::any().map(move || swapi.clone());

    let query_price = warp::get()
        .and(warp::path("spot_swap"))
        .and(warp::path::end())
        .and(get_json_filter())
        .and(filter.clone())
        .and_then(get_swap_price);

    let make_swap = warp::post()
        .and(warp::path("spot_swap"))
        .and(warp::path::end())
        .and(post_json_filter())
        .and(filter.clone())
        .and_then(make_swap);

    let routes = query_price.or(make_swap);

    let body = json::parse(r#"
    {
        "instId":"BTC-USDT",
        "tdMode":"cash",
        "clOrdId":"b15",
        "side":"buy",
        "ordType":"limit",
        "px":"2.15",
        "sz":"2"
    }

    "#).unwrap();

    let _h = okex::make_api_request("POST".to_string(),"/api/v5/trade/order".to_string(),body.to_string()).await;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8099))
        .await;
}