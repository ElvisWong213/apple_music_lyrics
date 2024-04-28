pub struct URL {}

impl URL {
    pub(crate) fn get_song_id(url: &str) -> String {
        let mut match_header: i8 = 0;
        let mut id: String = String::new();
        for c in url.chars() {
            if match_header == 2 {
                match c.to_digit(10) {
                    Some(_) => {
                        id.push(c);
                    },
                    None => {
                        match_header = 0;
                        continue;
                    }
                }
            }
            if match_header == 1 && c == '=' {
                match_header = 2;
            } else if c == 'i' {
                match_header = 1;
            }
        }
        id
    } 

    pub(crate) fn create_lyrics_url(song_id: &str, storefront: &str) -> String {
        format!("https://amp-api.music.apple.com/v1/catalog/{}/songs/{}?include[songs]=albums,lyrics,syllable-lyrics", storefront, song_id)
    }
}

#[cfg(test)]
mod test {
    use crate::services::apple_music_url::URL;

    #[test]
    fn get_song_id_test() {
        let url_a = "https://music.apple.com/hk/album/%e7%84%a1%e7%ad%94%e6%a1%88/1729188120?i=1729188121&l=en-gb";
        let url_b = "https://music.apple.com/hk/album/%e7%84%a1%e7%ad%94%e6%a1%88/1729188120?i=1729188121&l=zh-hant-tw";
        assert_eq!("1729188121", URL::get_song_id(url_a));
        assert_eq!("1729188121", URL::get_song_id(url_b));
    }

    #[test]
    fn get_song_id_test_empty() {
        assert_eq!("", URL::get_song_id(""));
        assert_eq!("", URL::get_song_id("https://music.apple.com/hk/album/%e7%84%a1%e7%ad%94%e6%a1%88/1729188120?i=&l=zh-hant-tw"));
        assert_eq!("", URL::get_song_id("https://music.apple.com/hk/album/%e7%84%a1%e7%ad%94%e6%a1%88/1729188120?l=zh-hant-tw"));
    }
}
