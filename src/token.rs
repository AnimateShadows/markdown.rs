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

impl TokenType {
    fn to_html_tag(&self) -> (String, bool) {
        let tag: (&str, bool) = match self {
            Self::Bold => ("strong", true),
            Self::Italic => ("em", true),
            Self::BulletPointItem => ("li", true),
            Self::NumberedListItem => ("li", true),
            Self::Text => ("p", true),
            Self::BlockQuote => ("blockquote", true),
            Self::CodeBlock => ("pre", true),
            Self::HorizontalLine => ("hr", false),
            Self::InlineCode => ("code", true),
            Self::HyperLinkImage => ("img src=\"{url}\" alt=\"{alt}\"", false),
            Self::HyperLink => ("a href=\"{url}\"", false)
        };
        tag.
        
        //tag.0 = String::from(tag.0);

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
    content: String,
    type_: TokenType,
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
