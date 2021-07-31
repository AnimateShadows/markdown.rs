use crate::lexer::lex;
use crate::token::Token;

fn extract_whitespace(string: &String) -> String {
    let mut ret = String::new();
    for chr in string.chars() {
        if !chr.is_whitespace() {
            ret.push(chr);
        }
    }
    ret
}

pub fn transform(tokens: Vec<Token>) -> String {
    let mut ret = String::new();
    for token in tokens.iter() {
        let tag = token.type_.to_html_tag();
        if extract_whitespace(&token.content).is_empty() {
            continue;
        }

        if &tag.0 == &String::from("") {
            ret.push_str(&format!("{content}", content = token.content).to_owned());
        } else {
            ret.push_str(
                &format!(
                    "<{tag}>{content}</{tag}>",
                    tag = tag.0,
                    content = token.content
                )
                .to_owned(),
            );
        }
    }
    println!("<p>{}</p>", ret);
    ret
}
