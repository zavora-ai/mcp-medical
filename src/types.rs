use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubMedArticle {
    pub pmid: String,
    pub title: String,
    pub authors: Vec<String>,
    pub journal: Option<String>,
    pub pub_date: Option<String>,
    pub doi: Option<String>,
    pub abstract_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIndicator {
    pub indicator_code: String,
    pub indicator_name: String,
    pub country: String,
    pub country_code: String,
    pub year: String,
    pub value: Option<f64>,
    pub dimension: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorMeta {
    pub code: String,
    pub name: String,
}
