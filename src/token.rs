use std::fmt;

#[allow(dead_code)]
pub enum TokenType {
    Bold,
    Italic,
    Strikethrough,
    Underline,
    BulletPointItem,
    NumberedListItem,
    Text,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: &str = match self {
            Self::Bold => "Bold",
            Self::Italic => "Italic",
            Self::Strikethrough => "StrikeThrough",
            Self::Underline => "Underline",
            Self::BulletPointItem => "BulletPointItem",
            Self::NumberedListItem => "NumberedListItem",
            Self::Text => "Text",
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
