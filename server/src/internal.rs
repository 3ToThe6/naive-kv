//! Services. Service "functions" defined here are in fact ``pub struct``s, so this submodule is needed to make them private.

use actix_web::{
    web::{self, Bytes},
    HttpResponse, Result,
};

use common::req::{Delete, Get, Put, Scan};

use super::*;

#[actix_web::get("/")]
async fn get(data: web::Data<AppData>, info: web::Json<Get>) -> Result<Bytes> {
    let kv = &data.kvmg()?.kv;
    Ok(bincode::serialize(&kv.get(&info.key)).map_err(ise)?.into())
}

#[actix_web::get("/scan")]
async fn scan(data: web::Data<AppData>, info: web::Json<Scan>) -> Result<Bytes> {
    use std::ops::Bound::{Excluded as E, Included as I, Unbounded as U};
    let s = info.start.map_or(U, |(s, e)| if !e { I(s) } else { E(s) });
    let e = info.end.map_or(U, |(e, i)| if !i { E(e) } else { I(e) });
    let kv = &data.kvmg()?.kv;
    let range = kv.range((s, e)).collect::<Vec<_>>();
    Ok(bincode::serialize(&range).map_err(ise)?.into())
}

#[actix_web::put("/")]
async fn put(data: web::Data<AppData>, info: web::Json<Put>) -> Result<HttpResponse> {
    let web::Json(Put { key, value }) = info;
    data.kvmg_mut()?.kv.insert(key, value);
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::delete("/")]
async fn delete(data: web::Data<AppData>, info: web::Json<Delete>) -> Result<HttpResponse> {
    data.kvmg_mut()?.kv.remove(&info.key);
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::post("/stop")]
async fn stop(data: web::Data<AppData>) -> Result<HttpResponse> {
    data.tx.send(()).unwrap();
    Ok(HttpResponse::Ok().finish())
}
