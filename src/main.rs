mod handlers;
mod routes;

mod entity;

use crate::{
    routes::global_routeur::start
};

#[tokio::main]
async fn main() {
    start().await
}


