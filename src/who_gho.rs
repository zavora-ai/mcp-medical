use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::types::{HealthIndicator, IndicatorMeta};

pub struct WhoGho {
    client: Client,
}

/// Common health indicators
pub const COMMON_INDICATORS: &[(&str, &str)] = &[
    ("WHOSIS_000001", "Life expectancy at birth (years)"),
    ("WHOSIS_000002", "Healthy life expectancy (HALE) at birth (years)"),
    ("MDG_0000000026", "Maternal mortality ratio (per 100,000 live births)"),
    ("MDG_0000000001", "Infant mortality rate (per 1000 live births)"),
    ("NCD_BMI_30A", "Prevalence of obesity among adults (%)"),
    ("WHS4_100", "Physicians density (per 10,000 population)"),
    ("WHS4_117", "Nursing and midwifery personnel density (per 10,000)"),
    ("MALARIA_EST_INCIDENCE", "Estimated malaria incidence (per 1000 at risk)"),
    ("HIV_0000000001", "Estimated HIV prevalence (15-49 years, %)"),
    ("TB_e_inc_100k", "Tuberculosis incidence (per 100,000)"),
];

impl WhoGho {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn get_indicator(&self, indicator_code: &str, country_code: &str, limit: u32) -> Result<Vec<HealthIndicator>> {
        let url = format!(
            "https://ghoapi.azureedge.net/api/{}?$filter=SpatialDim+eq+%27{}%27&$top={}&$orderby=TimeDim+desc",
            indicator_code, country_code, limit
        );
        let resp: Value = self.client.get(&url).send().await?.json().await?;
        let items = resp["value"].as_array().unwrap_or(&vec![]).clone();
        Ok(items.iter().map(|i| HealthIndicator {
            indicator_code: indicator_code.to_string(),
            indicator_name: String::new(),
            country: i["SpatialDim"].as_str().unwrap_or_default().to_string(),
            country_code: i["SpatialDim"].as_str().unwrap_or_default().to_string(),
            year: i["TimeDim"].as_str().or(i["TimeDim"].as_i64().map(|_| "").into()).unwrap_or_default().to_string(),
            value: i["NumericValue"].as_f64(),
            dimension: i["Dim1"].as_str().map(String::from),
        }).collect())
    }

    pub async fn get_country_profile(&self, country_code: &str) -> Result<Vec<HealthIndicator>> {
        let mut results = Vec::new();
        for (code, name) in COMMON_INDICATORS {
            if let Ok(mut indicators) = self.get_indicator(code, country_code, 1).await {
                for ind in &mut indicators {
                    ind.indicator_name = name.to_string();
                }
                results.extend(indicators);
            }
        }
        Ok(results)
    }

    pub async fn compare_countries(&self, indicator_code: &str, countries: &[&str]) -> Result<Vec<HealthIndicator>> {
        let mut results = Vec::new();
        for country in countries {
            if let Ok(indicators) = self.get_indicator(indicator_code, country, 1).await {
                results.extend(indicators);
            }
        }
        Ok(results)
    }

    pub async fn search_indicators(&self, query: &str, limit: u32) -> Result<Vec<IndicatorMeta>> {
        let url = format!(
            "https://ghoapi.azureedge.net/api/Indicator?$filter=contains(IndicatorName,%27{}%27)&$top={}",
            query.replace(' ', "%20"), limit
        );
        let resp: Value = self.client.get(&url).send().await?.json().await?;
        let items = resp["value"].as_array().unwrap_or(&vec![]).clone();
        Ok(items.iter().map(|i| IndicatorMeta {
            code: i["IndicatorCode"].as_str().unwrap_or_default().to_string(),
            name: i["IndicatorName"].as_str().unwrap_or_default().to_string(),
        }).collect())
    }
}
