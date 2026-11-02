use crate::routes::card_route::card_routes;
use axum::Router;
use sea_orm::DatabaseConnection;
use std::env;
use dotenvy::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub db_con : DatabaseConnection
}

pub async fn start() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not define in .env");
    let db_con = sea_orm::Database::connect(db_url).await.expect("BDD connexion failed");

    let state = AppState{db_con};

    let router = Router::new()
        .nest("/cards",card_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}