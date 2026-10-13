//! Type-safe Design System tokens and centralized API registry matching `DESIGN.md`.

use serde::{Deserialize, Serialize};

/// Official color palette for Aether's Dark Frosted Glassmorphic UI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColorPalette {
    /// `#070D14` Obsidian Space Blue (Never pure `#000000`).
    pub obsidian_dark: String,
    /// `#0EA5E9` Primary Water / Sky Blue.
    pub sky_blue: String,
    /// `#22D3EE` Cyan Glow for borders and active AI states.
    pub cyan_glow: String,
    /// `#14B8A6` Calm Teal.
    pub teal_calm: String,
    /// `#2DD4BF` Fresh Mint.
    pub mint_fresh: String,
    /// `#F59E0B` Golden Amber celebration accent.
    pub golden_amber: String,
    /// `rgba(255, 255, 255, 0.05)` translucent card surface.
    pub card_surface: String,
    /// `rgba(34, 211, 238, 0.18)` cyan glass border.
    pub card_border: String,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            obsidian_dark: "#070D14".to_string(),
            sky_blue: "#0EA5E9".to_string(),
            cyan_glow: "#22D3EE".to_string(),
            teal_calm: "#14B8A6".to_string(),
            mint_fresh: "#2DD4BF".to_string(),
            golden_amber: "#F59E0B".to_string(),
            card_surface: "rgba(255, 255, 255, 0.05)".to_string(),
            card_border: "rgba(34, 211, 238, 0.18)".to_string(),
        }
    }
}

/// Geometry and Frosted Glass tokens (`28px` card radius, `20px` blur).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GlassTokens {
    pub card_radius_px: u16,
    pub pill_radius_px: u16,
    pub backdrop_blur_px: u16,
    pub border_spec: String,
}

impl Default for GlassTokens {
    fn default() -> Self {
        Self {
            card_radius_px: 28,
            pill_radius_px: 9999,
            backdrop_blur_px: 20,
            border_spec: "1px solid rgba(34, 211, 238, 0.18)".to_string(),
        }
    }
}

/// AI Model standards enforcing the latest Gemini 3.8 Flash primary model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiModelRegistry {
    pub primary_model: String,
    pub fallback_model: String,
    pub lottie_mcp_url: String,
}

impl Default for AiModelRegistry {
    fn default() -> Self {
        Self {
            primary_model: "gemini-3.8-flash".to_string(),
            fallback_model: "gemini-2.5-flash".to_string(),
            lottie_mcp_url: "https://mcp.lottiefiles.com/mcp".to_string(),
        }
    }
}

/// Complete Aether Design & Platform Token bundle.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AetherDesignTokens {
    pub colors: ColorPalette,
    pub glass: GlassTokens,
    pub ai_models: AiModelRegistry,
}

impl AetherDesignTokens {
    /// Verifies that the token set strictly adheres to Dark Frosted Glassmorphism rules.
    pub fn validate(&self) -> bool {
        self.colors.obsidian_dark == "#070D14"
            && self.colors.obsidian_dark != "#000000"
            && self.glass.card_radius_px == 28
            && self.glass.backdrop_blur_px == 20
            && self.ai_models.primary_model == "gemini-3.8-flash"
            && self.ai_models.fallback_model == "gemini-2.5-flash"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_tokens_adhere_to_standards() {
        let tokens = AetherDesignTokens::default();
        assert!(tokens.validate());
        assert_eq!(tokens.colors.obsidian_dark, "#070D14");
        assert_ne!(tokens.colors.obsidian_dark, "#000000");
        assert_eq!(tokens.glass.card_radius_px, 28);
        assert_eq!(tokens.glass.backdrop_blur_px, 20);
        assert_eq!(tokens.ai_models.primary_model, "gemini-3.8-flash");
        assert_eq!(tokens.ai_models.fallback_model, "gemini-2.5-flash");
    }
}
