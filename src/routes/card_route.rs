use std::num::NonZeroU16;
use axum::{Router, routing::{get, post}, Json};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::entity::cards;
use crate::entity::cards::{ActiveModel, Model};
use crate::entity::prelude::Cards;
use crate::handlers::bdd_action::query::Query;
use crate::routes::global_routeur::AppState;
use crate::handlers::bdd_action::mutation::*;
#[derive(Deserialize)]
pub struct CreateInputCard {
    pub title : String,
    pub content : String
}




async fn list_all_cards(state: State<AppState>) -> Result<Json<Vec<Model>>,StatusCode> {
    Ok(Json(Query::fetch_all_cards(&state.db_con).await.expect("FUCKKKKK")))
}

async fn get_card_by_title(title : Path<String>){

}

async fn get_card_by_id(id : Path<u32>){

}

async fn add_card(State(state): State<AppState>,Json(card): Json<CreateInputCard>) -> Result<Json<cards::Model>,StatusCode>{
    Mutation::add_card(&state.db_con, card)
        .await
        .map_err(|_| StatusCode::NOT_MODIFIED)
        .map(|a| Json(a))
}


pub fn card_routes() -> Router<AppState> {
    Router::new()
        .route("/card",post(add_card))
        .route("/card",get(list_all_cards))
        .route("/card/{id}",get(get_card_by_id))
}