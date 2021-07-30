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
