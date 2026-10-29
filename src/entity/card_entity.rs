
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[sea_orm::model]
#[derive(Clone,Deserialize,Serialize,DeriveEntityModel,PartialEq,Eq,Debug)]
#[sea_orm(table_name = "Cards")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u32,
    pub title : String,
    #[sea_orm(column_type = "Text")]
    pub content : String
}

impl ActiveModelBehavior for ActiveModel {}