use reqwest::{Client, multipart::*};
use tracing::info;

use crate::config::PinataConfig;
use crate::error::{UniAxNftErr, UniAxNftResult};

#[derive(Debug)]
struct IpsfInfo {
    pub id: String,
    pub user_id: String,
    pub cid: String,
}

pub struct PinataSrv {
    pinata_client: Client,
    config: PinataConfig,
}

impl PinataSrv {
    pub fn new(config: PinataConfig) -> Self {
        Self {
            pinata_client: reqwest::Client::new(),
            config: config,
        }
    }

    // upload file to pinata. It returns json string of response message (if success)
    pub async fn upload_file(&self, raw: Vec<u8>, filename: &str) -> UniAxNftResult<IpsfInfo> {
        let form = Form::new()
            .text("network", "public")
            .part("file", Part::bytes(raw).file_name(filename.to_string()));

        let response = self
            .pinata_client
            .post(format!("{}/files", &self.config.upload_url))
            .bearer_auth(&self.config.jwt)
            .multipart(form)
            .send()
            .await
            .map_err(|e| UniAxNftErr::PinataErr(format!("upload file err: {}", e)))?;

        if !response.status().is_success() {
            return Err(UniAxNftErr::PinataErr(format!(
                "unexpected response: {}",
                response.status()
            )));
        }

        let response_body_text = response.text().await.map_err(|e| {
            UniAxNftErr::PinataErr(format!("Failed to parse response str, err: {}", e))
        })?;
        info!("pinata response: {}", response_body_text);

        let response: serde_json::Value =
            serde_json::from_str(&response_body_text).map_err(|e| {
                UniAxNftErr::PinataErr(format!("Failed to parse response json, err: {}", e))
            })?;
        let response_data = &response["data"];
        if response_data.is_null() {
            return Err(UniAxNftErr::PinataErr(
                "Failed to find \"data\" in response body".to_string(),
            ));
        }

        let map_to_pinata_err =
            |e| UniAxNftErr::PinataErr(format!("illegal json k-v field, err field: {}", e));
        Ok(IpsfInfo {
            id: response_data["id"]
                .as_str()
                .ok_or("id")
                .map_err(map_to_pinata_err)?
                .to_string(),
            user_id: response_data["user_id"]
                .as_str()
                .ok_or("user_id")
                .map_err(map_to_pinata_err)?
                .to_string(),
            cid: response_data["cid"]
                .as_str()
                .ok_or("cid")
                .map_err(map_to_pinata_err)?
                .to_string(),
        })
    }
}
