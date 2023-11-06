use std::collections::HashMap;

use color_eyre::eyre::Result;
use dashmap::DashMap;
use nots_client::api::*;
use opendal::Operator;
use serde::{de::DeserializeOwned, Serialize};

pub mod file;
pub mod kv;

pub fn fs_operator(path: &str) -> Result<opendal::Operator> {
    let mut builder = opendal::services::Fs::default();
    builder.root(path);

    let op: Operator = Operator::new(builder)?.finish();
    Ok(op)
}

pub fn persy_operator(path: &str) -> Result<opendal::Operator> {
    let mut builder = opendal::services::Persy::default();
    builder.datafile(path);
    builder.segment("data");
    builder.index("index");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(op)
}
