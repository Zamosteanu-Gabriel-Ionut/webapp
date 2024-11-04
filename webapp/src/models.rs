use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlogPost {
    pub id: i32,
    pub user_name: String,
    pub text: String,
    pub date: DateTime<Utc>,
    pub image_path: Option<String>,
    pub avatar_path: Option<String>,
}
