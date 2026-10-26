use axum::{
    Router
};

use crate::{
    routes::card_route::card_routes
};

pub async fn start() {

    let router = Router::new()
        .nest("/cards",card_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}