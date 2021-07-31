use std::fmt;

#[allow(dead_code)]
pub enum TokenType {
    Text,
    Bold,
    Italic,
    BulletPointItem,
    NumberedListItem,
    HyperLink,
    HyperLinkImage,
    BlockQuote,
    HorizontalLine,
    CodeBlock,
    InlineCode,
}

#[allow(dead_code)]
impl TokenType {
    pub fn to_html_tag(&self) -> (String, bool) {
        let tag: (String, bool) = match self {
            Self::Bold => (String::from("strong"), true),
            Self::Italic => (String::from("em"), true),
            Self::BulletPointItem => (String::from("li"), true),
            Self::NumberedListItem => (String::from("li"), true),
            Self::Text => (String::from(""), true),
            Self::BlockQuote => (String::from("blockquote"), true),
            Self::CodeBlock => (String::from("pre"), true),
            Self::HorizontalLine => (String::from("hr"), false),
            Self::InlineCode => (String::from("code"), true),
            Self::HyperLinkImage => (String::from("img src=\"{url}\" alt=\"{alt}\""), false),
            Self::HyperLink => (String::from("a href=\"{url}\""), false),
        };
        tag
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: &str = match self {
            Self::Bold => "Bold",
            Self::Italic => "Italic",
            Self::BulletPointItem => "BulletPointItem",
            Self::NumberedListItem => "NumberedListItem",
            Self::Text => "Text",
            Self::HyperLink => "HyperLink",
            Self::HyperLinkImage => "HyperLinkImage",
            Self::BlockQuote => "BlockQuote",
            Self::CodeBlock => "CodeBlock",
            Self::HorizontalLine => "HorizontalLine",
            Self::InlineCode => "InlineCode",
        };
        write!(f, "TokenType<{}>", name)
    }
}

#[allow(dead_code)]
pub struct Token {
    pub content: String,
    pub type_: TokenType,
}

impl Token {
    pub fn new(content: String, type_: TokenType) -> Token {
        Token {
            content: content,
            type_: type_,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token(\"{}\" type: {})", self.content, self.type_)
    }
}
