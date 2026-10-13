# 🌌 Aether (AuraMind)

> *"Zihnine Berraklık, Akışına Dinginlik."*  
> **Rust 2024 & Tauri v2 Dark Frosted Glassmorphic AI Command Center, MCP Server & Harmonic Focus Sanctuary**

Aether (`AuraMind`), **Gemini 3.8 Flash** (`gemini-3.8-flash`) birincil modeli, **Gemini 2.5 Flash** (`gemini-2.5-flash`) bulut yedeği ve çevrimdışı **Sıfır-Suçluluk (Zero-Guilt)** yerel motoruyla çalışan çoklu crate'li (Cargo Workspace) bir Rust ekosistemidir.

---

## 🏗️ Cargo Workspace Mimarisi

```text
aether/
├── DESIGN.md                                  # Google DESIGN.md Açık Standardı & YAML Tokenları
├── .agents/skills/aether-design-system/       # TypeUI uyumlu Tasarım Becerisi (SKILL.md)
└── crates/
    ├── aether-core/                           # Çekirdek Kütüphane
    │   ├── design_tokens.rs                   # #070D14 Obsidian, 28px Radius, 20px Blur
    │   ├── guardrails.rs                      # Sıfır-Suçluluk (Zero-Guilt) Dil Filtresi
    │   ├── gemini.rs                          # Gemini 3.8 Flash + 2.5 Flash + Yerel Motor
    │   ├── memory.rs                          # 128-Boyutlu Vektör Hafıza Kasası (Cosine Similarity)
    │   ├── audio.rs                           # 432/528/320/174 Hz & 1760/2637 Hz PCM WAV Sentezleyici
    │   └── focus.rs                           # Sıfır-Suçluluk Odak & Can Suyu Durum Makinesi
    ├── aether-cli/                            # Hızlı CLI & Model Context Protocol (MCP) Sunucusu
    └── aether-desktop/                        # Tauri v2 & Axum Koyu Buzlu Cam Masaüstü Uygulaması
```

---

## 🚀 Hızlı Başlangıç

### 1. Testleri Çalıştır
```bash
cargo test --workspace
```

### 2. CLI ve Sistem Doğrulaması (`aether doctor`)
```bash
cargo run -p aether-cli -- doctor
cargo run -p aether-cli -- ask "Bugün Rust mimarisi üzerinde dinginlikle nasıl odaklanabilirim?" --hz 432
cargo run -p aether-cli -- memory search "Gemini 3.8 Flash"
cargo run -p aether-cli -- audio --hz 1760 --duration 1.5 --output crystal_chime.wav
```

### 3. MCP (Model Context Protocol) JSON-RPC 2.0 Sunucusu
```bash
cargo run -p aether-cli -- mcp
```

### 4. Koyu Buzlu Cam Masaüstü Komut Merkezi (`aether-desktop`)
```bash
cargo run -p aether-desktop
# Ardından http://127.0.0.1:4320 adresini veya Tauri v2 penceresini açın
```
