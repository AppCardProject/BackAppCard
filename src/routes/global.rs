use axum::{
    Router
};

use crate::{
    routes::test_route::routes1,
    routes::test_route_2::routes2
};

pub async fn start() {

    let router = Router::new()
        .nest("/n1",routes1())
        .nest("/n2",routes2());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}