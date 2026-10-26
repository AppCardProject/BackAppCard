use axum::{Router, routing::{get, post}, Json};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
struct Card {
    title : String,
    content : String,
    date : String
}


async fn display_card(Json(card): Json<Card>) {
    println!("title : {}, cotent : {}, date : {}",card.title,card.content,card.date)
}

async fn list_all_cards() {

}

async fn get_gard_by_title(title : String){

}

async fn get_card_by_id(id : u32){

}

async fn add_card(Json(card): Json<Card>){

}


pub fn card_routes() -> Router{
    Router::new()
        .route("/card",post(display_card))
        .route("/card",get(list_all_cards))
        .route("/card/{id}",get(get_card_by_id))

}