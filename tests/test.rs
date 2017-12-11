extern crate lexer;


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


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<MyToken> for WhitespaceReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }

    fn read(&self, input: &mut Input, current: &State, next: &mut State) -> Option<MyToken> {
        match input.read(next) {
            Some(ch) => if ch.is_whitespace() {
                let mut string = String::new();

                string.push(ch);

                while !input.done(next) {
                    if let Some(ch) = input.peek(next, 0) {
                        if ch.is_whitespace() {
                            input.read(next);
                            string.push(ch);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Whitespace,
                    TokenValue::Str(string)
                ))
            } else {
                None
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<MyToken> for IdentifierReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(&self, input: &mut Input, current: &State, next: &mut State) -> Option<MyToken> {
        match input.read(next) {
            Some(ch) => if ch.is_alphabetic() {
                let mut string = String::new();

                string.push(ch);

                while !input.done(next) {
                    if let Some(ch) = input.peek(next, 0) {
                        if ch.is_alphanumeric() {
                            input.read(next);
                            string.push(ch);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Identifier,
                    TokenValue::Str(string)
                ))
            } else {
                None
            },
            None => None,
        }
    }
}


#[test]
fn test_lexer_whitespace() {
    let readers = ReadersBuilder::new()
        .add(WhitespaceReader)
        .add(IdentifierReader)
        .build();

    let chars = "   \n\t   ".chars().collect::<Vec<char>>();
    let lexer = readers.lexer(chars);
    let tokens: Vec<MyToken> = lexer.collect();

    assert_eq!(tokens.len(), 1);

    let ws_token = &tokens[0];
    assert_eq!(ws_token.kind(), &TokenKind::Whitespace);
    assert_eq!(ws_token.meta().col_start(), 1);
    assert_eq!(ws_token.meta().col_end(), 5);
    assert_eq!(ws_token.meta().col_count(), 5);
    assert_eq!(ws_token.meta().line_start(), 1);
    assert_eq!(ws_token.meta().line_end(), 2);
    assert_eq!(ws_token.meta().line_count(), 2);
    assert_eq!(ws_token.meta().len(), 8);
    if let &TokenValue::Str(ref string) = ws_token.value() {
        assert_eq!(string.len(), 8);
    }
}

#[test]
fn test_lexer_identifier() {
    let readers = ReadersBuilder::new()
        .add(WhitespaceReader)
        .add(IdentifierReader)
        .build();

    let chars = Chars::from(::std::fs::File::open("tests/file.txt").unwrap());
    let lexer = readers.lexer(chars);
    let tokens: Vec<MyToken> = lexer.collect();

    assert_eq!(tokens.len(), 4);

    let ident_token = &tokens[0];
    assert_eq!(ident_token.kind(), &TokenKind::Identifier);
    assert_eq!(ident_token.meta().col_start(), 1);
    assert_eq!(ident_token.meta().col_end(), 3);
    assert_eq!(ident_token.meta().col_count(), 3);
    assert_eq!(ident_token.meta().line_start(), 1);
    assert_eq!(ident_token.meta().line_end(), 1);
    assert_eq!(ident_token.meta().line_count(), 1);
    assert_eq!(ident_token.meta().len(), 3);
    if let &TokenValue::Str(ref string) = ident_token.value() {
        assert_eq!(string, "def");
        assert_eq!(string.len(), 3);
    }
    assert_eq!(tokens[1].kind(), &TokenKind::Whitespace);
    assert_eq!(tokens[2].kind(), &TokenKind::Identifier);
    assert_eq!(tokens[3].kind(), &TokenKind::Whitespace);
}
