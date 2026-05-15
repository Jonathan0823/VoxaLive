use std::path::PathBuf;

use crate::stt::{SttProvider, SttRequest, SttResponse};
use voxalive_core::domain::CoreError;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

#[derive(Debug)]
pub struct WhisperAdapter {
    context: WhisperContext,
}

impl WhisperAdapter {
    pub fn new(model_path: impl Into<PathBuf>) -> Result<Self, CoreError> {
        let model_path = model_path.into();
        let context = WhisperContext::new_with_params(
            &model_path.to_string_lossy(),
            WhisperContextParameters::default(),
        )
        .map_err(|err| CoreError::new("WHISPER_PROVIDER_ERROR", err.to_string()))?;

        Ok(Self { context })
    }

    fn map_error(message: impl Into<String>) -> CoreError {
        CoreError::new("WHISPER_PROVIDER_ERROR", message)
    }

    fn decode_pcm16_le(audio_bytes: &[u8]) -> Vec<f32> {
        audio_bytes
            .chunks_exact(2)
            .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]) as f32 / i16::MAX as f32)
            .collect()
    }
}

impl SttProvider for WhisperAdapter {
    fn transcribe(&self, request: SttRequest) -> Result<SttResponse, CoreError> {
        if request.audio_format != "pcm16" {
            return Err(Self::map_error(format!(
                "unsupported audio format: {}",
                request.audio_format
            )));
        }

        let mut state = self
            .context
            .create_state()
            .map_err(|err| Self::map_error(err.to_string()))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        if let Some(language) = request.language.as_deref() {
            if !language.is_empty() && language != "auto" {
                params.set_language(Some(language));
            }
        }
        params.set_print_progress(false);

        let audio_data = Self::decode_pcm16_le(&request.audio_bytes);
        state
            .full(params, &audio_data)
            .map_err(|err| Self::map_error(err.to_string()))?;

        let num_segments = state
            .full_n_segments()
            .map_err(|err| Self::map_error(err.to_string()))?;

        let mut transcript = String::new();
        for i in 0..num_segments {
            let segment = state
                .full_get_segment_text(i)
                .map_err(|err| Self::map_error(err.to_string()))?;
            transcript.push_str(&segment);
        }

        Ok(SttResponse { transcript })
    }
}
