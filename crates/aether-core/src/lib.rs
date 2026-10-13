//! # `aether-core`
//!
//! Core library for **Aether (AuraMind)** — Rust & Tauri v2 Dark Frosted Glassmorphic AI Command Center,
//! Model Context Protocol (MCP) engine, Semantic Vector Memory Vault, Harmonic Audio Sanctuary,
//! and World-Class Landing Page & Competitor CRO Benchmark Engine.

pub mod audio;
pub mod cro;
pub mod design_tokens;
pub mod focus;
pub mod gemini;
pub mod guardrails;
pub mod memory;

pub use audio::{HarmonicPreset, HarmonicSynthesizer};
pub use cro::{
    CroActionItem, CroBenchmarkEngine, CroComparisonReport, CroDimensionScores, SiteCandidate,
    SiteEvaluation,
};
pub use design_tokens::AetherDesignTokens;
pub use focus::{FocusSanctuary, SanctuaryState};
pub use gemini::{AetherAiResponse, AetherPromptRequest, GeminiClient, ModelTier};
pub use guardrails::{SanitizationReport, ZeroGuiltFilter};
pub use memory::{MemoryEntry, MemoryVault, ScoredMemory};
