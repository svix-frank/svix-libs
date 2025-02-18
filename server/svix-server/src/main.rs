// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use axum::{AddExtensionLayer, Router};
use bb8_redis::RedisConnectionManager;
use cfg::QueueType;
use dotenv::dotenv;
use std::{net::SocketAddr, process::exit, str::FromStr};
use tower_http::trace::TraceLayer;

use crate::{core::security::generate_token, db::init_db, worker::worker_loop};

mod cfg;
mod core;
mod db;
mod error;
mod queue;
mod v1;
mod worker;

const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// JWT utilities
    #[clap()]
    Jwt {
        #[clap(subcommand)]
        command: JwtCommands,
    },
}

#[derive(Subcommand)]
enum JwtCommands {
    /// Generate a new JWT
    #[clap()]
    Generate,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();
    let cfg = cfg::load().unwrap();

    if cfg!(debug_assertions) && std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{crate}={level},tower_http={level}",
                crate = CRATE_NAME,
                level = cfg.log_level.to_string()
            ),
        );
    }

    tracing_subscriber::fmt::init();

    if let Some(Commands::Jwt {
        command: JwtCommands::Generate,
    }) = &args.command
    {
        let token = generate_token(&cfg.jwt_secret).unwrap();
        println!("Token (Bearer): {}", token);
        exit(0);
    }

    let pool = init_db(&cfg).await;
    let redis_pool = if let Some(redis_dsn) = &cfg.redis_dsn {
        tracing::debug!("Redis: Initializing pool");
        let manager = RedisConnectionManager::new(redis_dsn.to_string()).unwrap();
        Some(bb8::Pool::builder().build(manager).await.unwrap())
    } else {
        None
    };

    tracing::debug!("Queue type: {:?}", cfg.queue_type);
    let (queue_tx, queue_rx) = match cfg.queue_type {
        QueueType::Memory => queue::memory::new_pair().await,
        QueueType::Redis => {
            queue::redis::new_pair(redis_pool.clone().unwrap_or_else(|| {
                panic!("Choosing queue_type=Redis requires setting a redis DSN.")
            }))
            .await
        }
    };

    // build our application with a route
    let mut app = Router::new()
        .nest("/api/v1", v1::router())
        .layer(TraceLayer::new_for_http().on_request(()))
        .layer(AddExtensionLayer::new(pool.clone()))
        .layer(AddExtensionLayer::new(queue_tx.clone()))
        .layer(AddExtensionLayer::new(cfg.clone()));

    if let Some(redis_pool) = &redis_pool {
        app = app.layer(AddExtensionLayer::new(redis_pool.clone()));
    };

    let addr = SocketAddr::from_str(&cfg.listen_address).unwrap();
    let with_api = cfg.api_enabled;
    let with_worker = cfg.worker_enabled;
    let (server, worker_loop) = tokio::join!(
        async {
            if with_api {
                tracing::debug!("API: Listening on {}", addr);
                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .await
            } else {
                tracing::debug!("API: off");
                Ok(())
            }
        },
        async {
            if with_worker {
                tracing::debug!("Worker: Initializing");
                worker_loop(cfg, pool, queue_tx, queue_rx).await
            } else {
                tracing::debug!("Worker: off");
                Ok(())
            }
        },
    );
    server.unwrap();
    worker_loop.unwrap();
}
