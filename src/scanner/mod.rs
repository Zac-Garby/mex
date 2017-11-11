pub mod token;

pub struct Scanner {
    chars: Vec<char>,
    next_index: usize,
}

impl Scanner {
    #[allow(dead_code)]
    pub fn new(s: String) -> Scanner {
        let chars = s.chars().collect::<Vec<char>>();
        let next_index = 0;

        Scanner{chars, next_index}
    }

    fn advance(&mut self) -> Option<char> {
        if self.next_index == 0 || self.next_index >= self.chars.len() + 1 {
            self.next_index += 1;
            return None
        }
    
        let ch = self.chars[self.next_index - 1];
        self.next_index += 1;

        Some(ch)
    }

    fn peek(&mut self) -> Option<char> {
        if self.next_index >= self.chars.len() {
            return None
        }

        Some(self.chars[self.next_index])
    }

    fn take_while<T>(&mut self, mut pred: T) -> String
        where T: FnMut(char) -> bool
    {
        let mut out: String = String::new();

        loop {
            if let Some(next) = self.peek() {
                if !pred(next) {
                    break
                }

                out.push(next);

                self.next_index += 1;
            } else {
                break
            }
        }

        out
    }
}

impl Iterator for Scanner {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (literal, t, adv) = match self.peek() {
            Some('+') => ("+".to_string(), token::Type::Plus, true),
            Some('-') => ("-".to_string(), token::Type::Minus, true),
            Some('*') => ("*".to_string(), token::Type::Multiply, true),
            Some('/') => ("/".to_string(), token::Type::Divide, true),
            Some('(') => ("(".to_string(), token::Type::LeftParen, true),
            Some(')') => (")".to_string(), token::Type::RightParen, true),
            Some('=') => ("=".to_string(), token::Type::Equals, true),

            Some(c) if c.is_whitespace() => {
                self.take_while(|x| x.is_whitespace());
                return self.next();
            }

            Some(c) if c.is_digit(10) => {
                let mut has_dot = false;

                let literal = self.take_while(|x| {
                    if x == '.' && !has_dot {
                        has_dot = true;
                        true
                    } else {
                        x.is_digit(10)
                    }
                });

                (literal, token::Type::Number, false)
            }

            Some(c) if c.is_alphabetic() => {
                let literal = self.take_while(|x| x.is_alphanumeric());
                (literal, token::Type::Identifier, false)
            }

            Some(c) => (c.to_string(), token::Type::Illegal, true),
            None => return None,
        };

        if adv {
            self.advance();
        }

        Some(token::Token{literal, t})
    }
}