use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "tt")]
pub struct SynedLyric {
    pub body: Body,
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub div: Vec<Div>,
}

#[derive(Serialize, Deserialize)]
pub struct Div {
    pub p: Vec<P>,
}

#[derive(Serialize, Deserialize)]
pub struct P {
    #[serde(rename = "@begin")]
    pub begin: String,
    #[serde(rename = "@end")]
    pub end: String,
    pub span: Vec<Span>,
}

#[derive(Serialize, Deserialize)]
pub struct Span {
    #[serde(rename = "@begin")]
    pub begin: String,
    #[serde(rename = "@end")]
    pub end: String,
    #[serde(rename = "$text")]
    pub word: String,
}
