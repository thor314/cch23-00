#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use error::MyError;

mod error;
#[cfg(test)] mod tests;
mod utils;

use axum::{routing::get, Router};

async fn hello_world() -> &'static str { "Hello, world!" }

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
  utils::setup().unwrap();

  let router = Router::new().route("/", get(hello_world));

  Ok(router.into())
}
