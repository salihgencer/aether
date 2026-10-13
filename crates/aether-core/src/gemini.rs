//! Google Gemini 3.8 Flash Multimodal & Structured Output Client with Smart Fallback.
//!
//! Architecture:
//! 1. Primary Model: `gemini-3.8-flash`
//! 2. Smart Cloud Fallback: `gemini-2.5-flash`
//! 3. Zero-Guilt Local Rule-Based Engine: Instant, compassionate offline fallback.

use crate::guardrails::ZeroGuiltFilter;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use thiserror::Error;

pub const PRIMARY_GEMINI_MODEL: &str = "gemini-3.8-flash";
pub const FALLBACK_GEMINI_MODEL: &str = "gemini-2.5-flash";

#[derive(Debug, Error)]
pub enum GeminiError {
    #[error("Network transport error: {0}")]
    Transport(String),
    #[error("API response status {0}: {1}")]
    ApiStatus(u16, String),
}

/// Indicates which tier answered the user's prompt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelTier {
    /// Primary flagship model (`gemini-3.8-flash`).
    PrimaryGemini38Flash,
    /// Secondary cloud fallback (`gemini-2.5-flash`).
    FallbackGemini25Flash,
    /// Local Zero-Guilt rule-based sanctuary engine.
    LocalZeroGuiltEngine,
}

impl ModelTier {
    pub fn model_id(&self) -> &'static str {
        match self {
            Self::PrimaryGemini38Flash => PRIMARY_GEMINI_MODEL,
            Self::FallbackGemini25Flash => FALLBACK_GEMINI_MODEL,
            Self::LocalZeroGuiltEngine => "aether-local-zero-guilt-v1",
        }
    }
}

/// Request payload for Aether's AI assistant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AetherPromptRequest {
    pub prompt: String,
    pub context_notes: Vec<String>,
    pub harmonic_hz: Option<f32>,
}

/// Structured, Zero-Guilt compliant AI response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AetherAiResponse {
    pub reply: String,
    pub action_suggestion: String,
    pub recommended_frequency_hz: f32,
    pub tier: ModelTier,
    pub model_used: String,
    pub latency_ms: u64,
    pub zero_guilt_verified: bool,
}

/// Client managing `gemini-3.8-flash` -> `gemini-2.5-flash` -> `LocalZeroGuiltEngine`.
#[derive(Debug, Clone)]
pub struct GeminiClient {
    api_key: Option<String>,
    http: reqwest::Client,
}

impl Default for GeminiClient {
    fn default() -> Self {
        Self::new(std::env::var("GEMINI_API_KEY").ok())
    }
}

impl GeminiClient {
    pub fn new(api_key: Option<String>) -> Self {
        let cleaned_key = api_key.and_then(|k| {
            let trimmed = k.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });
        Self {
            api_key: cleaned_key,
            http: reqwest::Client::new(),
        }
    }

    pub fn has_api_key(&self) -> bool {
        self.api_key.is_some()
    }

    /// Generates a Zero-Guilt response, trying `gemini-3.8-flash`, then `gemini-2.5-flash`,
    /// and seamlessly falling back to the local Zero-Guilt sanctuary engine.
    pub async fn generate(&self, req: &AetherPromptRequest) -> AetherAiResponse {
        let start = Instant::now();

        if let Some(ref key) = self.api_key {
            if let Ok(raw_text) = self.call_gemini_model(PRIMARY_GEMINI_MODEL, key, req).await {
                return Self::finalize_response(
                    raw_text,
                    ModelTier::PrimaryGemini38Flash,
                    req.harmonic_hz.unwrap_or(432.0),
                    start.elapsed().as_millis() as u64,
                );
            }

            if let Ok(raw_text) = self.call_gemini_model(FALLBACK_GEMINI_MODEL, key, req).await {
                return Self::finalize_response(
                    raw_text,
                    ModelTier::FallbackGemini25Flash,
                    req.harmonic_hz.unwrap_or(432.0),
                    start.elapsed().as_millis() as u64,
                );
            }
        }

        let (local_reply, freq) = Self::local_zero_guilt_synthesis(req);
        Self::finalize_response(
            local_reply,
            ModelTier::LocalZeroGuiltEngine,
            freq,
            start.elapsed().as_millis() as u64,
        )
    }

    async fn call_gemini_model(
        &self,
        model: &str,
        api_key: &str,
        req: &AetherPromptRequest,
    ) -> Result<String, GeminiError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );

        let system_instruction = "Sen Aether (AuraMind) dingin masaüstü bilgesisin. \
            Sıfır Suçluluk (Zero-Guilt) felsefesiyle konuşursun. \
            Asla 'ihmal, unuttun, ceza, hata, geç kaldın, eksik, başarısız' kelimelerini kullanma. \
            Kısa, berrak, şefkatli ve eyleme dönüştürülebilir yanıtlar ver.";

        let context_block = if req.context_notes.is_empty() {
            String::new()
        } else {
            format!("\n\nİlgili Hafıza Notları:\n- {}", req.context_notes.join("\n- "))
        };

        let body = serde_json::json!({
            "system_instruction": {
                "parts": [{ "text": system_instruction }]
            },
            "contents": [{
                "role": "user",
                "parts": [{ "text": format!("{}{}", req.prompt, context_block) }]
            }],
            "generationConfig": {
                "temperature": 0.6,
                "maxOutputTokens": 512
            }
        });

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| GeminiError::Transport(e.to_string()))?;

        let status = resp.status();
        if !status.is_success() {
            let err_body = resp.text().await.unwrap_or_default();
            return Err(GeminiError::ApiStatus(status.as_u16(), err_body));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| GeminiError::Transport(e.to_string()))?;

        let extracted = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("Zihnine berraklık, akışına dinginlik.")
            .to_string();

        Ok(extracted)
    }

    fn local_zero_guilt_synthesis(req: &AetherPromptRequest) -> (String, f32) {
        let lower = req.prompt.to_lowercase();
        let freq = req.harmonic_hz.unwrap_or_else(|| {
            if lower.contains("yaratıcı") || lower.contains("tasarım") || lower.contains("fikir") {
                528.0
            } else if lower.contains("dinlen") || lower.contains("mola") || lower.contains("uyku") {
                174.0
            } else if lower.contains("nefes") || lower.contains("denge") {
                320.0
            } else {
                432.0
            }
        });

        let memory_hint = if let Some(first) = req.context_notes.first() {
            format!(" Hafıza kasandaki \"{}\" notuyla uyumlu olarak;", first)
        } else {
            String::new()
        };

        let reply = if lower.contains("odak") || lower.contains("çalış") || lower.contains("kod") {
            format!(
                "Zihnine berraklık, akışına dinginlik.{memory_hint} \
                 Şimdi {:.0} Hz rezonans eşliğinde tek bir küçük ve anlamlı adıma odaklanabilirsin. \
                 Her dakikan değerli ve kendi ritminde ilerliyor.",
                freq
            )
        } else if lower.contains("yorgun") || lower.contains("bunal") || lower.contains("stres") {
            format!(
                "Şu an burada olman bile kendine gösterdiğin özenin bir parçası.{memory_hint} \
                 Odağını Dinlenme Moduna (Resting Sanctuary) alabilir, {:.0} Hz dingin frekansla \
                 bir yudum can suyu içerek zihnini tazeleyebilirsin.",
                freq
            )
        } else {
            format!(
                "Aether yanındaki dingin alanın.{memory_hint} \"{}\" düşünceni berraklaştırmak için \
                 {:.0} Hz harmonik akışta küçük bir başlangıç adımı seçelim.",
                req.prompt.trim(),
                freq
            )
        };

        (reply, freq)
    }

    fn finalize_response(
        raw_reply: String,
        tier: ModelTier,
        recommended_frequency_hz: f32,
        latency_ms: u64,
    ) -> AetherAiResponse {
        let sanitized_report = ZeroGuiltFilter::sanitize(&raw_reply);
        let action_suggestion = if recommended_frequency_hz >= 528.0 {
            "528 Hz Canlanma Kristalini başlat ve ilk fikrini Hafıza Kasasına kaydet.".to_string()
        } else if recommended_frequency_hz <= 174.0 {
            "174 Hz Temel Huzur moduna geçip 3 derin nefesle dinlenme alanına adım at.".to_string()
        } else {
            "432 Hz Berraklık akışını başlat ve 15 dakikalık dingin odak halkasını aç.".to_string()
        };

        AetherAiResponse {
            model_used: tier.model_id().to_string(),
            tier,
            reply: sanitized_report.sanitized,
            action_suggestion,
            recommended_frequency_hz,
            latency_ms,
            zero_guilt_verified: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_fallback_when_no_api_key() {
        let client = GeminiClient::new(None);
        assert!(!client.has_api_key());

        let req = AetherPromptRequest {
            prompt: "Bugün Rust mimarisi üzerinde odaklanmak istiyorum ama biraz geç kaldın hissi var".to_string(),
            context_notes: vec!["Cargo workspace ve Tauri v2 notları".to_string()],
            harmonic_hz: None,
        };

        let resp = client.generate(&req).await;
        assert_eq!(resp.tier, ModelTier::LocalZeroGuiltEngine);
        assert_eq!(resp.model_used, "aether-local-zero-guilt-v1");
        assert!(resp.zero_guilt_verified);
        assert!(ZeroGuiltFilter::is_compliant(&resp.reply));
    }
}
