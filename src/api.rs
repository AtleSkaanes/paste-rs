use reqwest::Error;

pub const PASTE_RS_URL: &str = "https://paste.rs/";

pub fn to_url(id: &str, ext: Option<&str>) -> String {
    let mut url = format!("{}{}", PASTE_RS_URL, id);
    if let Some(ext) = ext {
        url = format!("{}.{}", url, ext);
    }
    url
}
pub fn strip_id(id: &str) -> &str {
    let mut id = id;
    id = id.strip_prefix("https://").unwrap_or(id);
    id = id.strip_prefix("http://").unwrap_or(id);
    id = id.strip_prefix("www.").unwrap_or(id);
    id = id.strip_prefix("paste.rs/").unwrap_or(id);

    id
}

#[derive(Clone, Debug, Default)]
pub struct SendResponse {
    pub status_code: reqwest::StatusCode,
    pub url: String,
    pub id: String,
}

#[derive(Clone, Debug, Default)]
pub struct GetResponse {
    pub text: String,
}

pub async fn send_post_request(text: String) -> Result<SendResponse, Error> {
    let client = reqwest::Client::new();

    let res = client
        .post(PASTE_RS_URL)
        .body(text)
        .header("Content-Type", "text/plain")
        .send()
        .await?;

    match res.error_for_status() {
        Ok(res) => {
            let status = res.status();
            let url = res.text().await?;
            let send_response = SendResponse {
                status_code: status,
                url: url.clone(),
                id: strip_id(&url).to_owned(),
            };
            Ok(send_response)
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn send_get_request(id: String) -> Result<GetResponse, Error> {
    let res = reqwest::get(format!("{}{}", PASTE_RS_URL, id)).await?;

    match res.error_for_status() {
        Ok(res) => {
            let get_response = GetResponse {
                text: res.text().await?,
            };
            Ok(get_response)
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn send_delete_request(id: String) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let response = client.delete(to_url(&id, None)).send().await?;

    match response.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
