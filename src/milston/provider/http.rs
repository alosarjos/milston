use crate::error::Result;
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "http")]
use url::Url;

pub struct HttpDataProvider;

impl HttpDataProvider {
    pub async fn save_data<T>(data: &T, url: &Url) -> Result<()>
    where
        T: Serialize + Send + Sync,
    {
        let client = reqwest::Client::new();
        let _response = client
            .post(url.clone())
            .body(serde_json::to_string(data)?)
            .send()
            .await?;

        Ok(())
    }

    pub async fn load_data<T: DeserializeOwned>(url: &Url) -> Result<T> {
        let response = reqwest::get(url.clone()).await?;
        let response_text = response.text().await?;
        Ok(serde_json::from_str(&response_text)?)
    }
}
