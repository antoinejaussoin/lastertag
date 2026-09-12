use std::path::PathBuf;
use std::process::Stdio;

use anyhow::{bail, Context, Result};
use tokio::process::Command;
use tracing::info;

pub fn detect_chrome(configured: &str) -> Option<PathBuf> {
    if !configured.trim().is_empty() {
        let p = PathBuf::from(configured.trim());
        if p.exists() {
            return Some(p);
        }
    }
    for name in [
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "chrome",
    ] {
        if let Ok(out) = std::process::Command::new("which").arg(name).output() {
            if out.status.success() {
                let line = String::from_utf8_lossy(&out.stdout);
                let path = line.trim();
                if !path.is_empty() {
                    return Some(PathBuf::from(path));
                }
            }
        }
    }
    None
}

pub async fn capture_dashboard(chrome: &std::path::Path, url: &str) -> Result<Vec<u8>> {
    let dir = tempfile::tempdir().context("temp dir for screenshot")?;
    let png_path = dir.path().join("dashboard.png");
    let user_data = dir.path().join("chrome-profile");
    std::fs::create_dir_all(&user_data)?;

    info!(%url, chrome = %chrome.display(), "capturing dashboard");
    let status = Command::new(chrome)
        .arg("--headless=new")
        .arg("--disable-gpu")
        .arg("--no-sandbox")
        .arg("--disable-dev-shm-usage")
        .arg("--hide-scrollbars")
        .arg("--force-device-scale-factor=1")
        .arg("--default-background-color=FFFFFFFF")
        .arg("--window-size=1600,1200")
        .arg(format!("--user-data-dir={}", user_data.display()))
        .arg(format!("--screenshot={}", png_path.display()))
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .status()
        .await
        .with_context(|| format!("running {}", chrome.display()))?;

    if !status.success() {
        bail!("chrome exited with {status}");
    }
    if !png_path.exists() {
        bail!("chrome did not write {}", png_path.display());
    }
    Ok(std::fs::read(png_path)?)
}
