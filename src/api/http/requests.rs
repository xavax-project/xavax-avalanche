use reqwest::{Response};
use serde_json::{self, Value};
use tracing::trace;
/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    http calls to nodes will be done as a simple abstraction of the reqwest API,
    that's what post.rs does.
*/


/// # post_json
/// ___
/// `post_json` will make a post request with the given json_body, to any specified destination.
/// ## todo: better docs.
pub async fn post_json(json_body: Value, host: &String, endpoint: &String, port: &u16) -> Result<Response, String> {
    trace!("Creating post request with the value: {0}", json_body.to_string());
    let json: String = json_body.to_string();
    let client = reqwest::ClientBuilder::default()
    .build().expect("Failed building reqwest client");

    let url = format!("https://{0}:{1}{2}", host, port, endpoint);

    trace!("Sending https post request to destination: {0}", url);
    
    match client.post(url)
    .body(json)
    .header("Content-Type", "application/json")
    .header("Accept", "*/*")
    .header("Accept-Encoding", "gzip, deflate, br")
    .header("Connection", "keep-alive")
    .header("Host", host)
    .send().await
    {
        Ok(r) => {
            Ok(r)
        },
        Err(e) => {
            Err(format!("Post Request Error: {}",e.to_string()))
        },
    }
}