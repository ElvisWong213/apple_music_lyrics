use crate::models::apple_music::AppleMusic;
use crate::models::synced_lyric_xml::SynedLyricXML;
use crate::models::lyric_json::{Line, LyricsJSON, Word};
use crate::models::lyric_xml::LyricXML;
use std::char;
use std::{fs::File, io::Write};

pub struct Response {}

impl Response {
    
    pub(crate) fn create_json_file(data: &str) {
        let mut file = File::create("lyric.json").unwrap();
        file.write_all(data.as_bytes()).expect("Unable write data to file");
    }
    
    pub(crate) fn extract_lyrics(text: &str) -> Result<LyricsJSON, String> {
        let mut lyrics = LyricsJSON::new();
        let response_json: AppleMusic = serde_json::from_str(text).or_else(|error| {
            Err(error.to_string())
        })?;
    
        let synced_lyric_xml: SynedLyricXML = Self::extract_syned_lyric_xml(&response_json).or_else(|error| {
            Err(error)
        })?;
        let lyric_xml: LyricXML = Self::extract_lyric_xml(&response_json).or_else(|error| {
            Err(error)
        })?;

        // Self::convert_to_json(&synced_lyric_xml, &mut lyrics);
        Self::convert_formated_data_to_json(&synced_lyric_xml, &lyric_xml, &mut lyrics);
    
        Ok(lyrics)
    }

    fn convert_to_json(synced_lyric_xml: &SynedLyricXML, lyrics: &mut LyricsJSON) {
        let synced_lyric_array = &synced_lyric_xml.body.div;
        for div in synced_lyric_array {
            for p in &div.p {
                let mut line: Line = Line::new(p.begin.clone(), p.end.clone()); 
                for span in &p.span {
                    let span = span.clone();
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
    }

    fn convert_formated_data_to_json(synced_lyric_xml: &SynedLyricXML, lyric_xml: &LyricXML, lyrics: &mut LyricsJSON) {
        let synced_lyric_array = &synced_lyric_xml.body.div;
        let lyric_array = &lyric_xml.body.div;
        for (i_div, div) in synced_lyric_array.iter().enumerate() {
            for (i_p, p) in div.p.iter().enumerate() {
                let mut line: Line = Line::new(p.begin.clone(), p.end.clone()); 
                let lyric_chars: Vec<char> = lyric_array[i_div].p[i_p].line.clone().chars().collect();
                let mut lyric_char_index: usize = 0;
                for span in &p.span {
                    let span = span.clone();
                    match span.span {
                        Some(background) => {
                            for span in background {
                                let mut word = span.word.clone().unwrap();
                                let chars: Vec<char> = word.chars().collect();
                                Self::formatter(chars, &lyric_chars, &mut lyric_char_index, &mut word);
                                let word: Word = Word::new(span.begin.unwrap(), span.end.unwrap(), word);
                                line.add_background(word);
                            }
                        }
                        None => {
                            let mut word = span.word.clone().unwrap();
                            let chars: Vec<char> = word.chars().collect();
                            Self::formatter(chars, &lyric_chars, &mut lyric_char_index, &mut word);
                            let word: Word = Word::new(span.begin.unwrap(), span.end.unwrap(), word);
                            line.add_words(word);
                        },
                    }
                }
                lyrics.add_line(line);
            }
        }
    }

    fn formatter(synced_lyric_chars: Vec<char>, lyric_chars: &Vec<char>, lyric_char_index: &mut usize, word: &mut String) {
        for c in synced_lyric_chars {
            if *lyric_char_index >= lyric_chars.len() {
            println!("{:}", word);
                return;
            }
            if c == lyric_chars[*lyric_char_index] {
                *lyric_char_index += 1;
                continue; 
            }
        }
        if *lyric_char_index >= lyric_chars.len() {
            return;
        }
        if lyric_chars[*lyric_char_index] == ' ' {
            *lyric_char_index += 1;
            word.push(' ');
        }
    }
    
    fn extract_syned_lyric_xml(json: &AppleMusic) -> Result<SynedLyricXML, String> {
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
        let output: SynedLyricXML = match quick_xml::de::from_str(&ttml) {
            Ok(data) => data,
            Err(error) => {
                Err(error.to_string())?
            }
        };
        Ok(output)
    }

    fn extract_lyric_xml(json: &AppleMusic) -> Result<LyricXML, String> {
        let data = match json.data.first() {
            Some(data) => data,
            None => { Err("empty data".to_string())? },
        };
        let ttml = match data.relationships.lyrics.data.first() {
            Some(lyric_data) => {
                &lyric_data.attributes.ttml
            }
            None => { Err("empty data".to_string())? } 
        };
        let output: LyricXML = match quick_xml::de::from_str(&ttml) {
            Ok(data) => data,
            Err(error) => {
                Err(error.to_string())?
            }
        };
        Ok(output)
    }
}
