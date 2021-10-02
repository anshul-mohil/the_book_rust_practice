// couchbase with actix-web
// https://users.rust-lang.org/t/use-couchbase-with-actix-web/59108/8
// use std::env;
//
// use actix_web::{get, web, App, HttpResponse, HttpServer};
// use couchbase::{Cluster, Collection, GetOptions};
// use serde::{Deserialize, Serialize};
//
// #[derive(Deserialize)]
// pub struct QueryParameters {
//     stuff: String,
// }
//
// #[derive(Deserialize, Serialize)]
// pub struct CouchbaseDocument {
//     click: String,
//     #[serde(rename = "with JSON")]
//     with_json: String,
// }
//
// #[get("/")]
// async fn index(
//     parameters: web::Query<QueryParameters>,
//     collection: web::Data<Arc<Collection>>,
// ) -> HttpResponse {
//     let digest = format!("{:x}", md5::compute(parameters.stuff.as_bytes()));
//     let doc = collection.get(digest, GetOptions::default()).await.unwrap();
//
//     HttpResponse::Ok()
//         .content_type("application/json")
//         .json(doc.content::<CouchbaseDocument>().unwrap())
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let cb_url = env::var("COUCHBASE_URL").unwrap();
//     let cb_user = env::var("COUCHBASE_USER").unwrap();
//     let cb_password = env::var("COUCHBASE_PASSWORD").unwrap();
//     let cb_bucket = env::var("COUCHBASE_BUCKET").unwrap();
//
//     let cluster = Cluster::connect(cb_url, cb_user, cb_password);
//     let bucket = cluster.bucket(cb_bucket);
//     let collection = bucket.default_collection();
//     let collection_arc = Arc::new(collection);
//
//     HttpServer::new(move || App::new().service(index).data(collection_arc.clone()))
//         .bind(env::var("LISTEN_ON").unwrap_or_else(|_| String::from("0.0.0.0:8080")))?
//         .run()
//         .await
// }
