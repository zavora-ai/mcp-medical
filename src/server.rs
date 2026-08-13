use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};

use crate::pubmed::PubMed;
use crate::who_gho::WhoGho;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchQuery {
    /// Search query
    pub query: String,
    /// Max results (default 5)
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PmidInput {
    /// PubMed ID
    pub pmid: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CountryIndicatorInput {
    /// WHO indicator code (e.g. WHOSIS_000001 for life expectancy)
    pub indicator_code: String,
    /// ISO 3-letter country code (e.g. KEN, USA, GBR)
    pub country_code: String,
    /// Max results (default 5)
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CountryInput {
    /// ISO 3-letter country code (e.g. KEN, NGA, USA, GBR, IND)
    pub country_code: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CompareInput {
    /// WHO indicator code
    pub indicator_code: String,
    /// ISO 3-letter country codes to compare
    pub countries: Vec<String>,
}

#[derive(Clone)]
pub struct MedicalServer {
    pub pubmed: PubMed,
    pub who_gho: WhoGho,
}

#[tool_router]
impl MedicalServer {
    // --- PubMed ---

    #[tool(description = "Search PubMed for medical literature by topic, condition, or treatment")]
    async fn pubmed_search(&self, Parameters(input): Parameters<SearchQuery>) -> String {
        let limit = input.limit.unwrap_or(5);
        match self.pubmed.search_and_summarize(&input.query, limit).await {
            Ok(articles) => serde_json::to_string_pretty(&articles).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get the abstract of a PubMed article by PMID")]
    async fn pubmed_get_abstract(&self, Parameters(input): Parameters<PmidInput>) -> String {
        match self.pubmed.get_abstract(&input.pmid).await {
            Ok(Some(text)) => text,
            Ok(None) => format!("No abstract found for PMID {}", input.pmid),
            Err(e) => format!("Error: {e}"),
        }
    }

    // --- WHO GHO ---

    #[tool(description = "Get a specific WHO health indicator for a country (e.g. life expectancy, maternal mortality)")]
    async fn who_get_indicator(&self, Parameters(input): Parameters<CountryIndicatorInput>) -> String {
        let limit = input.limit.unwrap_or(5);
        match self.who_gho.get_indicator(&input.indicator_code, &input.country_code, limit).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get a comprehensive health profile for a country (life expectancy, mortality, disease prevalence, workforce)")]
    async fn who_country_profile(&self, Parameters(input): Parameters<CountryInput>) -> String {
        match self.who_gho.get_country_profile(&input.country_code).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Compare a health indicator across multiple countries")]
    async fn who_compare_countries(&self, Parameters(input): Parameters<CompareInput>) -> String {
        let codes: Vec<&str> = input.countries.iter().map(|s| s.as_str()).collect();
        match self.who_gho.compare_countries(&input.indicator_code, &codes).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Search available WHO health indicators by keyword")]
    async fn who_search_indicators(&self, Parameters(input): Parameters<SearchQuery>) -> String {
        let limit = input.limit.unwrap_or(10);
        match self.who_gho.search_indicators(&input.query, limit).await {
            Ok(indicators) => serde_json::to_string_pretty(&indicators).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List common WHO health indicator codes and their descriptions")]
    async fn who_list_common_indicators(&self, Parameters(_input): Parameters<SearchQuery>) -> String {
        let indicators: Vec<serde_json::Value> = crate::who_gho::COMMON_INDICATORS.iter()
            .map(|(code, name)| serde_json::json!({"code": code, "name": name}))
            .collect();
        serde_json::to_string_pretty(&indicators).unwrap_or_default()
    }
}

adk_mcp_sdk::mcp_2026_server! {
    server: MedicalServer,
    task_tools: [],
    approval_tools: [],
    cache_ttl_ms: 60_000,
}
