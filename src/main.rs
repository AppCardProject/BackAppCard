mod handlers;
mod routes;
mod entity;

use crate::{
    routes::global::start
};

#[tokio::main]
async fn main() {
    start().await
}


