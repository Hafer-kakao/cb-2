use logos::{Lexer, Logos, Filter};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    #[regex(r#"<a\s+(\s*[^">]*"[^"]*")*>[^<>]*</a\s*>"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex(r#"\s*<([^a]|a[^\s>])(\s*[^">]*("[^"]*")?)*\s*>[^<>]*"#, logos::skip)]
    Ignored,

    // Catch any error
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> Filter<(LinkUrl, LinkText)> {
    match extract_link_info_option(lex) {
        Some(result) => Filter::Emit(result),
        None => Filter::Skip
    }
}

fn extract_link_info_option(lex: &mut Lexer<URLToken>) -> Option<(LinkUrl, LinkText)> {
    let mut slice: &str = lex.slice();

    // extract url
    let href_i = slice.find("href")?;

    slice = &slice[href_i+4..];

    let url_begin = slice.find('\"')?;

    slice = &slice[url_begin+1..];

    let url_end = slice.find('\"')?;
    
    let url = String::from(&slice[..url_end]);


    // extract text
    let text_end = slice.rfind('<')?;

    slice = &slice[url_end..text_end];

    let text_begin = slice.rfind('>')?;

    let text = String::from(&slice[text_begin+1..]);

    Some((LinkUrl(url), LinkText(text)))
}