// use std::collections::VecDeque;
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use deadpool_postgres::Pool;

use warp::{filters::body::BodyDeserializeError, http::StatusCode, Filter, Rejection};

mod db;
mod handlers;
mod models;

type Result<T> = std::result::Result<T, Rejection>;
type DBConnection = Arc<Mutex<Pool>>;

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let db: DBConnection = Arc::new(Mutex::new(db::new_pool().unwrap()));

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let extrato = warp::path!("clientes" / i32 / "extrato")
        .and(warp::get())
        .and(with_db(Arc::clone(&db)))
        .and_then(handlers::get_client_balance);

    let transacoes = warp::path!("clientes" / i32 / "transacoes")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(Arc::clone(&db)))
        .and_then(handlers::add_client_transaction)
        .recover(|err: Rejection| async move {
            if let Some(_) = err.find::<BodyDeserializeError>() {
                Ok(StatusCode::UNPROCESSABLE_ENTITY)
            } else {
                Err(err)
            }
        });

    let routes = root
        .or(extrato)
        .or(transacoes)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}

fn with_db(
    database: DBConnection,
) -> impl Filter<Extract = (DBConnection,), Error = Infallible> + Clone {
    // warp::any().map(move || database.clone())
    warp::any().map(move || Arc::clone(&database))
}
