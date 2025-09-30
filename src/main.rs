mod controllers;
mod errors;
mod models;

use axum::{Router, extract::FromRef, routing::get, serve};
use axum_template::engine::Engine;
use dotenvy::dotenv;
use minijinja::{Environment, path_loader};
use minijinja_autoreload::AutoReloader;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{
    env,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use tokio::net::TcpListener;

use crate::{
    controllers::{get_troop_train, get_troops, post_troop_train},
    errors::ErrorHandler,
};

type AppEngine = Engine<AutoReloader>;

#[derive(FromRef)]
struct AppState {
    db_pool: Pool<Postgres>,
    engine: AppEngine,
}


fn get_attack_power(r#type: &str, tribe: &str) -> Result<i32, ErrorHandler> {
    let mut key = r#type.to_string();

    key.push_str(tribe);

    match key.as_str() {
        "InfantryGallia" => Ok(65),
        "InfantryTeuton" => Ok(60),
        "InfantryRoman" => Ok(70),
        "CavalryGallia" => Ok(140),
        "CavalryTeuton" => Ok(150),
        "CavalryRoman" => Ok(160),
        _ => Err(ErrorHandler::TroopTypeTribe(format!(
            "type: {}, tribe: {}",
            r#type, tribe
        ))),
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("Cannot read .env file!");

    let db_conn_str = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/database".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_conn_str)
        .await
        .expect("can't connect to database");

    let jinja = AutoReloader::new(move |notifier| {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("views");

        let mut env = Environment::new();
        env.set_loader(path_loader(&template_path));

        notifier.set_fast_reload(true);
        notifier.watch_path(&template_path, true);

        Ok(env)
    });

    let app_state = Arc::new(AppState {
        engine: Engine::from(jinja),
        db_pool: pool,
    });

    let app = Router::new()
        .route("/", get(controllers::get_home))
        .route("/commanders", get(controllers::get_commanders))
        .route("/troops", get(get_troops))
        .route("/troops/train", get(get_troop_train).post(post_troop_train))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Can't listen on port 3000!");

    println!(
        "🚀 Server listening on http://{}",
        listener.local_addr().unwrap()
    );

    serve(listener, app).await.unwrap();
}
