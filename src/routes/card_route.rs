use axum::{Router, routing::{get, post}, Json};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::entity::cards::Model;
use crate::entity::prelude::Cards;
use crate::handlers::query::Query;
use crate::routes::global::AppState;




async fn list_all_cards(state: State<AppState>) -> Result<Json<Vec<Model>>,StatusCode> {
    Ok(Json(Query::fetch_all_cards(&state.db_con).await.expect("FUCKKKKK")))
}

async fn get_card_by_title(title : Path<String>){

}

async fn get_card_by_id(id : Path<u32>){

}

async fn add_card(Json(card): Json<Cards>){

}


pub fn card_routes() -> Router<AppState> {
    Router::new()
        .route("/card",get(list_all_cards))
        .route("/card/{id}",get(get_card_by_id))
}