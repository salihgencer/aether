---
schema: "https://typeui.sh/schemas/design-md/v1.json"
project: "Aether (AuraMind)"
version: "1.0.0"
theme: "dark-frosted-glassmorphism"
philosophy: "zero-guilt-sanctuary"
tokens:
  colors:
    obsidian_dark: "#070D14"
    sky_blue: "#0EA5E9"
    cyan_glow: "#22D3EE"
    teal_calm: "#14B8A6"
    mint_fresh: "#2DD4BF"
    golden_amber: "#F59E0B"
    card_surface: "rgba(255, 255, 255, 0.05)"
    card_border: "rgba(34, 211, 238, 0.18)"
    text_primary: "#F8FAFC"
    text_secondary: "#94A3B8"
  radii:
    card: "28px"
    pill: "9999px"
    input: "18px"
  blur:
    frosted_glass: "20px"
  shadows:
    elevated_glass: "0 20px 40px -15px rgba(0, 0, 0, 0.6)"
  audio_frequencies_hz:
    earth_resonance: 432.0
    solar_warmth: 320.0
    foundation_calm: 174.0
    vitality_bloom: 528.0
    crystal_chime_primary: 1760.0
    crystal_chime_harmonic: 2637.0
  ai_models:
    primary: "gemini-3.8-flash"
    fallback: "gemini-2.5-flash"
  forbidden_words:
    - "ihmal"
    - "unuttun"
    - "ceza"
    - "hata"
    - "geç kaldın"
    - "eksik"
    - "başarısız"
---

# 🌌 Aether (AuraMind) — Design System & Sanctuary Specification (`DESIGN.md`)

> *"Zihnine Berraklık, Akışına Dinginlik."*

Bu doküman **Google `DESIGN.md` Açık Standardı** ve **TypeUI Design Skills** mimarisine uygun olarak hazırlanmıştır. Hem Rust/Tauri v2 masaüstü arayüzü hem de CLI/MCP çıktıları bu kurallara sıkı sıkıya bağlıdır.

---

## 🎨 1. Renk Evreni (Color Palette)

| Token | Hex / RGBA | Kullanım Alanı |
| :--- | :--- | :--- |
| `obsidian_dark` | `#070D14` | Ana pencere ve uzay derinliği zemini (**Asla `#000000` saf siyah kullanılmaz**) |
| `sky_blue` | `#0EA5E9` | Birincil aksiyonlar, odak akışı ve aktif durum vurguları |
| `cyan_glow` | `#22D3EE` | Işıltılı kenarlıklar, AI düşünme/akış göstergeleri |
| `teal_calm` | `#14B8A6` | Dingin zümrüt geçişleri, hafıza kasası rozetleri |
| `mint_fresh` | `#2DD4BF` | Tamamlanan dingin adımlar, pozitif geri bildirimler |
| `golden_amber` | `#F59E0B` | Öz-bakım kutlamaları, 528 Hz odak kristali |

---

## 🧊 2. Koyu Buzlu Cam Estetiği (Dark Frosted Glassmorphism)

- **Kart Yüzeyi:** `rgba(255, 255, 255, 0.05)`
- **Köşe Yarıçapı:** `28px` (`--radius-card: 28px`)
- **Kenarlık:** `1px solid rgba(34, 211, 238, 0.18)`
- **Bulanıklık (Backdrop):** `backdrop-filter: blur(20px)`
- **Derinlik Gölgesi:** `0 20px 40px -15px rgba(0, 0, 0, 0.6)`
- **Butonlar:** `Pill` (`9999px` yarıçap), `#0EA5E9` veya buzlu cam yüzey.

---

## 🎵 3. Duyusal Tasarım & Harmonik Frekanslar

Aether'in Rust tabanlı ses motoru (`aether_core::audio`) gerçek zamanlı saf sinüs ve harmonik zarflı ses dalgaları üretir:
- **432 Hz (Doğa & Berraklık):** Derin kodlama ve okuma seansları.
- **528 Hz (Canlanma & Yaratıcılık):** Beyin fırtınası ve tasarım akışı.
- **320 Hz (Dingin Denge):** Gün içi nefes ve zihin tazeleme.
- **174 Hz (Temel Huzur):** Yoğun tempo sonrası gevşeme.
- **1760 Hz / 2637 Hz (Kristal Su Çınlaması):** Komut tamamlandığında veya odak halkası dolduğunda tetiklenen akustik geri bildirim.

---

## 🕊️ 4. Sıfır Suçluluk Felsefesi (Zero-Guilt Philosophy)

- **Yasaklı Kelimeler:** `ihmal`, `unuttun`, `ceza`, `hata`, `geç kaldın`, `eksik`, `başarısız`.
- `aether_core::guardrails::ZeroGuiltFilter` modülü hem AI yanıtlarını hem de sistem mesajlarını tarayarak bu kelimeleri otomatik olarak şefkatli ve yapıcı alternatifleriyle (*"dinlenme molası"*, *"yeni bir başlangıç"*, *"nazikçe yeniden deneyelim"*) dönüştürür.
- Odak oturumları yarıda bırakıldığında sayaç asla sıfırlanıp kullanıcıyı suçlamaz; **"Dinlenme Modu (Resting Sanctuary)"** durumuna geçer ve o ana kadarki her dakikayı kutlar.
