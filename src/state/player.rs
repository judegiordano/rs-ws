use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Player {
    #[allow(dead_code)]
    #[serde(skip)]
    pub id: Uuid,
    pub display_name: String,
}
