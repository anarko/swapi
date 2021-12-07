#[macro_use]
extern crate diesel;
use std::collections::HashMap;
use chrono::prelude::{Utc};
use parking_lot::RwLock;
use std::{thread};
use std::sync::Arc;
use warp::{Filter};
use uuid::Uuid;
use serde_json::json;
use serde_json;

mod db;
mod okex;
mod schema;
mod models;
use models::*;

#[derive(Clone)]
struct Swapi {
  swap_list: Arc<RwLock<SpotSwaps>>,
  swap_fee: f32,
  max_swap_wait: i64,
  swap_price_tolerance: f32,
  instruments: serde_json::Value
}

impl Swapi {
    fn new(inst: serde_json::Value) -> Self {
        Swapi {
           swap_list: Arc::new(RwLock::new(HashMap::new())),
           swap_fee: db::get_swap_config("fee".to_string()).parse::<f32>().unwrap_or(0.0),
           max_swap_wait: db::get_swap_config("swap_wait".to_string()).parse::<i64>().unwrap_or(5),
           swap_price_tolerance: db::get_swap_config("swap_price_tolerance".to_string()).parse::<f32>().unwrap_or(0.01),
           instruments: inst
        }
    }
}

fn del_old_swaps( swapi: &Swapi) {
    let mut swap_list = swapi.swap_list.write();
    let mut tmp_del = SpotSwaps::new();
    /* Se que no es lo mas elegante, pero cumple la funcion,
       cuando aprenda bien como funciona el rlock lo mejorare
    */
    for (key,value) in swap_list.iter() {
        if Utc::now().timestamp()-value.time_satmp > swapi.max_swap_wait {
            tmp_del.insert(key.clone(),value.clone());
        }
    }
    for key in tmp_del.iter() {
        swap_list.remove(key.0);
    }

}

async fn estimate_swap_price(
        pair : String,
        quantity: f32,
        swap_side: String
    ) -> f32 {
        let mut cant_total:f32 = 0.0;
        let mut cant_por_precio:f32 = 0.0;
        let mut fill = false;
        let book_side: &str;

        if swap_side == "buy" {
            book_side = "asks";
        } else {
            book_side = "bids";
        }

        let book = okex::get_order_book(pair.clone()).await.unwrap();

        for price in book.get("data").unwrap()[0][book_side].as_array().unwrap() {
            cant_total += price[1].as_str().unwrap().parse::<f32>().unwrap_or(0.0);
            cant_por_precio +=  price[0].as_str().unwrap().parse::<f32>().unwrap()* price[1].as_str().unwrap().parse::<f32>().unwrap_or(0.0);
            if cant_total >= quantity {
                fill = true;
                break;
            }
        }
        let mut precio_swap:f32 = 0.0;
        if fill {
            /*
                Para sacar el precio del swap hago el promedio ponderado de los precios y cantidades
                ya que no voy a comprar toda la cantidad al mismo precio
            */
            precio_swap = cant_por_precio/cant_total;
        }
        precio_swap
}


async fn get_swap_price(
        swap_query : SwapQuery,
        swapi: Swapi
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let swap_fee_conf = swapi.swap_fee;
        let swap_list = swapi.swap_list.clone();
        let pair_split = swap_query.pair.split("-").collect::<Vec<&str>>();
        let mut book_instr = String::new();
        let mut swap_side = String::new();

        // busco a que instrumento corresponde el swap y a que side tengo que ir
        for i in swapi.clone().instruments.as_array().iter(){
            for a in i.iter(){
                if pair_split[0] == a.get("baseCcy").unwrap().as_str().unwrap() && pair_split[1] == a.get("quoteCcy").unwrap().as_str().unwrap() {
                    swap_side = String::from("sell");
                    book_instr = String::from(a.get("instId").unwrap().as_str().unwrap_or(&""));
                    break;
                } else if pair_split[1] == a.get("baseCcy").unwrap().as_str().unwrap() && pair_split[0] == a.get("quoteCcy").unwrap().as_str().unwrap() {
                    book_instr = String::from(a.get("instId").unwrap().as_str().unwrap_or(&""));
                    swap_side = String::from("buy");
                    break;
                }
            }
        }

        // mando un worker a borrar los swaps viejos
        thread::spawn(move || del_old_swaps(&swapi.clone()));

        let precio_swap = estimate_swap_price(book_instr.clone(),swap_query.quantity ,swap_side.clone() ).await;

        if precio_swap != 0.0 {

            // busco las comisiones de okex
            let okex_fee_conf = okex::get_pair_fee(book_instr.clone()).await.unwrap();
            let okex_fee : f32;
            if okex_fee_conf.clone().get("data").unwrap()[0]["taker"].as_str().unwrap().parse::<f32>().unwrap_or(0.0).abs()  >
            okex_fee_conf.clone().get("data").unwrap()[0]["maker"].as_str().unwrap().parse::<f32>().unwrap_or(0.0).abs() {
                okex_fee = okex_fee_conf.clone().get("data").unwrap()[0]["taker"].as_str().unwrap().parse::<f32>().unwrap_or(0.0).abs()
            } else {
                okex_fee = okex_fee_conf.clone().get("data").unwrap()[0]["maker"].as_str().unwrap().parse::<f32>().unwrap_or(0.0).abs()
            }
            // es la cantidad estimada que recibe el cliente
            let mut estmiated_qty:f32;
            if swap_side == "buy" {
                estmiated_qty=swap_query.quantity/precio_swap;
            } else {
                estmiated_qty=swap_query.quantity*precio_swap;
            }

            // El fee de okex se debita en la moneda a acreditar, se lo saco a lo que recibe el cliente
            // Uso el mismo sistema q okex para el fee de la api
            estmiated_qty = estmiated_qty-(estmiated_qty*(okex_fee+swap_fee_conf));

            let proposed_swap = SpotSwap{
                swap_uuid : Uuid::new_v4().to_string(),
                pair : swap_query.pair.clone(),
                book : book_instr.clone(),
                side : swap_side.clone(),
                quantity : swap_query.quantity,
                estimated_price : precio_swap,
                estimated_qty: estmiated_qty,
                time_satmp : Utc::now().timestamp()
            };
            swap_list.write().insert(proposed_swap.swap_uuid.clone(), proposed_swap.clone());
            Ok(warp::reply::json(
                &proposed_swap.clone()
            ))
        } else {
            Ok(warp::reply::json(
                &ErrorResponse {msg: "El swap no puede estimarse, no hay market data".to_string(),err_nro : 1}
            ))
        }
    }

async fn make_swap(
    swap_uuid: SwapUUID,
    swapi: Swapi
    ) -> Result<impl warp::Reply, warp::Rejection> {

        let swap = swapi.swap_list.write().remove(&swap_uuid.swap_uuid);

        if swap.is_none() {
            Ok(warp::reply::json(
                &ErrorResponse {msg: "El swap no existe o ha expirado el tiempo".to_string(),err_nro : 2}
            ))
        } else {

            if Utc::now().timestamp()-swap.clone().as_ref().unwrap().time_satmp <= swapi.max_swap_wait {
                // el swap todavia esta vigente

                // pedir de nuevo los precios, ver si estamos cerca => ejecutar a precio de mercado
                let _swap = swap.clone().unwrap();
                let swap_price = estimate_swap_price(_swap.clone().book,_swap.clone().quantity,_swap.clone().side).await;

                // si hay % de tolerancia al cambio se fija
                if swapi.swap_price_tolerance != 0.0 {
                    let dif_porcent = (swap_price*100.0/_swap.estimated_price)-100.0;
                    if dif_porcent > swapi.swap_price_tolerance {
                        return Ok(warp::reply::json(
                            &ErrorResponse{msg:"El precio actual del swap supera el % de tolerancia de variacion".to_string(),err_nro:3}
                        ));
                    }
                }
                // Du de swap
                let uu = Uuid::parse_str(_swap.clone().swap_uuid.clone().as_str()).unwrap().to_simple().to_string();
                // En okex cuando vas a vender la cantidad es en base asset y comprar en quoted asset
                let order = json!({"instId":_swap.clone().book,
                                   "tdMode":"cash",
                                   "clOrdId":uu,
                                   "side":_swap.clone().side,
                                   "ordType":"market",
                                   "sz":format!("{:.8}",_swap.clone().quantity)
                            });
                let mut ret = okex::place_order(order.to_string()).await.unwrap();
                // verificamos que se haya puesto la orden bien
                if ret.get("code").unwrap().as_str().unwrap() != "0" {
                    return Ok(warp::reply::json(
                        &ErrorResponse{msg:format!("EL swap no pudo ser realizado ({})",ret.get("data").unwrap()[0].get("sMsg").unwrap()),err_nro:4}
                    ));
                }
                // como la orden es a mercado se ejecuta inmediatamente
                ret = okex::get_order(_swap.clone().book,uu.clone()).await.unwrap();
                // guardamos el swap ejecutado en la db
                let swapi_fee = -swapi.swap_fee*ret.get("data").unwrap()[0].get("fillSz").unwrap().as_str().unwrap().parse::<f32>().unwrap();
                let rt = DbSwap {
                    swap_uuid: &uu,
                    pair: &_swap.pair,
                    book: &_swap.book,
                    side: &_swap.side,
                    quantity: &_swap.quantity.to_string(),
                    price : &ret.get("data").unwrap()[0].get("avgPx").unwrap().as_str().unwrap(),
                    swapi_fee : &swapi_fee.to_string(),
                    fee : &ret.get("data").unwrap()[0].get("fee").unwrap().as_str().unwrap(),
                    fee_currency: &ret.get("data").unwrap()[0].get("feeCcy").unwrap().as_str().unwrap(),
                    time_satmp: &_swap.time_satmp.to_string(),
                };
                db::save_swap(rt.clone());
                Ok(warp::reply::json(
                    &rt
                ))
            } else {
                Ok(warp::reply::json(
                    &ErrorResponse{msg:"El tiempo maximo de espera ha sido superado".to_string(),err_nro:3}
                ))
            }
        }
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
    let swapi = Swapi::new(okex::get_instruments().await.unwrap_or(json!(null)));
    //swapi.instruments = okex::get_instruments().await.unwrap_or(json!(null));

    println!("Iniciando con Fee de {}%\nTiempo de espera antes de anular el precio {}s segundos\nTolerancia de variacion de precio {}%",swapi.clone().swap_fee*100.0,swapi.clone().max_swap_wait,swapi.clone().swap_price_tolerance);
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

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8099))
        .await;
}
