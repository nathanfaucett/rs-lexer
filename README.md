lexer
=====

plugin based lexical reader

```rust
extern crate lexer;

use lexer::{Input, Reader, ReaderResult, ReadersBuilder, State, Token, TokenError, TokenMeta};

#[derive(Debug)]
pub enum TokenValue {
    Whitespace(String),
    Identifier(String),
}

pub type MyToken = Token<TokenValue>;
pub type MyError = TokenError<&'static str>;

pub struct WhitespaceReader;

impl Reader<MyToken, MyError, ()> for WhitespaceReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(
        &self,
        input: &mut Input,
        current: &State,
        next: &mut State,
        _: &mut (),
    ) -> ReaderResult<MyToken, MyError> {
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
                    TokenValue::Whitespace(string),
                ))
            } else {
                ReaderResult::None
            },
            None => ReaderResult::None,
        }
    }
}

pub struct IdentifierReader;

impl Reader<MyToken, MyError, ()> for IdentifierReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }

    fn read(
        &self,
        input: &mut Input,
        current: &State,
        next: &mut State,
        _: &mut (),
    ) -> ReaderResult<MyToken, MyError> {
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
                    TokenValue::Identifier(string),
                ))
            } else {
                ReaderResult::None
            },
            None => ReaderResult::None,
        }
    }
}

fn main() {
    let readers = ReadersBuilder::new()
        .add(WhitespaceReader)
        .add(IdentifierReader)
        .build();

    let mut data = ();
    let lexer = readers.lexer("Hello world\n".chars(), &mut data);
    let tokens: Vec<MyToken> = lexer.map(|t| t.unwrap()).collect();

    println!("{:#?}", tokens);
}
```
