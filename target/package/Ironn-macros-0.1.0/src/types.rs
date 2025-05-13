use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
pub use actix_web::{web, App, Error, HttpResponse, HttpServer};
pub type HandlerFuture = Pin<Box<dyn Future<Output = HttpResponse> + Send>>;
pub type IronnHandler = Arc<dyn Fn() -> HandlerFuture + Send + Sync>;