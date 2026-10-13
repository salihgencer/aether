//! `aether` CLI & MCP Server binary.

pub mod mcp;

use aether_core::{
    AetherDesignTokens, AetherPromptRequest, FocusSanctuary, GeminiClient, HarmonicPreset,
    HarmonicSynthesizer, MemoryVault, ZeroGuiltFilter,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "aether",
    version,
    about = "🌌 Aether (AuraMind) — Gemini 3.8 Flash AI Command Center, Vector Memory & Harmonic Sanctuary CLI/MCP"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Ask Aether's Gemini 3.8 Flash assistant (with Gemini 2.5 Flash & Zero-Guilt fallback)
    Ask {
        /// Prompt or question for Aether
        prompt: String,
        /// Optional harmonic resonance frequency (432, 528, 320, 174)
        #[arg(long, default_value_t = 432.0)]
        hz: f32,
    },
    /// Semantic Vector Memory Vault commands
    Memory {
        #[command(subcommand)]
        action: MemoryAction,
    },
    /// Synthesize harmonic ambient audio or 1760/2637 Hz Crystal Water Chime WAV file
    Audio {
        /// Preset frequency in Hz (432, 528, 320, 174, or 1760 for Crystal Chime)
        #[arg(long, default_value_t = 1760.0)]
        hz: f32,
        /// Duration in seconds
        #[arg(long, default_value_t = 1.5)]
        duration: f32,
        /// Output WAV file path
        #[arg(long, default_value = "aether_chime.wav")]
        output: PathBuf,
    },
    /// Run system & design token verification check
    Doctor,
    /// Launch Model Context Protocol (MCP) JSON-RPC 2.0 Server over stdio
    Mcp,
}

#[derive(Subcommand, Debug)]
enum MemoryAction {
    /// Search semantic vector memory vault
    Search {
        query: String,
        #[arg(long, default_value_t = 3)]
        top_k: usize,
    },
    /// Add a note to the semantic vector memory vault
    Add {
        content: String,
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ask { prompt, hz } => {
            let vault = MemoryVault::with_sanctuary_seeds();
            let hits = vault.search(&prompt, 2);
            let context_notes = hits.into_iter().map(|h| h.entry.content).collect();

            let client = GeminiClient::default();
            let resp = client
                .generate(&AetherPromptRequest {
                    prompt,
                    context_notes,
                    harmonic_hz: Some(hz),
                })
                .await;

            println!("╭──────────────────────────────────────────────────────────────╮");
            println!("│ 🌌 Aether AI • Model: {:<38} │", resp.model_used);
            println!("╰──────────────────────────────────────────────────────────────╯");
            println!("\n{}\n", resp.reply);
            println!("✨ Önerilen Adım: {}", resp.action_suggestion);
            println!(
                "🎵 Harmonik Frekans: {:.0} Hz | 🛡️ Sıfır-Suçluluk Doğrulandı: {}",
                resp.recommended_frequency_hz, resp.zero_guilt_verified
            );
        }
        Commands::Memory { action } => {
            let mut vault = MemoryVault::with_sanctuary_seeds();
            match action {
                MemoryAction::Search { query, top_k } => {
                    let results = vault.search(&query, top_k);
                    println!("🔍 \"{}\" için Vektör Hafıza Sonuçları:", query);
                    for (idx, item) in results.iter().enumerate() {
                        println!(
                            "  {}. [Benzerlik: {:.2}] {} (Etiketler: {:?})",
                            idx + 1,
                            item.similarity,
                            item.entry.content,
                            item.entry.tags
                        );
                    }
                }
                MemoryAction::Add { content, tags } => {
                    let entry = vault.add(content, tags);
                    println!("✅ Hafıza Kasasına Eklendi [{}]: {}", entry.id, entry.content);
                }
            }
        }
        Commands::Audio {
            hz,
            duration,
            output,
        } => {
            let preset = HarmonicPreset::from_hz(hz);
            let wav_bytes = HarmonicSynthesizer::synthesize_wav(preset, duration, 44100);
            std::fs::write(&output, &wav_bytes)?;
            println!(
                "🎵 {} ({} bayt PCM WAV) başarıyla oluşturuldu: {}",
                preset.label_tr(),
                wav_bytes.len(),
                output.display()
            );
        }
        Commands::Doctor => {
            let tokens = AetherDesignTokens::default();
            let sanctuary = FocusSanctuary::default();
            let client = GeminiClient::default();
            println!("🌌 Aether (AuraMind) Sistem & Mimari Doğrulaması");
            println!("  • Tasarım Sistemi (DESIGN.md): {} (Obsidian {})", tokens.validate(), tokens.colors.obsidian_dark);
            println!("  • Birincil AI Modeli         : {}", tokens.ai_models.primary_model);
            println!("  • Akıllı Yedek AI Modeli     : {}", tokens.ai_models.fallback_model);
            println!("  • Gemini Bulut Anahtarı      : {}", if client.has_api_key() { "Aktif" } else { "Yerel Sıfır-Suçluluk Motoru Aktif (Offline-Ready)" });
            println!("  • Sıfır-Suçluluk Filtresi    : Aktif ({} yasaklı terim koruması)", aether_core::guardrails::FORBIDDEN_REPLACEMENTS.len());
            println!("  • Odak Mabedi Durumu         : {:?} ({})", sanctuary.state, ZeroGuiltFilter::is_compliant(&sanctuary.compassionate_message));
        }
        Commands::Mcp => {
            let mut server = mcp::McpServer::default();
            server.run_stdio().await?;
        }
    }

    Ok(())
}
