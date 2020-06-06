pub mod api;
mod crypto;
mod error;

use json::json;
pub use log;
use serde_json as json;
use std::error::Error;

// saving some typing
type GenError = Box<dyn Error + Send + Sync + 'static>;
