use actix_web::Responder;
use actix_web::{ web,};
use crate::db;
use crate::structs::structs::{CreateRequestBody, QueryParams};

// pub async fn get_chunk(
//     pool: web::Data<db::db::DbPool>,
//     query: web::Query<QueryParams>,
// )-> impl Responder{
//     let pool = pool.clone();
// }