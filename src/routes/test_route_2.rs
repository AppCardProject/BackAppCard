use axum::{
    Router,
    routing::{get},
};

use crate::{
    handlers::handler::{
        test2,test3,test1
    }
};

pub fn routes2() -> Router {
    Router::new()
        .route("/1",get(test1))
        .route("/2",get(test2))
        .route("/3",get(test3))
}