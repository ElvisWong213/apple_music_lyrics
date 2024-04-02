use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Lyrics {
    lines: Vec<Line>,
}

#[derive(Serialize, Deserialize)]
pub struct Line {
    begin: String,
    end: String,
    chunks: Vec<Chunk>,
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    begin: String,
    end: String,
    text: String,
}

impl Lyrics {
    pub fn new() -> Self {
        Self { 
            lines: Vec::new() 
        }
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }
}

impl Line {
    pub fn new(begin: String, end: String) -> Self {
        Self {
            begin,
            end,
            chunks: Vec::new() 
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
}

impl Chunk {
    pub fn new(begin: String, end: String, text: String) -> Self {
        Self {
            begin,
            end,
            text
        }
    }
}
