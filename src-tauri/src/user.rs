use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use user_service::UserService;

use crate::service::Service;

pub mod user_service;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct User {
    id: u32,
    username: String,
}
