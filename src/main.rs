mod models;
mod services;

use core::panic;
use std::io::stdin;

use reqwest::header::HeaderMap;
use reqwest::{header, Client};
use models::lyric_json::Lyrics;
use models::user_storefront::UserStorefront;
use services::token_handler::Token;
use services::response_handler::Response;
use services::apple_music_url::URL;

#[tokio::main]
async fn main() {
    let authorization = Token::get_access_token().await.unwrap();
    let user_token = Token::get_user_token();
    let mut request = Request::new(authorization, user_token);
    request.get_user_storefront().await;

    let mut user_url_input: String = String::new();
    println!("Enter song url: ");
    stdin().read_line(&mut user_url_input).unwrap();
    user_url_input = match user_url_input.strip_suffix("\n") {
        Some(url) => url.to_string(),
        None => user_url_input
    };
    if user_url_input.is_empty() {
        panic!("URL is empty");
    }
    let song_id = URL::get_song_id(&user_url_input);
    if song_id.is_empty() {
       panic!("Cannot found song id"); 
    }
    let url = URL::create_lyrics_url(&song_id, &request.storefront);

    let headers = request.create_header();
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = match client.get(url).send().await {
        Ok(r) => {
            if r.status() != 200 {
                panic!("Invalid response: {:}", r.status())
            }
            r
        },
        Err(error) => {
           panic!("{:}", error.to_string()); 
        }
    };
    let res_string = res.text().await.unwrap();

    let lyrics: Lyrics = Response::extract_lyrics(&res_string).unwrap();
    let output_string = serde_json::to_string(&lyrics).unwrap();

    Response::create_json_file(&output_string);
}

struct Request {
    authorization: String,
    user_token: String,
    storefront: String,
}

impl Request {
    fn new(authorization: String, user_token: String) -> Self {
        Self {
            authorization,
            user_token,
            storefront: String::new(),
        }
    }
    
    fn create_header(&mut self) -> HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert("origin", "https://music.apple.com".parse().unwrap());
        headers.insert("Authorization", self.authorization.parse().unwrap());
        headers.insert("Media-User-Token", self.user_token.parse().unwrap());
        headers
    }

    async fn get_user_storefront(&mut self) {
        let headers = self.create_header();
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let res = client.get("https://api.music.apple.com/v1/me/storefront").send().await.unwrap();
        let res_string = res.text().await.unwrap();
        let user_storefront: UserStorefront = serde_json::from_str(&res_string).unwrap();
        self.storefront = match user_storefront.data.first() {
            Some(data) => {
                data.id.to_owned()
            }
            None =>  {
                panic!("Can't found user storefront")
            }
        }
    }
}
