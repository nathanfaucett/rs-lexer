#![feature(test)]


extern crate test;

extern crate lexer;


use test::Bencher;

use lexer::*;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenKind {
    Whitespace,
    Identifier,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenValue {
    Chr(char),
    Str(String),
}

pub type MyToken = Token<TokenKind, TokenValue>;
pub type MyError = TokenError<&'static str>;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<MyToken, MyError> for WhitespaceReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }

    fn read(&self, input: &mut Input, current: &State, next: &mut State) -> ReaderResult<MyToken, MyError> {
        match input.read(next) {
            Some(ch) => if ch.is_whitespace() {
                let mut string = String::new();

                string.push(ch);

                while let Some(ch) = input.peek(next, 0) {
                    if ch.is_whitespace() {
                        input.read(next);
                        string.push(ch);
                    } else {
                        break;
                    }
                }

                ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Whitespace,
                    TokenValue::Str(string)
                ))
            } else {
                ReaderResult::None
            },
            None => ReaderResult::None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<MyToken, MyError> for IdentifierReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(&self, input: &mut Input, current: &State, next: &mut State) -> ReaderResult<MyToken, MyError> {
        match input.read(next) {
            Some(ch) => if ch.is_alphabetic() {
                let mut string = String::new();

                string.push(ch);

                while let Some(ch) = input.peek(next, 0) {
                    if ch.is_alphanumeric() {
                        input.read(next);
                        string.push(ch);
                    } else {
                        break;
                    }
                }

                ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Identifier,
                    TokenValue::Str(string)
                ))
            } else {
                ReaderResult::None
            },
            None => ReaderResult::None,
        }
    }
}


#[bench]
fn bench_lexer(b: &mut Bencher) {
    let readers = ReadersBuilder::new()
        .add(WhitespaceReader)
        .add(IdentifierReader)
        .build();

    let vec = " def  \n\t   name ".chars().collect::<Vec<char>>();

    b.iter(|| {
        let lexer = readers.lexer(&vec);
        let _: Vec<MyToken> = lexer.map(|t| t.unwrap()).collect();
    });
}

#[bench]
fn bench_lexer_full(b: &mut Bencher) {
    b.iter(|| {
        let readers = ReadersBuilder::new()
            .add(WhitespaceReader)
            .add(IdentifierReader)
            .build();

        let vec = " def  \n\t   name ".chars().collect::<Vec<char>>();
        let lexer = readers.lexer(&vec);
        let _: Vec<MyToken> = lexer.map(|t| t.unwrap()).collect();
    });
}
