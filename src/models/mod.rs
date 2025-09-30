use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Commander {
    pub id: Uuid,
    pub name: Option<String>,
    pub nation: Option<String>,
    pub age: Option<i16>,
    pub military_force: Option<i32>,
    pub total_troops: Option<i64>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct IdNameCommander {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Troop {
    pub name: Option<String>,
    pub tribe: Option<String>,
    pub r#type: Option<String>,
    pub attack_power: Option<i16>,
    pub commander_name: Option<String>,
}
