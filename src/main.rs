mod args;
mod axumserver;
mod beedata;
mod beesearch;
mod graph;
mod gtfparse;
use crate::args::CommandParse;
use crate::args::Commands;
use axum::{
    Form, Router,
    extract::State,
    routing::{get, post},
};
use axum_extra::extract::Form as TypedForm;
use clap::Parser;
use handlebars::Handlebars;
use serde::Deserialize;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tower_http::services::ServeDir;

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

#[tokio::main]
async fn main() {}
