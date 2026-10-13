//! Harmonic Ambient & Crystal Water Chime Synthesizer (`aether_core::audio`).
//!
//! Synthesizes 16-bit mono PCM WAV audio in memory for:
//! - Ambient harmonic presets: `432 Hz`, `528 Hz`, `320 Hz`, `174 Hz`
//! - Crystal Water Chime: `1760 Hz` + `2637 Hz` dual-harmonic bell resonance.

use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Official Aether Harmonic Frequency Presets.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum HarmonicPreset {
    /// 432 Hz — Natural clarity & deep coding flow.
    EarthResonance432Hz,
    /// 528 Hz — Creative bloom & vitality.
    VitalityBloom528Hz,
    /// 320 Hz — Calm solar balance & breathing.
    SolarBalance320Hz,
    /// 174 Hz — Deep grounding & sanctuary rest.
    FoundationCalm174Hz,
    /// 1760 Hz + 2637 Hz — Crystal Water Chime (celebration & tactile feedback).
    CrystalWaterChime,
}

impl HarmonicPreset {
    pub fn primary_hz(&self) -> f32 {
        match self {
            Self::EarthResonance432Hz => 432.0,
            Self::VitalityBloom528Hz => 528.0,
            Self::SolarBalance320Hz => 320.0,
            Self::FoundationCalm174Hz => 174.0,
            Self::CrystalWaterChime => 1760.0,
        }
    }

    pub fn overtone_hz(&self) -> f32 {
        match self {
            Self::EarthResonance432Hz => 648.0,
            Self::VitalityBloom528Hz => 792.0,
            Self::SolarBalance320Hz => 480.0,
            Self::FoundationCalm174Hz => 261.0,
            Self::CrystalWaterChime => 2637.0,
        }
    }

    pub fn label_tr(&self) -> &'static str {
        match self {
            Self::EarthResonance432Hz => "432 Hz • Doğa & Derin Odak Rezonansı",
            Self::VitalityBloom528Hz => "528 Hz • Yaratıcı Çiçeklenme & Canlılık",
            Self::SolarBalance320Hz => "320 Hz • Dingin Nefes & Zihin Dengesi",
            Self::FoundationCalm174Hz => "174 Hz • Temel Huzur & Dinlenme Mabedi",
            Self::CrystalWaterChime => "1760/2637 Hz • Kristal Su Çınlaması",
        }
    }

    pub fn from_hz(hz: f32) -> Self {
        let diff = |target: f32| (hz - target).abs();
        if diff(528.0) < 20.0 {
            Self::VitalityBloom528Hz
        } else if diff(320.0) < 20.0 {
            Self::SolarBalance320Hz
        } else if diff(174.0) < 20.0 {
            Self::FoundationCalm174Hz
        } else if diff(1760.0) < 100.0 || diff(2637.0) < 100.0 {
            Self::CrystalWaterChime
        } else {
            Self::EarthResonance432Hz
        }
    }
}

/// Pure-Rust WAV generator producing valid RIFF WAVE 16-bit PCM audio bytes.
pub struct HarmonicSynthesizer;

impl HarmonicSynthesizer {
    /// Synthesizes a 16-bit mono PCM WAV buffer for the given preset and duration.
    pub fn synthesize_wav(preset: HarmonicPreset, duration_secs: f32, sample_rate: u32) -> Vec<u8> {
        let total_samples = ((duration_secs.max(0.1)) * sample_rate as f32) as usize;
        let f1 = preset.primary_hz();
        let f2 = preset.overtone_hz();

        let mut pcm_samples = Vec::with_capacity(total_samples);
        for i in 0..total_samples {
            let t = i as f32 / sample_rate as f32;
            let progress = i as f32 / total_samples as f32;

            // Smooth attack and exponential/cosine decay envelope
            let envelope = match preset {
                HarmonicPreset::CrystalWaterChime => {
                    let attack = (t / 0.015).min(1.0);
                    let decay = (-3.8 * t / duration_secs.max(0.2)).exp();
                    attack * decay
                }
                _ => {
                    let fade_in = (progress / 0.1).min(1.0);
                    let fade_out = ((1.0 - progress) / 0.15).min(1.0);
                    fade_in * fade_out
                }
            };

            let primary_wave = (2.0 * PI * f1 * t).sin();
            let harmonic_wave = (2.0 * PI * f2 * t).sin() * 0.35;
            let combined = (primary_wave * 0.72 + harmonic_wave) * envelope * 0.5;
            let sample_i16 = (combined.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
            pcm_samples.push(sample_i16);
        }

        encode_riff_wav(&pcm_samples, sample_rate)
    }
}

fn encode_riff_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let data_len = (samples.len() * 2) as u32;
    let file_len = 36 + data_len;
    let mut buf = Vec::with_capacity(file_len as usize + 8);

    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&file_len.to_le_bytes());
    buf.extend_from_slice(b"WAVE");

    // fmt subchunk
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes()); // PCM chunk size
    buf.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat = 1 (PCM)
    buf.extend_from_slice(&1u16.to_le_bytes()); // NumChannels = 1 (Mono)
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    let byte_rate = sample_rate * 2;
    buf.extend_from_slice(&byte_rate.to_le_bytes());
    buf.extend_from_slice(&2u16.to_le_bytes()); // BlockAlign
    buf.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample

    // data subchunk
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_len.to_le_bytes());
    for &sample in samples {
        buf.extend_from_slice(&sample.to_le_bytes());
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic_wav_generation_produces_valid_riff_header() {
        let wav = HarmonicSynthesizer::synthesize_wav(HarmonicPreset::CrystalWaterChime, 0.5, 44100);
        assert!(wav.len() > 44);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[12..16], b"fmt ");
        assert_eq!(&wav[36..40], b"data");
    }

    #[test]
    fn test_preset_frequencies_match_design_md() {
        assert_eq!(HarmonicPreset::EarthResonance432Hz.primary_hz(), 432.0);
        assert_eq!(HarmonicPreset::VitalityBloom528Hz.primary_hz(), 528.0);
        assert_eq!(HarmonicPreset::SolarBalance320Hz.primary_hz(), 320.0);
        assert_eq!(HarmonicPreset::FoundationCalm174Hz.primary_hz(), 174.0);
        assert_eq!(HarmonicPreset::CrystalWaterChime.primary_hz(), 1760.0);
        assert_eq!(HarmonicPreset::CrystalWaterChime.overtone_hz(), 2637.0);
    }
}
