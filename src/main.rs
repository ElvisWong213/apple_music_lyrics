mod models;
mod services;

use reqwest::header::HeaderMap;
use reqwest::{header, Client};
use models::lyric_json::Lyrics;
use services::token_handler::Token;
use services::response_handler::Response;

#[tokio::main]
async fn main() {
    let authorization = Token::get_access_token().await.unwrap();
    let user_token = Token::get_user_token();

    let url = "https://amp-api.music.apple.com/v1/catalog/hk/songs/1729188121?include[songs]=albums,lyrics,syllable-lyrics&l=zh-Hant-HK";

    let headers = create_header(&authorization, &user_token);
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = client.get(url).send().await.unwrap();
    let res_string = res.text().await.unwrap();

    let lyrics: Lyrics = Response::extract_lyrics(&res_string).unwrap();
    let output_string = serde_json::to_string(&lyrics).unwrap();

    Response::create_json_file(&output_string);
}

fn create_header(authorization: &str, user_token: &str) -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("origin", "https://music.apple.com".parse().unwrap());
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Media-User-Token", user_token.parse().unwrap());
    headers
}

