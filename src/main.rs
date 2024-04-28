mod models;
mod services;

use reqwest::Client;
use core::panic;
use std::io::stdin;

use models::lyric_json::Lyrics;
use services::token_handler::Token;
use services::response_handler::Response;
use services::apple_music_url::Request;

#[tokio::main]
async fn main() {
    println!("Loading apple music access token...");
    let authorization = Token::get_access_token().await.unwrap();
    println!("Access token: Done!");

    println!("Loading user token...");
    let user_token = Token::get_user_token();
    println!("User token: Done!");
    
    let mut request = Request::new(authorization, user_token);
    
    println!("Get user storefront...");
    request.get_user_storefront().await;
    println!("User storefront: Done!");

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
    println!("Get song id...");
    let song_id = Request::get_song_id(&user_url_input);
    if song_id.is_empty() {
       panic!("Cannot found song id"); 
    }
    let url = request.create_lyrics_url(&song_id);
    println!("Get song id: Done!");

    println!("Get lyrics...");
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
    println!("Get lyrics: Done!");

    println!("Parsering lyrics...");
    let lyrics: Lyrics = Response::extract_lyrics(&res_string).unwrap();
    let output_string = serde_json::to_string(&lyrics).unwrap();
    println!("Parsering lyrics: Done!");

    println!("Creating file...");
    Response::create_json_file(&output_string);
    println!("Creating file: Done!");
}

