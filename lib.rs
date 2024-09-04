pub mod config;
pub mod error;
pub mod models;
pub mod services;
pub mod utils;
pub use crate::smart_contracts::utils_contract::format_balance;

pub mod smart_contracts {
    pub mod token_contract;
    pub mod utils_contract;
    pub mod storage;
    
    pub use token_contract::{TokenData, initialize_supply, get_total_supply};
}
