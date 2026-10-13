//! Zero-Guilt Focus & Self-Care Sanctuary State Machine (`aether_core::focus`).
//!
//! Entities in Aether never wither, dry up, or penalize the user.
//! Pausing or stopping early enters `SanctuaryState::Resting` ("Dinlenme Modu")
//! while celebrating every single minute of presence and every sip of water.

use crate::audio::HarmonicPreset;
use crate::guardrails::ZeroGuiltFilter;
use serde::{Deserialize, Serialize};

/// Organic state of the user's Focus & Self-Care Sanctuary.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SanctuaryState {
    /// Gently resting ("Dinlenme Modu — Yarın ve her an yeni bir başlangıç").
    Resting,
    /// Active harmonic focus flow ("Akış Halinde").
    Flowing,
    /// Blooming celebration after completing a mindful step ("Çiçeklenme").
    Blooming,
}

/// Snapshot of the user's sanctuary session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusSanctuary {
    pub state: SanctuaryState,
    pub intention: String,
    pub target_minutes: u32,
    pub completed_minutes: u32,
    pub hydration_ml: u32,
    pub active_preset: HarmonicPreset,
    pub compassionate_message: String,
}

impl Default for FocusSanctuary {
    fn default() -> Self {
        Self {
            state: SanctuaryState::Resting,
            intention: "Zihnine Berraklık, Akışına Dinginlik".to_string(),
            target_minutes: 25,
            completed_minutes: 0,
            hydration_ml: 250,
            active_preset: HarmonicPreset::EarthResonance432Hz,
            compassionate_message:
                "Dingin mabedin hazır. Ne zaman istersen 432 Hz akışını başlatabilirsin."
                    .to_string(),
        }
    }
}

impl FocusSanctuary {
    /// Starts or resumes a mindful focus session.
    pub fn start_flow(&mut self, intention: impl Into<String>, target_minutes: u32, hz: f32) {
        let raw_intention = intention.into();
        self.intention = ZeroGuiltFilter::sanitize(&raw_intention).sanitized;
        self.target_minutes = target_minutes.max(1);
        self.active_preset = HarmonicPreset::from_hz(hz);
        self.state = SanctuaryState::Flowing;
        self.compassionate_message = format!(
            "{} eşliğinde \"{}\" akışı başladı. Her dakikan dingin bir nefes.",
            self.active_preset.label_tr(),
            self.intention
        );
    }

    /// Records mindful minutes and transitions to `Blooming` when target is reached,
    /// or celebrates even 1 minute of presence.
    pub fn record_minutes(&mut self, minutes: u32) {
        self.completed_minutes = self.completed_minutes.saturating_add(minutes);
        if self.completed_minutes >= self.target_minutes {
            self.state = SanctuaryState::Blooming;
            self.compassionate_message = format!(
                "✨ Çiçeklenme tamamlandı! {} dakika boyunca dinginlikle akışta kaldın. 1760 Hz kristal çınlaması senin için.",
                self.completed_minutes
            );
        } else {
            self.compassionate_message = format!(
                "💧 {} dakikalık berrak odaklanma eklendi. Kendi ritminde ışıldamaya devam ediyorsun.",
                minutes
            );
        }
    }

    /// Transitions to `Resting` ("Dinlenme Modu") without ever resetting or blaming the user.
    pub fn enter_resting_mode(&mut self) {
        self.state = SanctuaryState::Resting;
        self.compassionate_message = format!(
            "🌙 Dinlenme Modu (Resting Sanctuary): Bugün biriktirdiğin {} dakika ve {} ml can suyu özenle korundu. Zihnin dinlenirken varlığın huzurla parlıyor.",
            self.completed_minutes, self.hydration_ml
        );
    }

    /// Celebrates even a 100 ml sip of water ("Can Suyu").
    pub fn add_hydration_ml(&mut self, ml: u32) {
        let added = ml.max(100);
        self.hydration_ml = self.hydration_ml.saturating_add(added);
        self.compassionate_message = format!(
            "💧 +{} ml can suyu! Tenine ışıltı, zihnine ferahlık kattın (Toplam: {} ml).",
            added, self.hydration_ml
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanctuary_never_punishes_and_preserves_progress_in_resting_mode() {
        let mut s = FocusSanctuary::default();
        s.start_flow("Rust Mimarisi", 25, 528.0);
        assert_eq!(s.state, SanctuaryState::Flowing);

        s.record_minutes(10);
        s.enter_resting_mode();
        assert_eq!(s.state, SanctuaryState::Resting);
        assert_eq!(s.completed_minutes, 10);
        assert!(ZeroGuiltFilter::is_compliant(&s.compassionate_message));
    }
}
