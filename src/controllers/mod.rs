use crate::{
    AppState,
    errors::ErrorHandler,
    models::{Commander, IdNameCommander, Troop},
};
use axum::{extract::{Query, State}, response::IntoResponse};
use axum_template::RenderHtml;
use minijinja::context;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types};
use uuid::Uuid;
use std::{collections::HashMap, sync::Arc};

pub type TJson<T> = types::Json<T>;

#[derive(Debug, Serialize)]
struct Empty;

pub async fn get_home(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ErrorHandler> {
    Ok(RenderHtml("index.html", state.engine.clone(), Empty {}))
}

pub async fn get_commanders(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ErrorHandler> {
    let commanders = sqlx::query_as!(
        Commander,
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
                count(t.id) AS total_troops 
            FROM commanders c
            LEFT JOIN troops t ON c.id = t.id
            GROUP BY c.id;"#
    )
    .fetch_all(&state.db_pool)
    .await?;

    Ok(RenderHtml(
        "commanders.html",
        state.engine.clone(),
        context! {commanders => commanders},
    ))
}

pub async fn get_troops(
    Query(param): Query<HashMap<String,String>>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ErrorHandler> {
    let mut commander_id: Option<Uuid> = None;
    
    if let Some(param) = param.get("commander_id") && param.len() > 0 {
        commander_id = Some(Uuid::parse_str(param)?)    
    }

    #[derive(Debug, FromRow, Serialize, Deserialize)]
    struct Result {
        troops: Option<TJson<Vec<Troop>>>,
        commanders: Option<TJson<Vec<IdNameCommander>>>,
    }

    let result =
            sqlx::query_as!(Result,
                r#"
                    WITH commanders_cte AS (
                        SELECT id, name FROM commanders
                    ),
                    troops_cte AS (
                        SELECT t."name" , t.tribe, t."type", t.attack_power, cc.name AS "commander_name" FROM troops t
                        JOIN (
                            SELECT id, name FROM commanders
                            WHERE ($1::uuid IS NULL OR id = $1::uuid)
                        ) cc ON cc.id = t.commander_id
                    )
                    SELECT
                        t.json AS "troops: TJson<Vec<Troop>>",
                        c.json AS "commanders: TJson<Vec<IdNameCommander>>"
                    FROM (
                        SELECT json_agg(t) AS "json" FROM troops_cte t 
                    ) t,
                    (
                        SELECT json_agg(c) AS "json" FROM commanders_cte c 
                    ) c;
                "#, commander_id).fetch_one(&state.db_pool).await?;
    
    Ok(RenderHtml("troops.html", state.engine.clone(), context! {
        troops => result.troops,
        commanders => result.commanders,
        commander_id => commander_id
    }))
}
