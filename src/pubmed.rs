use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::types::PubMedArticle;

#[derive(Clone)]
pub struct PubMed {
    client: Client,
}

impl PubMed {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn search(&self, query: &str, limit: u32) -> Result<Vec<String>> {
        let url = format!(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=pubmed&term={}&retmax={}&retmode=json&sort=relevance",
            query.replace(' ', "+"), limit
        );
        let resp: Value = self.client.get(&url).send().await?.json().await?;
        let ids = resp["esearchresult"]["idlist"].as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        Ok(ids)
    }

    pub async fn get_summaries(&self, pmids: &[String]) -> Result<Vec<PubMedArticle>> {
        if pmids.is_empty() { return Ok(vec![]); }
        let ids = pmids.join(",");
        let url = format!(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esummary.fcgi?db=pubmed&id={}&retmode=json",
            ids
        );
        let resp: Value = self.client.get(&url).send().await?.json().await?;
        let result = resp["result"].as_object();
        let mut articles = Vec::new();
        if let Some(obj) = result {
            for pmid in pmids {
                if let Some(article) = obj.get(pmid) {
                    let authors: Vec<String> = article["authors"].as_array()
                        .map(|a| a.iter().filter_map(|au| au["name"].as_str().map(String::from)).collect())
                        .unwrap_or_default();
                    articles.push(PubMedArticle {
                        pmid: pmid.clone(),
                        title: article["title"].as_str().unwrap_or_default().to_string(),
                        authors,
                        journal: article["source"].as_str().map(String::from),
                        pub_date: article["pubdate"].as_str().map(String::from),
                        doi: article["elocationid"].as_str().map(String::from),
                        abstract_text: None,
                    });
                }
            }
        }
        Ok(articles)
    }

    pub async fn search_and_summarize(&self, query: &str, limit: u32) -> Result<Vec<PubMedArticle>> {
        let ids = self.search(query, limit).await?;
        self.get_summaries(&ids).await
    }

    pub async fn get_abstract(&self, pmid: &str) -> Result<Option<String>> {
        let url = format!(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=pubmed&id={}&retmode=xml&rettype=abstract",
            pmid
        );
        let xml = self.client.get(&url).send().await?.text().await?;
        // Extract abstract from XML
        let abstract_text = extract_between(&xml, "<AbstractText>", "</AbstractText>")
            .or_else(|| extract_between(&xml, "<AbstractText ", "</AbstractText>").map(|s| {
                // Handle attributed AbstractText tags
                s.split_once('>').map(|(_, rest)| rest.to_string()).unwrap_or(s)
            }));
        Ok(abstract_text)
    }
}

fn extract_between(text: &str, start_tag: &str, end_tag: &str) -> Option<String> {
    let start = text.find(start_tag)?;
    let content_start = start + start_tag.len();
    let end = text[content_start..].find(end_tag)?;
    let content = &text[content_start..content_start + end];
    // Strip any remaining XML tags
    let clean: String = content.chars().fold((String::new(), false), |(mut acc, in_tag), c| {
        if c == '<' { (acc, true) }
        else if c == '>' { (acc, false) }
        else if !in_tag { acc.push(c); (acc, false) }
        else { (acc, true) }
    }).0;
    Some(clean.trim().to_string())
}
