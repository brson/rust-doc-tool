use url::Url;

#[derive(Debug)]
pub struct Document {
    pub meta: Meta,
    pub doc: Body,
}

#[derive(Debug)]
pub struct Meta {
    origin_url: Url,
}

#[derive(Debug)]
pub struct Body {
    blocks: Vec<Block>,
}

#[derive(Debug)]
pub enum Block {
    Header(Header),
    Paragraph(Paragraph),
}

#[derive(Debug)]
pub struct Header {
    text: String,
}

#[derive(Debug)]
pub struct Paragraph {
    spans: Vec<Inline>,
}

#[derive(Debug)]
pub enum Inline {
    Text(String),
    Bold(Box<Inline>),
    Italic(Box<Inline>),
    Code(Box<Inline>),
}
