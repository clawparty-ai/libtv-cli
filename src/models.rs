use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────
// API 通用响应包装
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i64,
    #[serde(default)]
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn is_success(&self) -> bool {
        self.code == 0
    }
}

// ─────────────────────────────────────────────
// 文生图 / 图生图 请求参数
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Default)]
pub struct GenerateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkPointId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negativePrompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clipSkip: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfgScale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgCount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randnSource: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restoreFaces: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalNetwork")]
    pub additional_network: Option<Vec<AdditionalNetwork>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controlNet")]
    pub control_net: Option<Vec<ControlNetParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vaeId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hiResFixInfo")]
    pub hires_fix_info: Option<HiResFixInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdditionalNetwork {
    #[serde(rename = "modelId")]
    pub model_id: String,
    #[serde(rename = "modelName")]
    pub model_name: Option<String>,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ControlNetParams {
    #[serde(rename = "controlNetModelId")]
    pub control_net_model_id: String,
    pub weight: Option<f64>,
    #[serde(rename = "inputImage")]
    pub input_image: Option<String>,
    #[serde(rename = "module")]
    pub module: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HiResFixInfo {
    pub enabled: i32,
    #[serde(rename = "upscaler")]
    pub upscaler: Option<String>,
    #[serde(rename = "denoisingStrength")]
    pub denoising_strength: Option<f64>,
    pub steps: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Text2ImgRequest {
    #[serde(rename = "templateUuid")]
    pub template_uuid: String,
    #[serde(rename = "generateParams")]
    pub generate_params: GenerateParams,
}

#[derive(Debug, Clone, Serialize)]
pub struct Img2ImgRequest {
    #[serde(rename = "templateUuid")]
    pub template_uuid: String,
    #[serde(rename = "generateParams")]
    pub generate_params: GenerateParams,
    #[serde(rename = "initImage")]
    pub init_image: String,
    #[serde(rename = "denoisingStrength")]
    pub denoising_strength: Option<f64>,
}

// ─────────────────────────────────────────────
// 生图响应
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct GenerateResponseData {
    #[serde(rename = "generateUuid")]
    pub generate_uuid: String,
}

// ─────────────────────────────────────────────
// 状态查询 响应
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct StatusResponseData {
    #[serde(rename = "generateUuid")]
    pub generate_uuid: String,

    /// generateStatus: 1=PENDING, 2=RUNNING, 3=SUCCESS, 4=FAILED, 0=UNKNOWN
    #[serde(rename = "generateStatus", default)]
    pub generate_status: i32,

    #[serde(default)]
    pub images: Vec<ImageInfo>,

    #[serde(default)]
    pub videos: Vec<serde_json::Value>,

    #[serde(default)]
    pub audios: Vec<serde_json::Value>,

    /// 进度 0.0 ~ 1.0
    #[serde(rename = "percentCompleted", default)]
    pub percent_completed: f64,

    #[serde(rename = "generateMsg", default)]
    pub generate_msg: Option<String>,

    #[serde(rename = "pointsCost", default)]
    pub points_cost: Option<i32>,

    #[serde(rename = "accountBalance", default)]
    pub account_balance: Option<i32>,
}

impl StatusResponseData {
    /// 将 generateStatus 转换为可读状态字符串
    pub fn status_str(&self) -> &'static str {
        match self.generate_status {
            1 => "PENDING",
            2 => "RUNNING",
            3 => "SUCCESS",
            4 => "FAILED",
            _ => "UNKNOWN",
        }
    }

    /// 将 percentCompleted 转为 0-100 整数
    pub fn progress_percent(&self) -> i32 {
        (self.percent_completed * 100.0).round() as i32
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageInfo {
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "imageUuid", default)]
    pub image_uuid: Option<String>,
    #[serde(default)]
    pub width: i32,
    #[serde(default)]
    pub height: i32,
}
