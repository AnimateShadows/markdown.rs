use crate::token;

#[allow(dead_code)]
struct Stream {
    data: String,
    current: usize,
    limit: usize,
}

#[allow(dead_code)]
impl Stream {
    fn shift(&mut self) -> String {
        if self.current == self.limit {
            return String::from("");
        }
        let ret: &str = &self.data[self.current..self.current + 1];
        self.current += 1;
        String::from(ret)
    }

    fn peek(&self, amount: usize) -> String {
        let amount: usize = amount + self.current;
        if amount >= self.limit {
            return String::from("");
        }
        String::from(&self.data[amount..amount + 1])
    }

    fn new(data: String) -> Stream {
        let len: usize = data.len();
        Stream {
            data: data,
            current: 0,
            limit: len,
        }
    }
}

fn read_until(stream: &mut Stream, identifier: &str, limit: usize) -> String {
    let mut ret: String = String::from("");
    let mut times: usize = 0;
    for _i in 0..limit - 1 {
        let _ = stream.shift();
    }
    loop {
        let shifted: String = stream.shift();
        if shifted == identifier {
            times += 1;
        }
        if times >= limit {
            break;
        }
        if shifted == "" {
            break;
        }
        ret = format!("{}{}", ret, shifted)
    }
    String::from(&ret[..ret.len() - (limit - 1)])
}

fn read_bare(stream: &mut Stream, start: String) -> String {
    let mut ret: String = start.clone();

    loop {
        let peek: String = stream.peek(1);
        if peek == "*" || peek == "_" {
            break;
        }
        let shifted: String = stream.shift();
        if shifted == "" {
            break;
        }
        ret = format!("{}{}", ret, shifted);
    }
    ret
}

pub fn lex(data: String) {
    let mut stream: Stream = Stream::new(data);
    let mut tokens: Vec<token::Token> = Vec::new();
    loop {
        let peek = stream.peek(1);
        let shifted: String = stream.shift();
        if shifted == "" {
            break;
        }

        if shifted == "*" || shifted == "_" {
            if peek == shifted {
                tokens.push(token::Token::new(
                    read_until(&mut stream, &shifted.to_owned(), 2),
                    token::TokenType::Bold,
                ))
            } else {
                tokens.push(token::Token::new(
                    read_until(&mut stream, &shifted.to_owned(), 1),
                    token::TokenType::Italic,
                ))
            }
        } else {
            tokens.push(token::Token::new(
                read_bare(&mut stream, shifted),
                token::TokenType::Text,
            ))
        }
    }

    for t in tokens.iter() {
        print!("{} ", t);
    }
}
