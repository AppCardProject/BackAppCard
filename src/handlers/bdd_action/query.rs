use crate::entity::{cards, cards::Entity as Cards};
use sea_orm::*;

pub struct Query;


impl Query {
    
    pub async fn fetch_all_cards(db : &DbConn) -> Result<Vec<cards::Model>,DbErr> {
        Cards::find().all(db).await
    }



}