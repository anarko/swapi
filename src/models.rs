use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::schema::*;

pub type SpotSwaps = HashMap<String, SpotSwap>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub msg: String,
    pub err_nro: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SwapQuery {
    pub pair: String,
    pub quantity: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SwapUUID {
    pub swap_uuid: String,
}

#[derive( Debug, Deserialize, Serialize, Clone)]
pub struct SpotSwap {
    pub swap_uuid: String,
    pub pair: String,
    pub book: String,
    pub side: String,
    pub quantity: f32,
    pub estimated_price : f32,
    pub estimated_qty : f32,
    pub time_satmp : i64
}

#[derive(Insertable,Serialize,Clone)]
#[table_name = "swaps"]
pub struct DbSwap<'a> {
    pub swap_uuid: &'a str,
    pub pair: &'a str,
    pub book: &'a str,
    pub side: &'a str,
    pub quantity: &'a str,
    pub price: &'a str,
    pub fee: &'a str,
    pub swapi_fee : &'a str,
    pub fee_currency: &'a str,
    pub time_satmp: &'a str
}
#[derive(Debug, Queryable)]
pub struct SwapConfig {
   pub conf:String,
   pub value:String
}
