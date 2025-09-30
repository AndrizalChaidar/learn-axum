use crate::{AppState, errors::ErrorHandler};
use axum::{extract::State, response::IntoResponse};
use axum_template::RenderHtml;
use chrono::{DateTime, Utc};
use minijinja::context;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct Empty;

#[allow(dead_code)]
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
struct Commanders {
    id: Uuid,
    name: Option<String>,
    nation: Option<String>,
    age: Option<i16>,
    military_force: Option<i32>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    total_troops: Option<i64>,
}

pub async fn get_home(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ErrorHandler> {
    Ok(RenderHtml("index.html", state.engine.clone(), Empty {}))
}

pub async fn get_commanders(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ErrorHandler> {
    let commanders = sqlx::query_as!(
        Commanders,
        r#"SELECT 
                c.id,
                CASE 
                    WHEN c.military_force >= 500 THEN 'General ' || c.name
                    WHEN c.military_force >= 100 THEN 'Major ' || c.name
                    ELSE 'Sergeant ' || c.name
                END AS name,
                nation,
                age,
                military_force,
                c.created_at,
                c.updated_at,
                count(t.id) AS total_troops 
            FROM commanders c
            LEFT JOIN troops t ON c.id = t.id
            GROUP BY c.id;"#
    )
    .fetch_all(&state.db_pool)
    .await?;

    Ok(RenderHtml(
        "getCommanders.html",
        state.engine.clone(),
        context! {commanders => commanders},
    ))
}
