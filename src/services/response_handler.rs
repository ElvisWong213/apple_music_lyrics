use crate::models::apple_music::AppleMusic;
use crate::models::synced_lyric_xml::SynedLyric;
use crate::models::lyric_json::{Line, Lyrics, Word};
use std::{fs::File, io::Write};

pub struct Response {}

impl Response {
    
    pub(crate) fn create_json_file(data: &str) {
        let mut file = File::create("lyric.json").unwrap();
        file.write_all(data.as_bytes()).expect("Unable write data to file");
        println!("file create successfully")
    }
    
    pub(crate) fn extract_lyrics(text: &str) -> Result<Lyrics, String> {
        let mut lyrics = Lyrics::new();
        let response_json: AppleMusic = serde_json::from_str(text).or_else(|error| {
            Err(error.to_string())
        })?;
    
        let lyric_xml: SynedLyric = Self::extract_xml(&response_json).or_else(|error| {
            Err(error)
        })?;
        let lyric_array = lyric_xml.body.div;
        for div in lyric_array {
            for p in div.p {
                // println!("{:}, {:}", p.begin, p.end);
                let mut line: Line = Line::new(p.begin, p.end); 
                for span in p.span {
                    // println!("{:}, {:}, {:}", span.begin, span.end, span.word);
                    match span.span {
                        Some(background) => {
                            for word in background {
                                let word: Word = Word::new(word.begin.unwrap(), word.end.unwrap(), word.word.unwrap());
                                line.add_background(word);
                            }
                        }
                        None => {
                            let word: Word = Word::new(span.begin.unwrap(), span.end.unwrap(), span.word.unwrap());
                            line.add_words(word);
                        },
                    }
                }
                lyrics.add_line(line);
            }
        }
    
        Ok(lyrics)
    }
    
    pub(crate) fn extract_xml(json: &AppleMusic) -> Result<SynedLyric, String> {
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
}
