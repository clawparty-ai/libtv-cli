use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;

mod client;
mod download;
mod generate;
mod models;

use client::LiblibClient;

/// libtv-cli — LiblibAI (liblib.art) 文生图命令行工具
#[derive(Parser, Debug)]
#[command(name = "libtv-cli")]
#[command(about = "CLI for LiblibAI text-to-image generation")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// AccessKey (或设置环境变量 LIBLIB_ACCESS_KEY)
    #[arg(long, global = true, env = "LIBLIB_ACCESS_KEY")]
    access_key: Option<String>,

    /// SecretKey (或设置环境变量 LIBLIB_SECRET_KEY)
    #[arg(long, global = true, env = "LIBLIB_SECRET_KEY")]
    secret_key: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 文生图 (text-to-image)
    #[command(name = "text2img")]
    Text2Img(Text2ImgArgs),

    /// 图生图 (image-to-image)
    #[command(name = "img2img")]
    Img2Img(Img2ImgArgs),

    /// 查询生成任务状态
    #[command(name = "status")]
    Status(StatusArgs),

    /// 通过一个完整命令快速生图（兼容简单用法）
    #[command(name = "generate")]
    Generate(Text2ImgArgs),
}

#[derive(Parser, Debug)]
struct Text2ImgArgs {
    /// 正向提示词
    #[arg(short, long)]
    prompt: String,

    /// 模板 UUID
    #[arg(short = 't', long = "template-uuid")]
    template_uuid: String,

    /// 模型/Checkpoint ID
    #[arg(short = 'c', long = "checkpoint-id")]
    checkpoint_id: Option<String>,

    /// 负向提示词
    #[arg(short = 'n', long = "negative-prompt")]
    negative_prompt: Option<String>,

    /// 图片宽度
    #[arg(short = 'W', long, default_value = "512")]
    width: i32,

    /// 图片高度
    #[arg(short = 'H', long, default_value = "512")]
    height: i32,

    /// 采样步数
    #[arg(short = 's', long, default_value = "20")]
    steps: i32,

    /// CFG Scale
    #[arg(long, default_value = "7.0")]
    cfg_scale: f64,

    /// 随机种子 (-1 表示随机)
    #[arg(long, default_value = "-1")]
    seed: i64,

    /// 生成数量
    #[arg(short = 'N', long, default_value = "1")]
    img_count: i32,

    /// 输出文件路径
    #[arg(short = 'o', long, default_value = "generated.png")]
    output: PathBuf,

    /// 轮询间隔（秒）
    #[arg(long, default_value = "3")]
    poll_interval: u64,

    /// 最大等待时间（秒）
    #[arg(long, default_value = "300")]
    max_wait: u64,
}

#[derive(Parser, Debug)]
struct Img2ImgArgs {
    /// 正向提示词
    #[arg(short, long)]
    prompt: String,

    /// 模板 UUID
    #[arg(short = 't', long = "template-uuid")]
    template_uuid: String,

    /// 输入图片路径或 URL
    #[arg(short = 'i', long = "input")]
    input_image: String,

    /// 去噪强度 (0.0 ~ 1.0)
    #[arg(short = 'd', long = "denoising", default_value = "0.75")]
    denoising_strength: f64,

    /// 模型/Checkpoint ID
    #[arg(short = 'c', long = "checkpoint-id")]
    checkpoint_id: Option<String>,

    /// 负向提示词
    #[arg(short = 'n', long = "negative-prompt")]
    negative_prompt: Option<String>,

    #[arg(short = 'W', long, default_value = "512")]
    width: i32,

    #[arg(short = 'H', long, default_value = "512")]
    height: i32,

    #[arg(short = 's', long, default_value = "20")]
    steps: i32,

    #[arg(long, default_value = "7.0")]
    cfg_scale: f64,

    #[arg(long, default_value = "-1")]
    seed: i64,

    #[arg(short = 'o', long, default_value = "generated.png")]
    output: PathBuf,

    #[arg(long, default_value = "3")]
    poll_interval: u64,

    #[arg(long, default_value = "300")]
    max_wait: u64,
}

#[derive(Parser, Debug)]
struct StatusArgs {
    /// 生成任务 UUID
    #[arg(short, long)]
    uuid: String,
}

// ─────────────────────────────────────────────

fn get_keys(cli: &Cli) -> Result<(String, String)> {
    let ak = cli
        .access_key
        .clone()
        .or_else(|| env::var("LIBLIB_ACCESS_KEY").ok())
        .ok_or_else(|| anyhow!("Missing AccessKey. Use --access-key or set LIBLIB_ACCESS_KEY env."))?;
    let sk = cli
        .secret_key
        .clone()
        .or_else(|| env::var("LIBLIB_SECRET_KEY").ok())
        .ok_or_else(|| anyhow!("Missing SecretKey. Use --secret-key or set LIBLIB_SECRET_KEY env."))?;
    Ok((ak, sk))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Text2Img(args) | Commands::Generate(args) => cmd_text2img(&cli, args).await,
        Commands::Img2Img(args) => cmd_img2img(&cli, args).await,
        Commands::Status(args) => cmd_status(&cli, args).await,
    }
}

async fn cmd_text2img(cli: &Cli, args: &Text2ImgArgs) -> Result<()> {
    let (ak, sk) = get_keys(cli)?;
    let client = LiblibClient::new(ak, sk)?;

    println!("🎨 发起文生图任务...");
    println!("   Prompt: {}", &args.prompt);
    println!("   Template: {}", &args.template_uuid);
    println!("   Size: {}x{}", args.width, args.height);

    let uuid = generate::text2img(
        &client,
        &args.template_uuid,
        args.checkpoint_id.as_deref(),
        &args.prompt,
        args.negative_prompt.as_deref(),
        args.width,
        args.height,
        args.steps,
        args.cfg_scale,
        args.seed,
        args.img_count,
    )
    .await?;

    println!("✅ 任务已提交，generateUuid={}", uuid);
    println!("⏳ 等待生成完成...");

    let urls = generate::poll_until_done(
        &client,
        &uuid,
        args.poll_interval,
        args.max_wait,
    )
    .await?;

    println!("📥 下载 {} 张图片...", urls.len());
    let paths = download::download_images(&urls, &args.output).await?;

    for p in &paths {
        println!("💾 已保存: {}", p.canonicalize().unwrap_or_else(|_| p.clone()).display());
    }

    Ok(())
}

async fn cmd_img2img(cli: &Cli, args: &Img2ImgArgs) -> Result<()> {
    let (ak, sk) = get_keys(cli)?;
    let client = LiblibClient::new(ak, sk)?;

    println!("🎨 发起图生图任务...");
    println!("   Prompt: {}", &args.prompt);
    println!("   Input: {}", &args.input_image);

    let uuid = generate::img2img(
        &client,
        &args.template_uuid,
        args.checkpoint_id.as_deref(),
        &args.prompt,
        &args.input_image,
        args.denoising_strength,
        args.negative_prompt.as_deref(),
        args.width,
        args.height,
        args.steps,
        args.cfg_scale,
        args.seed,
    )
    .await?;

    println!("✅ 任务已提交，generateUuid={}", uuid);
    println!("⏳ 等待生成完成...");

    let urls = generate::poll_until_done(
        &client,
        &uuid,
        args.poll_interval,
        args.max_wait,
    )
    .await?;

    println!("📥 下载 {} 张图片...", urls.len());
    let paths = download::download_images(&urls, &args.output).await?;

    for p in &paths {
        println!("💾 已保存: {}", p.canonicalize().unwrap_or_else(|_| p.clone()).display());
    }

    Ok(())
}

async fn cmd_status(cli: &Cli, args: &StatusArgs) -> Result<()> {
    let (ak, sk) = get_keys(cli)?;
    let client = LiblibClient::new(ak, sk)?;

    let status = generate::query_status(&client, &args.uuid).await?;
    println!("GenerateUuid:   {}", status.generate_uuid);
    println!("Status:         {}", status.status_str());
    println!("Progress:       {}%", status.progress_percent());
    if let Some(msg) = &status.generate_msg {
        println!("Msg:            {}", msg);
    }
    if let Some(cost) = status.points_cost {
        println!("PointsCost:     {}", cost);
    }
    if let Some(balance) = status.account_balance {
        println!("AccountBalance: {}", balance);
    }
    if !status.images.is_empty() {
        println!("Images:");
        for (i, img) in status.images.iter().enumerate() {
            println!("  [{}] {} ({}x{})", i + 1, img.image_url, img.width, img.height);
        }
    }
    Ok(())
}
