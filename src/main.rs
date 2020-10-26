use std::convert::TryInto;
use std::sync::mpsc::channel;

use std::sync::mpsc::*;
use crate::lexer::Lexer;

mod lexer;

fn main() {
    let code = "let a = 3\
    print(a)\
    a + 2\
    ";

    rayon::scope(|s| {
        let (sender, receiver) = channel();
        s.spawn(|_| {
            Lexer::begin_lexing(&code, sender)
        });
        while let Ok(token) = receiver.recv() {
            println!("Token received from channel: {:?}", token)
        }
    })
}
