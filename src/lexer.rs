use std::sync::mpsc::Sender;

use crate::lexer::Token::{Identifier, Let};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    token_sender: Sender<Token<'a>>,
}

#[derive(Debug)]
pub enum Token<'a> {
    OpenParen(usize),
    CloseParen(usize),
    Identifier(&'a str, usize),
    Let(usize),
    Equal(usize),
}

// Function pointer definition must be wrapped in a struct to be recursive
struct StateFunction(fn(&mut Lexer) -> Option<StateFunction>);

impl<'a> Lexer<'a> {
    pub fn begin_lexing(s: &'a str, sender: Sender<Token<'a>>) {
        let mut lexer = Lexer::<'a> {
            input: s,
            pos: 0,
            token_sender: sender,
        };

        lexer.run()
    }

    fn run(&mut self) {
        let mut state = Some(StateFunction(Lexer::determine_token));

        while let Some(next_state) = state {
            state = next_state.0(self)
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.pos >= self.input.len() {
            None
        } else {
            let c = self.input[self.pos..].chars().next().unwrap();
            self.pos += 1;
            Some(c)
        }
    }

    fn emit(&mut self, token: Token<'a>) {
        println!("Sending token on channel: {:?}", token);
        self.token_sender
            .send(token)
            .expect("Unable to send token");
    }

    fn determine_token(l: &mut Lexer) -> Option<StateFunction> {
        while let Some(c) = l.next() {
            if Lexer::is_white_space(c) {
                //ignore
                continue;
            }
            if c == 'l' {
                return Some(StateFunction(Lexer::lex_let_or_identifier));
            }
        }
        return None;
    }

    fn lex_let_or_identifier(&mut self) -> Option<StateFunction> {
        let mut s = "";
        while let Some(c) = self.next() {
            if Lexer::is_white_space(c) {
                break;
            }
            s + c;
        }
        let token = if s == "let" {
            Let(self.pos)
        } else {
            Identifier(s, self.pos)
        };

        self.emit(token);
        return Some(StateFunction(Lexer::determine_token));
    }
    fn is_white_space(ch: char) -> bool {
        ch == ' ' || ch == '\n' || ch == '\t' || ch == '\r'
    }
}
