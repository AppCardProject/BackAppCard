mod handlers;
mod routes;


use crate::{
    routes::global::start
};

#[tokio::main]
async fn main() {
    start().await
}


