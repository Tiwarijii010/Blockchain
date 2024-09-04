// src/models.rs

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub account_number: String,
    pub tokens: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TransferData {
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub amount: u64,
}