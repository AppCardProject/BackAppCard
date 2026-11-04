use crate::entity::{cards, cards::Entity as Cards};
use sea_orm::*;
use anyhow::*;
pub struct Query;


impl Query {
    
    pub async fn fetch_all_cards(db : &DbConn) -> Result<Vec<cards::Model>> {
        Cards::find().all(db).await.context("Error when fetching DB data")
    }



}