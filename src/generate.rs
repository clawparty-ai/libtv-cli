use anyhow::{anyhow, Context, Result};
use std::time::Duration;
use tokio::time::sleep;

use crate::client::*;
use crate::models::*;

/// 发起文生图请求，返回任务 UUID
pub async fn text2img(
    client: &LiblibClient,
    template_uuid: &str,
    checkpoint_id: Option<&str>,
    prompt: &str,
    negative_prompt: Option<&str>,
    width: i32,
    height: i32,
    steps: i32,
    cfg_scale: f64,
    seed: i64,
    img_count: i32,
) -> Result<String> {
    let mut gp = GenerateParams {
        prompt: Some(prompt.to_string()),
        width: Some(width),
        height: Some(height),
        steps: Some(steps),
        cfgScale: Some(cfg_scale),
        seed: Some(seed),
        imgCount: Some(img_count),
        randnSource: Some(0),
        clipSkip: Some(2),
        sampler: Some(1),
        restoreFaces: Some(0),
        ..Default::default()
    };

    if let Some(np) = negative_prompt {
        gp.negativePrompt = Some(np.to_string());
    }
    if let Some(cid) = checkpoint_id {
        gp.checkPointId = Some(cid.to_string());
    }

    let req = Text2ImgRequest {
        template_uuid: template_uuid.to_string(),
        generate_params: gp,
    };

    let resp: GenerateResponseData = client
        .post_json("/api/generate/webui/text2img", &req)
        .await
        .context("text2img request")?;

    Ok(resp.generate_uuid)
}

/// 发起图生图请求
pub async fn img2img(
    client: &LiblibClient,
    template_uuid: &str,
    checkpoint_id: Option<&str>,
    prompt: &str,
    init_image: &str,
    denoising_strength: f64,
    negative_prompt: Option<&str>,
    width: i32,
    height: i32,
    steps: i32,
    cfg_scale: f64,
    seed: i64,
) -> Result<String> {
    let mut gp = GenerateParams {
        prompt: Some(prompt.to_string()),
        width: Some(width),
        height: Some(height),
        steps: Some(steps),
        cfgScale: Some(cfg_scale),
        seed: Some(seed),
        imgCount: Some(1),
        randnSource: Some(0),
        clipSkip: Some(2),
        sampler: Some(1),
        restoreFaces: Some(0),
        ..Default::default()
    };

    if let Some(np) = negative_prompt {
        gp.negativePrompt = Some(np.to_string());
    }
    if let Some(cid) = checkpoint_id {
        gp.checkPointId = Some(cid.to_string());
    }

    let req = Img2ImgRequest {
        template_uuid: template_uuid.to_string(),
        generate_params: gp,
        init_image: init_image.to_string(),
        denoising_strength: Some(denoising_strength),
    };

    let resp: GenerateResponseData = client
        .post_json("/api/generate/webui/img2img", &req)
        .await
        .context("img2img request")?;

    Ok(resp.generate_uuid)
}

/// 查询生成任务状态（liblib 状态接口实际为 POST）
pub async fn query_status(
    client: &LiblibClient,
    generate_uuid: &str,
) -> Result<StatusResponseData> {
    let body = serde_json::json!({"generateUuid": generate_uuid});
    client
        .post_json("/api/generate/webui/status", &body)
        .await
        .context("query status")
}

/// 轮询等待任务完成，返回图片 URL 列表
pub async fn poll_until_done(
    client: &LiblibClient,
    generate_uuid: &str,
    poll_interval_secs: u64,
    max_wait_secs: u64,
) -> Result<Vec<String>> {
    let start = std::time::Instant::now();
    let max_duration = Duration::from_secs(max_wait_secs);

    loop {
        if start.elapsed() > max_duration {
            return Err(anyhow!(
                "Timeout after {}s waiting for generate job {}",
                max_wait_secs,
                generate_uuid
            ));
        }

        let status = query_status(client, generate_uuid).await?;
        println!(
            "  [{}] status={}, progress={}%",
            generate_uuid,
            status.status_str(),
            status.progress_percent()
        );

        // liblib 的 generateStatus 定义：1=PENDING, 2=RUNNING, 3=SUCCESS, 4=FAILED, 5=?
        // 实际观察：当 generateStatus=5 且 images 非空时也表示成功
        if status.generate_status == 4 {
            return Err(anyhow!(
                "Generate job failed: {} (msg={:?})",
                generate_uuid,
                status.generate_msg
            ));
        }

        if !status.images.is_empty() && status.percent_completed >= 1.0 {
            let urls: Vec<String> = status
                .images
                .into_iter()
                .map(|img| img.image_url)
                .collect();
            return Ok(urls);
        }

        // 继续等待
        sleep(Duration::from_secs(poll_interval_secs)).await;
    }
}
