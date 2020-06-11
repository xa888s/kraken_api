pub mod api;
mod crypto;
mod error;

pub use log;
use std::error::Error;

// saving some typing
type GenError = Box<dyn Error + Send + Sync + 'static>;
