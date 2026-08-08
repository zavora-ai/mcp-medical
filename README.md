# mcp-medical

[![Crates.io](https://img.shields.io/crates/v/mcp-medical.svg)](https://crates.io/crates/mcp-medical)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Clinical reference and global health intelligence MCP server — search medical literature, retrieve abstracts, explore WHO health indicators, compare country health profiles. **7 tools** powered by PubMed and WHO Global Health Observatory, all free public APIs.

## What this is

A **medical literature and health statistics** platform for AI agents. It answers:

- "What are the latest studies on malaria vaccine efficacy?"
- "What's Kenya's life expectancy, maternal mortality, and physician density?"
- "Compare HIV prevalence across East African countries"
- "Find WHO indicators related to tuberculosis"

## What this is NOT

This server does **not** provide clinical diagnosis, treatment recommendations, or patient-specific medical advice. It is a reference tool for accessing published research and population-level health statistics.

## Backends

| Backend | Coverage | Free? | What it provides |
|---------|----------|:-----:|-----------------|
| **PubMed** | Global | ✅ | 36M+ medical articles, abstracts, citations |
| **WHO GHO** | Global | ✅ | Health indicators for 194 countries |

## Quick Start

```bash
cargo install mcp-medical

# No credentials needed
mcp-medical
```

## Tools (7)

### PubMed (Medical Literature)
| Tool | Description |
|------|-------------|
| `pubmed_search` | Search articles by topic, condition, treatment, or author |
| `pubmed_get_abstract` | Get full abstract text by PMID |

### WHO Global Health Observatory
| Tool | Description |
|------|-------------|
| `who_get_indicator` | Get a specific indicator for a country over time |
| `who_country_profile` | Comprehensive health profile (10 key indicators) |
| `who_compare_countries` | Compare an indicator across multiple countries |
| `who_search_indicators` | Find available indicators by keyword |
| `who_list_common_indicators` | List common indicator codes |

## Common WHO Indicator Codes

| Code | Indicator |
|------|-----------|
| `WHOSIS_000001` | Life expectancy at birth |
| `WHOSIS_000002` | Healthy life expectancy (HALE) |
| `MDG_0000000026` | Maternal mortality ratio |
| `MDG_0000000001` | Infant mortality rate |
| `NCD_BMI_30A` | Obesity prevalence (adults) |
| `WHS4_100` | Physicians per 10,000 |
| `MALARIA_EST_INCIDENCE` | Malaria incidence |
| `HIV_0000000001` | HIV prevalence (15-49) |
| `TB_e_inc_100k` | Tuberculosis incidence |

## Configuration

```json
{
  "mcpServers": {
    "medical": {
      "command": "mcp-medical"
    }
  }
}
```

## License

Apache-2.0

## rmcp and MCP compatibility

This server is built with [`rmcp` 3.1.2](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.2) and requires Rust 1.88 or newer. The rmcp 3 rollout retains legacy MCP initialization compatibility and targets MCP protocol revisions `2025-11-25` and `2026-07-28`.
