//! Local Vector Semantic Memory Vault (`MemoryVault`).
//!
//! Computes deterministic 128-dimensional character/token trigram embeddings in pure Rust
//! and performs fast cosine similarity search without external native C dependencies.

use crate::guardrails::ZeroGuiltFilter;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

pub const EMBEDDING_DIM: usize = 128;

/// A single note stored inside Aether's semantic vector vault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub embedding: Vec<f32>,
}

/// Result item returned from semantic vector search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredMemory {
    pub entry: MemoryEntry,
    pub similarity: f32,
}

/// Local semantic vector store.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryVault {
    entries: Vec<MemoryEntry>,
}

impl MemoryVault {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Creates a pre-seeded sanctuary vault with helpful starter notes.
    pub fn with_sanctuary_seeds() -> Self {
        let mut vault = Self::new();
        vault.add(
            "Aether koyu buzlu cam (#070D14) tasarım sistemi ve 28px kart yarıçapı standardı.",
            vec!["design".into(), "glassmorphism".into()],
        );
        vault.add(
            "Gemini 3.8 Flash birincil multimodal model, Gemini 2.5 Flash ve yerel sıfır-suçluluk motoru yedek mimarisi.",
            vec!["ai".into(), "gemini".into(), "rust".into()],
        );
        vault.add(
            "Derin odaklanma için 432 Hz doğal rezonans, yaratıcı akış için 528 Hz ve 1760 Hz kristal su çınlaması.",
            vec!["audio".into(), "focus".into(), "sanctuary".into()],
        );
        vault
    }

    /// Adds a new note to the vault, sanitizing it with `ZeroGuiltFilter` and computing its vector embedding.
    pub fn add(&mut self, content: impl Into<String>, tags: Vec<String>) -> MemoryEntry {
        let raw = content.into();
        let clean = ZeroGuiltFilter::sanitize(&raw).sanitized;
        let combined_for_embed = format!("{} {}", clean, tags.join(" "));
        let embedding = compute_embedding(&combined_for_embed);

        let entry = MemoryEntry {
            id: Uuid::new_v4(),
            content: clean,
            tags,
            created_at: Utc::now(),
            embedding,
        };
        self.entries.push(entry.clone());
        entry
    }

    pub fn entries(&self) -> &[MemoryEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Performs semantic search using cosine similarity + lexical boost.
    pub fn search(&self, query: &str, top_k: usize) -> Vec<ScoredMemory> {
        if self.entries.is_empty() || top_k == 0 {
            return Vec::new();
        }
        let q_embed = compute_embedding(query);
        let q_lower = query.to_lowercase();

        let mut scored: Vec<ScoredMemory> = self
            .entries
            .iter()
            .map(|entry| {
                let mut sim = cosine_similarity(&q_embed, &entry.embedding);
                if entry.content.to_lowercase().contains(&q_lower)
                    || entry.tags.iter().any(|t| t.to_lowercase().contains(&q_lower))
                {
                    sim = (sim + 0.35).min(1.0);
                }
                ScoredMemory {
                    entry: entry.clone(),
                    similarity: sim,
                }
            })
            .collect();

        scored.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored.truncate(top_k);
        scored
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let vault: Self = serde_json::from_str(&data)?;
        Ok(vault)
    }
}

/// Computes a normalized `EMBEDDING_DIM`-dimensional feature vector using FNV-1a character 3-gram hashing.
pub fn compute_embedding(text: &str) -> Vec<f32> {
    let mut vec = vec![0.0_f32; EMBEDDING_DIM];
    let chars: Vec<char> = text.to_lowercase().chars().collect();
    if chars.is_empty() {
        return vec;
    }

    for window in chars.windows(3.min(chars.len())) {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &ch in window {
            hash ^= ch as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        let idx = (hash as usize) % EMBEDDING_DIM;
        let sign = if (hash >> 32) & 1 == 0 { 1.0 } else { -1.0 };
        vec[idx] += sign;
    }

    let norm: f32 = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 1e-6 {
        for v in &mut vec {
            *v /= norm;
        }
    }
    vec
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    dot.clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_memory_search_ranks_relevant_entry_first() {
        let mut vault = MemoryVault::with_sanctuary_seeds();
        assert_eq!(vault.len(), 3);

        vault.add(
            "Tauri v2 masaüstü kısayolu Cmd+K ile global komut merkezi açılır.",
            vec!["tauri".into(), "shortcut".into()],
        );

        let results = vault.search("Tauri kısayol Cmd+K", 2);
        assert_eq!(results.len(), 2);
        assert!(results[0].entry.content.contains("Tauri v2"));
        assert!(results[0].similarity > results[1].similarity);
    }
}
