# Changelog

## [1.1.0] - 2026-08-13

### Changed
- Upgraded to rmcp 3.1.2 and raised the minimum supported Rust version to 1.94.1.
- Added MCP 2026-07-28 stateless request handling while retaining MCP 2025-11-25 initialization compatibility.

### Added
- Per-request identity and protocol metadata, on-demand discovery/cache hints, and the configured Tasks and sealed MRTR approval policies.

## [1.0.0] — 2026-05-26

### Added
- **PubMed backend** — article search (36M+ papers), abstract retrieval by PMID
- **WHO GHO backend** — health indicators for 194 countries, country profiles, cross-country comparison, indicator search
- 7 tools total, all read-only, zero credentials required
- 10 pre-configured common health indicators (life expectancy, maternal mortality, HIV, TB, malaria, obesity, physician density)
- Registry-compatible `mcp-server.toml` manifest
