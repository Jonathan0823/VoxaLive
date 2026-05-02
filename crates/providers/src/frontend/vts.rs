use std::sync::Mutex;

use serde_json::json;
use tungstenite::{connect, Message};
use crate::frontend::{FrontendAdapter, FrontendOutput};
use voxalive_core::domain::CoreError;

#[derive(Debug)]
pub struct VtsAdapter {
    endpoint: String,
    plugin_name: String,
    plugin_developer: String,
    auth_token: Mutex<Option<String>>,
}

impl VtsAdapter {
    pub fn new(
        endpoint: impl Into<String>,
        plugin_name: impl Into<String>,
        plugin_developer: impl Into<String>,
        auth_token: Option<String>,
    ) -> Self {
        Self {
            endpoint: endpoint.into(),
            plugin_name: plugin_name.into(),
            plugin_developer: plugin_developer.into(),
            auth_token: Mutex::new(auth_token),
        }
    }

    fn map_error(message: impl Into<String>) -> CoreError {
        CoreError::new("VTS_PROVIDER_ERROR", message)
    }

    fn connect(&self) -> Result<tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, CoreError> {
        let (socket, _) = connect(self.endpoint.as_str()).map_err(|err| Self::map_error(err.to_string()))?;
        Ok(socket)
    }

    fn send_message(&self, message: serde_json::Value) -> Result<serde_json::Value, CoreError> {
        let mut socket = self.connect()?;
        socket
            .send(Message::Text(message.to_string().into()))
            .map_err(|err| Self::map_error(err.to_string()))?;

        let reply = socket
            .read()
            .map_err(|err| Self::map_error(err.to_string()))?;

        match reply {
            Message::Text(text) => {
                serde_json::from_str::<serde_json::Value>(&text).map_err(|err| Self::map_error(err.to_string()))
            }
            other => Err(Self::map_error(format!("unexpected websocket message: {:?}", other))),
        }
    }

    pub fn request_authentication_token(&self) -> Result<String, CoreError> {
        let response = self.send_message(json!({
            "apiName": "VTubeStudioPublicAPI",
            "apiVersion": "1.0",
            "requestID": "auth-token-request",
            "messageType": "AuthenticationTokenRequest",
            "data": {
                "pluginName": &self.plugin_name,
                "pluginDeveloper": &self.plugin_developer,
                "pluginIcon": ""
            }
        }))?;

        response
            .get("data")
            .and_then(|data| data.get("authenticationToken"))
            .and_then(|token| token.as_str())
            .map(|token| token.to_string())
            .ok_or_else(|| Self::map_error("VTube Studio did not return an auth token"))
    }

    pub fn authenticate(&self) -> Result<(), CoreError> {
        let token = {
            let guard = self.auth_token.lock().expect("auth token mutex poisoned");
            guard.clone()
        };

        let token = match token {
            Some(token) => token,
            None => {
                let token = self.request_authentication_token()?;
                *self.auth_token.lock().expect("auth token mutex poisoned") = Some(token.clone());
                token
            }
        };

        self.send_message(json!({
            "apiName": "VTubeStudioPublicAPI",
            "apiVersion": "1.0",
            "requestID": "auth-session-001",
            "messageType": "AuthenticationRequest",
            "data": {
                "pluginName": &self.plugin_name,
                "pluginDeveloper": &self.plugin_developer,
                "authenticationToken": token
            }
        }))?;

        Ok(())
    }

    pub fn inject_parameter_data(
        &self,
        mode: &str,
        parameter_values: Vec<VtsParameterValue>,
    ) -> Result<(), CoreError> {
        self.authenticate()?;

        self.send_message(json!({
            "apiName": "VTubeStudioPublicAPI",
            "apiVersion": "1.0",
            "requestID": "inject-001",
            "messageType": "InjectParameterDataRequest",
            "data": {
                "faceFound": true,
                "mode": mode,
                "parameterValues": parameter_values,
            }
        }))?;

        Ok(())
    }
}

impl FrontendAdapter for VtsAdapter {
    fn send_output(&self, output: FrontendOutput) -> Result<(), CoreError> {
        let mut parameter_values = Vec::new();
        let has_audio = output.audio_bytes.is_some();

        if let Some(text) = output.text {
            parameter_values.push(VtsParameterValue::float("VoxaLiveTextLength", text.len() as f64, None));
        }

        if let Some(audio_bytes) = output.audio_bytes {
            parameter_values.push(VtsParameterValue::float("VoxaLiveAudioBytes", audio_bytes.len() as f64, None));
        }

        let mode = if has_audio { "add" } else { "set" };
        self.inject_parameter_data(mode, parameter_values)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct VtsParameterValue {
    pub id: String,
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

impl VtsParameterValue {
    pub fn float(id: impl Into<String>, value: f64, weight: Option<f64>) -> Self {
        Self {
            id: id.into(),
            value,
            weight,
        }
    }
}
