use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Job {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewJob {
    pub name: String,
}

#[derive(Deserialize, Clone)]
pub struct TimeWindow {
    pub left: String,
    pub right: String,
}

#[derive(Serialize, Deserialize)]
pub struct Doc {
    pub _id: String,
    pub _rev: String,
    pub doc_ids: Vec<i32>,
    pub doc_scores: Vec<f32>,
    pub doc_texts: Vec<String>,
    pub keywords: Vec<String>,
    pub kw_scores: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Events {
    pub docs: Vec<Doc>,
}

#[derive(Serialize)]
pub struct Selector {
    pub selector: DateFilter,
}

#[derive(Serialize)]
pub struct DateFilter {
    pub date: HashMap<String, String>,
}
