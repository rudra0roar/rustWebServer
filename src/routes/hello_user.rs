use serde::Serialize;
use actix_web::{http::StatusCode, web::{Json, Path}, Responder , get};
use crate::routes::logging;


#[derive(Serialize)]
pub struct User {
    first_name : String,
    last_name : String,
}

impl User {
    fn new(firstName : String , lastName : String) -> Self{
        Self {first_name : firstName , last_name : lastName}
    }
}


#[get("/hello/{firsname}/{lastname}")]
pub async fn hello_user(params : Path<(String , String)>) -> impl Responder {
    let route = format!("GET:/hello/{}/{}" , params.0.clone() , params.1.clone() );
    logging(&route);
    let response : User = User::new(params.0.clone(), params.1.clone());
    (Json(response) , StatusCode::OK)
}
