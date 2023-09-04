use reqwest::Response as ReqwestResponse;

#[derive(Debug)]
pub struct TextResponse {
    pub status: u16,
    pub content_type: Option<String>,
    pub body: String,
}

impl TextResponse {
    /// Create a new `TextResponse` from a reqwest Response
    ///
    /// # Panics
    ///
    /// Panics if the response body is not valid UTF-8 or if the response does
    /// not contain a content-type header.
    pub async fn from(response: ReqwestResponse) -> Self {
        Self {
            status: response.status().as_u16(),
            content_type: response
                .headers()
                .get("content-type")
                .map(|content_type| content_type.to_str().unwrap().to_owned()),
            body: response.text().await.unwrap(),
        }
    }

    #[must_use]
    pub fn is_json_and_ok(&self) -> bool {
        self.is_ok() && self.is_json()
    }

    #[must_use]
    pub fn is_json(&self) -> bool {
        if let Some(content_type) = &self.content_type {
            return content_type == "application/json";
        }
        false
    }

    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.status == 200
    }
}

#[derive(Debug)]
pub struct BinaryResponse {
    pub status: u16,
    pub content_type: Option<String>,
    pub bytes: Vec<u8>,
}

impl BinaryResponse {
    /// Create a new `BinaryResponse` from a reqwest Response
    ///
    /// # Panics
    ///
    /// Panics if the response body does not contain binary data or if the
    /// response does not contain a content-type header.
    pub async fn from(response: ReqwestResponse) -> Self {
        Self {
            status: response.status().as_u16(),
            content_type: response
                .headers()
                .get("content-type")
                .map(|content_type| content_type.to_str().unwrap().to_owned()),
            bytes: response.bytes().await.unwrap().to_vec(),
        }
    }
    #[must_use]
    pub fn is_bittorrent_and_ok(&self) -> bool {
        self.is_ok() && self.is_bittorrent()
    }

    #[must_use]
    pub fn is_bittorrent(&self) -> bool {
        if let Some(content_type) = &self.content_type {
            return content_type == "application/x-bittorrent";
        }
        false
    }

    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.status == 200
    }
}
