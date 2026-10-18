//! Startup check for a newer GitHub release. Read-only, unauthenticated, nothing is downloaded or installed.
use std::time::Duration;

use reqwest::{Client, redirect::Policy};
use serde::Deserialize;

use crate::error::{AppError, AppResult};

pub const REPO_URL: &str = "https://github.com/Tar-Tarj/Civitai-companion";
pub const RELEASES_URL: &str = "https://github.com/Tar-Tarj/Civitai-companion/releases/latest";
const LATEST_RELEASE_API: &str =
    "https://api.github.com/repos/Tar-Tarj/Civitai-companion/releases/latest";
const MAX_RESPONSE_BYTES: usize = 512 * 1024;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

/// Newest published version when it is higher than `current`.
pub async fn newer_release(current: &str) -> AppResult<Option<String>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(Policy::none())
        .user_agent(concat!(
            "Civitai-Companion-Desktop/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .map_err(|_| AppError::Operation)?;
    let mut response = client
        .get(LATEST_RELEASE_API)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|_| AppError::Network)?;
    if !response.status().is_success() {
        return Err(AppError::InvalidResponse);
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| AppError::Network)? {
        if bytes.len() + chunk.len() > MAX_RESPONSE_BYTES {
            return Err(AppError::InvalidResponse);
        }
        bytes.extend_from_slice(&chunk);
    }
    let release: Release = serde_json::from_slice(&bytes).map_err(|_| AppError::InvalidResponse)?;
    let latest = release.tag_name.trim_start_matches('v').to_string();
    Ok(is_newer(&latest, current).then_some(latest))
}

fn parse_version(value: &str) -> Option<[u64; 3]> {
    let mut parts = value.split('.').map(|part| part.parse::<u64>().ok());
    let version = [parts.next()??, parts.next()??, parts.next()??];
    parts.next().is_none().then_some(version)
}

fn is_newer(latest: &str, current: &str) -> bool {
    matches!((parse_version(latest), parse_version(current)), (Some(latest), Some(current)) if latest > current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_higher_version_counts_as_an_update() {
        assert!(is_newer("1.0.27", "1.0.26"));
        assert!(is_newer("1.1.0", "1.0.99"));
        assert!(!is_newer("1.0.26", "1.0.26"));
        assert!(!is_newer("1.0.25", "1.0.26"));
        assert!(!is_newer("garbage", "1.0.26"));
        assert!(!is_newer("1.0.27.1", "1.0.26"));
    }
}
