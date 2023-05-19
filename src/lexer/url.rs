use logos::{Lexer, Logos, Source};
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
    // TODO: Capture link definitions
    //              Link-URL Link-Text trust
    //              ________ ______________
    // Bei URL maybe noch "name=..." dabei
    #[regex(r#"href=("(.*)")>[a-zA-Z\s\-.]*<"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    // TODO: Ignore all characters that do not belong to a link definition
    #[regex(r#"[^(href=(["'])(.*)(["'])>[a-zA-Z\s\-.]*<)]"#, logos::skip)]
    Ignored,

    // Catch any error
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let slice: &str = lex.slice();

    let mut url: String = String::new();
    let mut text: String = String::new();

    for c in slice.chars() {
        let mut append_next: bool = false;
        if c == '"' && append_next == false {
            append_next = true;
            continue;
        }
        else if c == '"' && append_next == true {
            append_next = false;
            continue;
        } else {
            if append_next {
                url.push(c);
            }
            continue;
        }
    }
    
    (LinkUrl(url), LinkText(text))
}
