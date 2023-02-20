//Adding a new client 
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct AuthRequestBody {
    client_id: String,
    client_secret: String,
    audience: String,
    grant_type: String,
}

#[derive(Deserialize)]
struct AuthResponseBody {
    access_token: String,
    //token_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://acmenm.us.auth0.com/oauth/token";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let request_body = AuthRequestBody {
        client_id: String::from("FINvEQNDzH7x5yiNF50QyTgUt2fwI0dn"),
        client_secret: String::from("vMQhmTZjILxRu1YaMK0Be85Ab74IPwXrtpMZRO6CIxDts-BPKDK_IaZS3L_zg98p"),
        audience: String::from("https://ku934wgxji.us-east-1.awsapprunner.com/"),
        grant_type: String::from("client_credentials"),
    };

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()?;

    if response.status().is_success() {
        let auth_response: AuthResponseBody = response.json()?;
        println!("Access token: {}", auth_response.access_token);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

