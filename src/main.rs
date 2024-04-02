mod apple_music;
mod synced_lyric_xml;
mod lyric_json;

use fancy_regex::Regex;
use reqwest::{header, Client};
use apple_music::AppleMusic;
use synced_lyric_xml::SynedLyric;
use lyric_json::{Line, Lyrics, Chunk};
use std::{env, fs::File, io::Write};

#[tokio::main]
async fn main() {
    let authorization = get_access_token().await.unwrap();
    let user_token = get_user_token();

    let url = "https://amp-api.music.apple.com/v1/catalog/hk/songs/1729188121?include[songs]=albums,lyrics,syllable-lyrics&l=zh-Hant-HK";

    let mut headers = header::HeaderMap::new();
    headers.insert("origin", "https://music.apple.com".parse().unwrap());
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Media-User-Token", user_token.parse().unwrap());
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = client.get(url).send().await.unwrap();
    let res_string = res.text().await.unwrap();

    let lyrics: Lyrics = extract_lyrics(&res_string).unwrap();
    let output_string = serde_json::to_string(&lyrics).unwrap();

    create_json_file(&output_string);
}

fn get_user_token() -> String {
    dotenv::from_path("./.env").unwrap();
    env::var("USER_TOKEN").unwrap()
}

async fn get_access_token() -> Result<String, String> {
    let res = reqwest::get("https://music.apple.com").await.or_else(|error| {
        Err(error.to_string())
    })?;
    if res.status().as_u16() != 200 {
        Err("Unable to get music.apple.com".to_string())?
    }
    let res_text = res.text().await.or_else(|error| {
        Err(error.to_string())
    })?;
    
    let js_re = Regex::new(r#"(?<=index)(.*?)(?=\.js")"#).unwrap();
    let js_file = js_re.find(&res_text).unwrap().unwrap().as_str();
    let js_res = reqwest::get(format!("https://music.apple.com/assets/index{js_file}.js")).await.unwrap();
    if js_res.status().as_u16() != 200 {
        Err("Unable to get js file".to_string())?
    }
    let js_res_text = js_res.text().await.unwrap();
    
    let token_re = Regex::new(r#"(?=eyJh)(.*?)(?=")"#).unwrap();
    let token = match token_re.find(&js_res_text) {
        Ok(data) => {
            match data {
                Some(value) => value.as_str(),
                None => {
                    Err("Token is empty".to_string())?
                }
            }
        },
        Err(error) => {
            Err(error.to_string())?
        }
    };
    Ok(format!("Bearer {token}"))
}

fn create_json_file(data: &str) {
    let mut file = File::create("lyric.json").unwrap();
    file.write_all(data.as_bytes()).expect("Unable write data to file");
}

fn extract_lyrics(text: &str) -> Result<Lyrics, String> {
    let mut lyrics = Lyrics::new();
    let response_json: AppleMusic = serde_json::from_str(text).or_else(|error| {
        Err(error.to_string())
    })?;

    let lyric_xml: SynedLyric = extract_xml(&response_json).or_else(|error| {
        Err(error)
    })?;
    let lyric_array = lyric_xml.body.div;
    for div in lyric_array {
        for p in div.p {
            // println!("{:}, {:}", p.begin, p.end);
            let mut line: Line = Line::new(p.begin, p.end); 
            for span in p.span {
                // println!("{:}, {:}, {:}", span.begin, span.end, span.word);
                let chunk: Chunk = Chunk::new(span.begin, span.end, span.word);
                line.add_chunk(chunk);
            }
            lyrics.add_line(line);
        }
    }

    Ok(lyrics)
}

fn extract_xml(json: &AppleMusic) -> Result<SynedLyric, String> {
    let data = match json.data.first() {
        Some(data) => data,
        None => { Err("empty data".to_string())? },
    };
    let ttml = match data.relationships.syllable_lyrics.data.first() {
        Some(lyric_data) => {
            &lyric_data.attributes.ttml
        }
        None => { Err("empty data".to_string())? } 
    };
    let output: SynedLyric = match quick_xml::de::from_str(&ttml) {
        Ok(data) => data,
        Err(error) => {
            Err(error.to_string())?
        }
    };
    Ok(output)
}
