#![feature(format_args_nl)]

pub mod modules;
pub mod outputs;
use reqwest::Client;
use std::{error::Error, result, sync::LazyLock};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub static HTTP: LazyLock<Client> = LazyLock::new(Client::new);
