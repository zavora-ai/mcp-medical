mod types;
mod pubmed;
mod who_gho;
mod server;

use rmcp::{ServiceExt, transport::stdio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manifest = adk_mcp_sdk::ServerManifest::from_file(std::path::Path::new("mcp-server.toml"))?;
    let errors = manifest.validate();
    if !errors.is_empty() {
        eprintln!("Manifest warnings:");
        for e in &errors { eprintln!("  - {e}"); }
    }

    let server = server::MedicalServer {
        pubmed: pubmed::PubMed::new(),
        who_gho: who_gho::WhoGho::new(),
    };

    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
