use anyhow::{Context, Result};
use reqwest::Client;
use std::path::{Path, PathBuf};

/// 下载图片到本地文件
pub async fn download_image(url: &str, output_path: &Path) -> Result<PathBuf> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .context("build download client")?;

    let resp = client
        .get(url)
        .send()
        .await
        .context(format!("download image from {}", url))?;

    if !resp.status().is_success() {
        return Err(anyhow::anyhow!(
            "Download failed HTTP {}: {}",
            resp.status(),
            url
        ));
    }

    let bytes = resp
        .bytes()
        .await
        .context("read image bytes")?;

    // 确保输出目录存在
    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .context("create output directory")?;
    }

    tokio::fs::write(output_path, &bytes)
        .await
        .context(format!("write image to {:?}", output_path))?;

    Ok(output_path.to_path_buf())
}

/// 批量下载，自动在文件名后加序号
pub async fn download_images(urls: &[String], output_base: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::with_capacity(urls.len());
    let ext = output_base
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    let stem = output_base
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("generated");
    let parent = output_base.parent();

    for (i, url) in urls.iter().enumerate() {
        let filename = if urls.len() == 1 {
            format!("{}.{}", stem, ext)
        } else {
            format!("{}_{:03}.{}", stem, i + 1, ext)
        };
        let path = parent.map(|p| p.join(&filename)).unwrap_or_else(|| PathBuf::from(&filename));
        let downloaded = download_image(url, &path).await?;
        paths.push(downloaded);
    }

    Ok(paths)
}
