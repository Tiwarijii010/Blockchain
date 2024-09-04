// src/routes.rs

use crate::models::{User, TransferData};
use crate::services::user::{register_user, login_user};
use crate::services::token::{transfer_tokens, get_user_info, list_users};
use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};
use rocket::response::Redirect;
use std::io;
use uuid::Uuid;

#[post("/register", data = "<user_data>")]
pub async fn register(user_data: Json<(String, String, u64)>) -> Result<Redirect, io::Error> {
    let (name, email, tokens) = user_data.into_inner();
    let _user_id = register_user(name, email, tokens)?;
    Ok(Redirect::to("/home"))
}

#[post("/login", data = "<email>")]
pub async fn login(email: Json<String>) -> Result<Json<Option<User>>, io::Error> {
    let user = login_user(&email)?;
    Ok(Json(user))
}

#[get("/home/<user_id>")]
pub async fn home(user_id: Uuid) -> Result<Json<Option<User>>, io::Error> {
    let user_info = get_user_info(user_id)?;
    Ok(Json(user_info))
}

#[get("/users")]
pub async fn users() -> Result<Json<Vec<User>>, io::Error> {
    let user_list = list_users()?;
    Ok(Json(user_list))
}

#[post("/transfer", data = "<transfer_data>")]
pub async fn transfer(transfer_data: Json<TransferData>) -> Result<Json<String>, io::Error> {
    transfer_tokens(transfer_data.into_inner())?;
    Ok(Json("Transfer successful".to_string()))
}

pub fn routes() -> Vec<Route> {
    routes![register, login, home, users, transfer]
}
