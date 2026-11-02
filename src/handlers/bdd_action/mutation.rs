use crate::entity::{cards, cards::Entity as Cards};
use crate::entity::cards::{ActiveModel, Model};
use sea_orm::*;
use serde::Serialize;
use crate::routes::card_route::CreateInputCard;
pub struct Mutation{}


impl Mutation {


    pub async fn add_card(db : &DbConn, card : CreateInputCard) -> Result<Model, DbErr> {
       let new_card = ActiveModel {
            title: Set(card.title),
            content: Set(card.content),
            ..Default::default()
       };
        new_card
            .insert(db)
            .await
    }
}