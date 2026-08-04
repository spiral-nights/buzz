//! `buzz invites` — mint and read community invite codes.
//!
//! Community invites are role-gated HTTP endpoints (`POST /api/invites`),
//! authenticated via NIP-98 signed events. An invite code is claimed by the
//! *joining* device to admit itself into the workspace — minting is the
//! owner/admin-side operation this module exposes.

use crate::client::BuzzClient;
use crate::error::CliError;
use crate::InvitesCmd;

pub async fn dispatch(command: InvitesCmd, client: &BuzzClient) -> Result<(), CliError> {
    match command {
        InvitesCmd::Mint { ttl_secs, max_uses } => {
            cmd_mint(client, ttl_secs, max_uses).await
        }
    }
}

/// Mint a community invite code via `POST /api/invites` (NIP-98 signed).
async fn cmd_mint(
    client: &BuzzClient,
    ttl_secs: Option<u64>,
    max_uses: Option<i32>,
) -> Result<(), CliError> {
    let mut body = serde_json::Map::new();
    if let Some(ttl) = ttl_secs {
        body.insert("ttl_secs".into(), ttl.into());
    }
    if let Some(uses) = max_uses {
        body.insert("max_uses".into(), uses.into());
    }
    let payload = serde_json::Value::Object(body);

    let resp = client.post_authed("/api/invites", &payload).await?;
    println!("{resp}");
    Ok(())
}
