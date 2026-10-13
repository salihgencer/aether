//! `aether-desktop` — Tauri v2 / Axum Dark Frosted Glassmorphic Desktop App & IPC Server.

use aether_core::{
    AetherDesignTokens, AetherPromptRequest, FocusSanctuary, GeminiClient, HarmonicPreset,
    HarmonicSynthesizer, MemoryEntry, MemoryVault, ScoredMemory,
};
use axum::{
    Json, Router,
    extract::{Query, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use clap::Parser;
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

const INDEX_HTML: &str = include_str!("../ui/index.html");

#[derive(Parser, Debug)]
#[command(name = "aether-desktop", about = "Aether Dark Frosted Glassmorphic Desktop Sanctuary")]
struct DesktopArgs {
    /// Port for the local desktop bridge
    #[arg(long, default_value_t = 4320)]
    port: u16,
    /// Print configuration and exit immediately (useful for CI/health checks)
    #[arg(long)]
    check: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub gemini: GeminiClient,
    pub vault: Arc<RwLock<MemoryVault>>,
    pub sanctuary: Arc<RwLock<FocusSanctuary>>,
    pub tokens: AetherDesignTokens,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            gemini: GeminiClient::default(),
            vault: Arc::new(RwLock::new(MemoryVault::with_sanctuary_seeds())),
            sanctuary: Arc::new(RwLock::new(FocusSanctuary::default())),
            tokens: AetherDesignTokens::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct AskPayload {
    pub prompt: String,
    pub harmonic_hz: Option<f32>,
}

#[derive(Deserialize)]
pub struct MemorySearchPayload {
    pub query: String,
    pub top_k: Option<usize>,
}

#[derive(Deserialize)]
pub struct MemoryAddPayload {
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct FlowPayload {
    pub minutes: u32,
    pub hz: Option<f32>,
}

#[derive(Deserialize)]
pub struct HydratePayload {
    pub ml: u32,
}

#[derive(Deserialize)]
pub struct WavQuery {
    pub hz: Option<f32>,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/api/tokens", get(tokens_handler))
        .route("/api/ask", post(ask_handler))
        .route("/api/memory", get(memory_list_handler))
        .route("/api/memory/search", post(memory_search_handler))
        .route("/api/memory/add", post(memory_add_handler))
        .route("/api/focus/flow", post(focus_flow_handler))
        .route("/api/focus/rest", post(focus_rest_handler))
        .route("/api/focus/hydrate", post(focus_hydrate_handler))
        .route("/api/audio/wav", get(audio_wav_handler))
        .with_state(state)
}

async fn index_handler() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn tokens_handler(State(state): State<AppState>) -> Json<AetherDesignTokens> {
    Json(state.tokens)
}

async fn ask_handler(
    State(state): State<AppState>,
    Json(payload): Json<AskPayload>,
) -> Json<aether_core::AetherAiResponse> {
    let context_notes = {
        let vault = state.vault.read().await;
        vault
            .search(&payload.prompt, 2)
            .into_iter()
            .map(|m| m.entry.content)
            .collect()
    };

    let resp = state
        .gemini
        .generate(&AetherPromptRequest {
            prompt: payload.prompt,
            context_notes,
            harmonic_hz: payload.harmonic_hz,
        })
        .await;

    Json(resp)
}

async fn memory_list_handler(State(state): State<AppState>) -> Json<Vec<MemoryEntry>> {
    let vault = state.vault.read().await;
    Json(vault.entries().to_vec())
}

async fn memory_search_handler(
    State(state): State<AppState>,
    Json(payload): Json<MemorySearchPayload>,
) -> Json<Vec<ScoredMemory>> {
    let vault = state.vault.read().await;
    Json(vault.search(&payload.query, payload.top_k.unwrap_or(5)))
}

async fn memory_add_handler(
    State(state): State<AppState>,
    Json(payload): Json<MemoryAddPayload>,
) -> Json<MemoryEntry> {
    let mut vault = state.vault.write().await;
    let entry = vault.add(payload.content, payload.tags.unwrap_or_default());
    Json(entry)
}

async fn focus_flow_handler(
    State(state): State<AppState>,
    Json(payload): Json<FlowPayload>,
) -> Json<FocusSanctuary> {
    let mut sanctuary = state.sanctuary.write().await;
    sanctuary.start_flow(
        "Zihnine Berraklık, Akışına Dinginlik",
        25,
        payload.hz.unwrap_or(432.0),
    );
    sanctuary.record_minutes(payload.minutes);
    Json(sanctuary.clone())
}

async fn focus_rest_handler(State(state): State<AppState>) -> Json<FocusSanctuary> {
    let mut sanctuary = state.sanctuary.write().await;
    sanctuary.enter_resting_mode();
    Json(sanctuary.clone())
}

async fn focus_hydrate_handler(
    State(state): State<AppState>,
    Json(payload): Json<HydratePayload>,
) -> Json<FocusSanctuary> {
    let mut sanctuary = state.sanctuary.write().await;
    sanctuary.add_hydration_ml(payload.ml);
    Json(sanctuary.clone())
}

async fn audio_wav_handler(Query(q): Query<WavQuery>) -> impl IntoResponse {
    let preset = HarmonicPreset::from_hz(q.hz.unwrap_or(1760.0));
    let wav = HarmonicSynthesizer::synthesize_wav(preset, 1.2, 44100);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "audio/wav")],
        wav,
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = DesktopArgs::parse();
    let state = AppState::default();

    if args.check {
        println!(
            "✅ Aether Desktop hazır • Obsidian: {} • Primary AI: {}",
            state.tokens.colors.obsidian_dark, state.tokens.ai_models.primary_model
        );
        return Ok(());
    }

    let app = build_router(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("🌌 Aether (AuraMind) Masaüstü Komut Merkezi çalışıyor: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_desktop_state_and_ui_bundle_validity() {
        let state = AppState::default();
        assert!(state.tokens.validate());
        assert!(INDEX_HTML.contains("#070D14"));
        assert!(INDEX_HTML.contains("gemini-3.8-flash"));
        assert!(INDEX_HTML.contains("backdrop-filter: blur("));
    }
}
