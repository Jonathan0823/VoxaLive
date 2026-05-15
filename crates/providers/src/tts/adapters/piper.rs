use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::tts::{TtsProvider, TtsRequest, TtsResponse};
use voxalive_core::domain::CoreError;

#[derive(Debug, Clone)]
pub struct PiperAdapter {
    executable: String,
    model_path: PathBuf,
}

impl PiperAdapter {
    pub fn new(executable: impl Into<String>, model_path: impl Into<PathBuf>) -> Self {
        Self {
            executable: executable.into(),
            model_path: model_path.into(),
        }
    }

    fn map_error(message: impl Into<String>) -> CoreError {
        CoreError::new("PIPER_PROVIDER_ERROR", message)
    }
}

impl TtsProvider for PiperAdapter {
    fn synthesize(&self, request: TtsRequest) -> Result<TtsResponse, CoreError> {
        // Use a fixed output file path - piper writes here when ffplay is unavailable
        let output_file = "output.wav";

        let mut child = Command::new(&self.executable)
            .arg("-m")
            .arg(&self.model_path)
            .arg("-f")
            .arg(output_file)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| Self::map_error(err.to_string()))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(request.text.as_bytes())
                .map_err(|err| Self::map_error(err.to_string()))?;
        }

        let output = child
            .wait_with_output()
            .map_err(|err| Self::map_error(err.to_string()))?;

        if !output.status.success() {
            return Err(Self::map_error(format!(
                "piper exited with status {}",
                output.status
            )));
        }

        // Read the audio from the output file
        let audio_bytes = std::fs::read(output_file)
            .map_err(|err| Self::map_error(format!("failed to read output file: {}", err)))?;

        // Clean up the output file
        let _ = std::fs::remove_file(output_file);

        Ok(TtsResponse {
            audio_format: "wav".to_string(),
            audio_bytes,
        })
    }
}
