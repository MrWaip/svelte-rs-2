use svelte_span::Span;

static CSS_WS: [bool; 256] = {
    let mut t = [false; 256];
    t[0x20] = true;
    t[0x09] = true;
    t[0x0A] = true;
    t[0x0D] = true;
    t[0x0C] = true;
    t
};

#[inline(always)]
fn is_css_ws(b: u8) -> bool {
    CSS_WS[b as usize]
}

#[inline(always)]
fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b >= 0x80
}

#[inline(always)]
fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b >= 0x80
}

#[inline(always)]
fn utf8_char_len(b: u8) -> usize {
    if b < 0x80 {
        1
    } else if b < 0xE0 {
        2
    } else if b < 0xF0 {
        3
    } else {
        4
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Ident,

    AtKeyword,

    Hash,

    String,

    Number,

    Percentage,

    Dimension,

    Whitespace,

    Comment,

    Cdo,

    Cdc,

    Colon,

    Semicolon,

    Comma,

    LBrace,

    RBrace,

    LBracket,

    RBracket,

    LParen,

    RParen,

    Delim(u8),

    Eof,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    #[inline(always)]
    const fn new(kind: TokenKind, start: u32, end: u32) -> Self {
        Self {
            kind,
            span: Span::new(start, end),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ScannerCheckpoint {
    pos: usize,
    prev_end: u32,
}

pub(crate) struct Scanner<'src> {
    src: &'src str,
    tokens: Vec<Token>,
    pos: usize,

    pub(crate) prev_end: u32,
}

impl<'src> Scanner<'src> {
    pub fn new(src: &'src str) -> Self {
        let tokens = tokenize(src);
        Self {
            src,
            tokens,
            pos: 0,
            prev_end: 0,
        }
    }

    #[inline(always)]
    pub fn peek(&self) -> Token {
        self.tokens[self.pos]
    }

    #[inline(always)]
    pub fn peek_n(&self, offset: usize) -> Token {
        let idx = self.pos + offset;
        if idx < self.tokens.len() {
            self.tokens[idx]
        } else {
            *self
                .tokens
                .last()
                .expect("scanner always emits an Eof token at the end of tokens")
        }
    }

    #[inline(always)]
    pub fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos];
        if tok.kind != TokenKind::Eof {
            self.prev_end = tok.span.end;
            self.pos += 1;
        }
        tok
    }

    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        self.tokens[self.pos].kind == TokenKind::Eof
    }

    #[inline(always)]
    pub fn is_at(&self, kind: TokenKind) -> bool {
        self.tokens[self.pos].kind == kind
    }

    #[inline(always)]
    pub fn is_at_delim(&self, ch: u8) -> bool {
        self.tokens[self.pos].kind == TokenKind::Delim(ch)
    }

    #[inline(always)]
    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.is_at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn eat_delim(&mut self, ch: u8) -> bool {
        if self.is_at_delim(ch) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.is_at(TokenKind::Whitespace) {
            self.advance();
        }
    }

    pub fn skip_whitespace_and_comments(&mut self) {
        while let TokenKind::Whitespace | TokenKind::Comment | TokenKind::Cdo | TokenKind::Cdc =
            self.peek().kind
        {
            self.advance();
        }
    }

    #[inline(always)]
    pub fn current_start(&self) -> u32 {
        self.tokens[self.pos].span.start
    }

    #[inline(always)]
    pub fn span_at(&self) -> Span {
        let s = self.current_start();
        Span::new(s, s)
    }

    #[inline(always)]
    pub fn span_from(&self, start: u32) -> Span {
        Span::new(start, self.prev_end)
    }

    #[inline]
    pub fn source_text(&self, span: Span) -> &'src str {
        &self.src[span.start as usize..span.end as usize]
    }

    #[inline]
    pub fn current_text(&self) -> &'src str {
        let sp = self.tokens[self.pos].span;
        &self.src[sp.start as usize..sp.end as usize]
    }

    #[inline(always)]
    pub fn save(&self) -> ScannerCheckpoint {
        ScannerCheckpoint {
            pos: self.pos,
            prev_end: self.prev_end,
        }
    }

    #[inline(always)]
    pub fn restore(&mut self, cp: ScannerCheckpoint) {
        self.pos = cp.pos;
        self.prev_end = cp.prev_end;
    }
}

fn tokenize(src: &str) -> Vec<Token> {
    let bytes = src.as_bytes();
    let len = bytes.len();

    let mut tokens = Vec::with_capacity(len / 4 + 1);
    let mut pos: usize = 0;

    while pos < len {
        let start = pos as u32;
        let b = bytes[pos];

        if is_css_ws(b) {
            pos += 1;
            while pos < len && is_css_ws(bytes[pos]) {
                pos += 1;
            }
            tokens.push(Token::new(TokenKind::Whitespace, start, pos as u32));
            continue;
        }

        match b {
            b'"' | b'\'' => {
                pos += 1;
                scan_string_tail(bytes, &mut pos, b);
                tokens.push(Token::new(TokenKind::String, start, pos as u32));
            }

            b'/' => {
                if pos + 1 < len && bytes[pos + 1] == b'*' {
                    pos += 2;
                    scan_comment_tail(bytes, &mut pos);
                    tokens.push(Token::new(TokenKind::Comment, start, pos as u32));
                } else {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delim(b'/'), start, pos as u32));
                }
            }

            b'<' => {
                if pos + 3 < len
                    && bytes[pos + 1] == b'!'
                    && bytes[pos + 2] == b'-'
                    && bytes[pos + 3] == b'-'
                {
                    pos += 4;
                    scan_html_comment_tail(bytes, &mut pos);
                    tokens.push(Token::new(TokenKind::Cdo, start, pos as u32));
                } else {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delim(b'<'), start, pos as u32));
                }
            }

            b'@' => {
                pos += 1;
                if pos < len
                    && (is_ident_start(bytes[pos]) || bytes[pos] == b'-' || bytes[pos] == b'\\')
                {
                    consume_ident(bytes, &mut pos);
                    tokens.push(Token::new(TokenKind::AtKeyword, start, pos as u32));
                } else {
                    tokens.push(Token::new(TokenKind::Delim(b'@'), start, pos as u32));
                }
            }

            b'#' => {
                pos += 1;
                if pos < len && (is_ident_char(bytes[pos]) || bytes[pos] == b'\\') {
                    consume_ident(bytes, &mut pos);
                    tokens.push(Token::new(TokenKind::Hash, start, pos as u32));
                } else {
                    tokens.push(Token::new(TokenKind::Delim(b'#'), start, pos as u32));
                }
            }

            b'0'..=b'9' => {
                consume_number(bytes, &mut pos);
                let kind = classify_after_number(bytes, &mut pos);
                tokens.push(Token::new(kind, start, pos as u32));
            }

            b'.' => {
                if pos + 1 < len && bytes[pos + 1].is_ascii_digit() {
                    consume_number(bytes, &mut pos);
                    let kind = classify_after_number(bytes, &mut pos);
                    tokens.push(Token::new(kind, start, pos as u32));
                } else {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delim(b'.'), start, pos as u32));
                }
            }

            b'+' => {
                if starts_number_after_sign(bytes, pos) {
                    consume_number(bytes, &mut pos);
                    let kind = classify_after_number(bytes, &mut pos);
                    tokens.push(Token::new(kind, start, pos as u32));
                } else {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delim(b'+'), start, pos as u32));
                }
            }

            b'-' => {
                if pos + 2 < len && bytes[pos + 1] == b'-' && bytes[pos + 2] == b'>' {
                    pos += 3;
                    tokens.push(Token::new(TokenKind::Cdc, start, pos as u32));
                } else if starts_number_after_sign(bytes, pos) {
                    consume_number(bytes, &mut pos);
                    let kind = classify_after_number(bytes, &mut pos);
                    tokens.push(Token::new(kind, start, pos as u32));
                } else if pos + 1 < len
                    && (is_ident_start(bytes[pos + 1])
                        || bytes[pos + 1] == b'-'
                        || bytes[pos + 1] == b'\\')
                {
                    consume_ident(bytes, &mut pos);
                    tokens.push(Token::new(TokenKind::Ident, start, pos as u32));
                } else {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delim(b'-'), start, pos as u32));
                }
            }

            b'a'..=b'z' | b'A'..=b'Z' | b'_' | 0x80..=0xFF => {
                consume_ident(bytes, &mut pos);
                tokens.push(Token::new(TokenKind::Ident, start, pos as u32));
            }

            b'\\' => {
                if pos + 1 < len && !is_css_ws(bytes[pos + 1]) {
                    consume_ident(bytes, &mut pos);
                    tokens.push(Token::new(TokenKind::Ident, start, pos as u32));
                } else {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delim(b'\\'), start, pos as u32));
                }
            }

            b':' => {
                pos += 1;
                tokens.push(Token::new(TokenKind::Colon, start, pos as u32));
            }
            b';' => {
                pos += 1;
                tokens.push(Token::new(TokenKind::Semicolon, start, pos as u32));
            }
            b',' => {
                pos += 1;
                tokens.push(Token::new(TokenKind::Comma, start, pos as u32));
            }
            b'{' => {
                pos += 1;
                tokens.push(Token::new(TokenKind::LBrace, start, pos as u32));
            }
            b'}' => {
                pos += 1;
                tokens.push(Token::new(TokenKind::RBrace, start, pos as u32));
            }
            b'[' => {
                pos += 1;
                tokens.push(Token::new(TokenKind::LBracket, start, pos as u32));
            }
            b']' => {
                pos += 1;
                tokens.push(Token::new(TokenKind::RBracket, start, pos as u32));
            }
            b'(' => {
                pos += 1;
                tokens.push(Token::new(TokenKind::LParen, start, pos as u32));
            }
            b')' => {
                pos += 1;
                tokens.push(Token::new(TokenKind::RParen, start, pos as u32));
            }

            _ => {
                pos += 1;
                tokens.push(Token::new(TokenKind::Delim(b), start, pos as u32));
            }
        }
    }

    tokens.push(Token::new(TokenKind::Eof, len as u32, len as u32));
    tokens
}

#[inline]
fn starts_number_after_sign(bytes: &[u8], pos: usize) -> bool {
    let len = bytes.len();
    if pos + 1 >= len {
        return false;
    }
    let next = bytes[pos + 1];
    if next.is_ascii_digit() {
        return true;
    }
    next == b'.' && pos + 2 < len && bytes[pos + 2].is_ascii_digit()
}

#[inline]
fn consume_ident(bytes: &[u8], pos: &mut usize) {
    let len = bytes.len();
    while *pos < len {
        let b = bytes[*pos];
        if b == b'\\' {
            *pos += 1;
            if *pos < len {
                consume_escape(bytes, pos);
            }
        } else if is_ident_char(b) {
            *pos += utf8_char_len(b);
        } else {
            break;
        }
    }
}

#[inline]
fn consume_escape(bytes: &[u8], pos: &mut usize) {
    if bytes[*pos].is_ascii_hexdigit() {
        let hex_start = *pos;
        while *pos < bytes.len() && *pos - hex_start < 6 && bytes[*pos].is_ascii_hexdigit() {
            *pos += 1;
        }

        if *pos < bytes.len() && is_css_ws(bytes[*pos]) {
            *pos += 1;
        }
    } else {
        *pos += utf8_char_len(bytes[*pos]);
    }
}

#[inline]
fn consume_number(bytes: &[u8], pos: &mut usize) {
    let len = bytes.len();

    if *pos < len && (bytes[*pos] == b'+' || bytes[*pos] == b'-') {
        *pos += 1;
    }

    while *pos < len && bytes[*pos].is_ascii_digit() {
        *pos += 1;
    }

    if *pos < len && bytes[*pos] == b'.' && *pos + 1 < len && bytes[*pos + 1].is_ascii_digit() {
        *pos += 1;
        while *pos < len && bytes[*pos].is_ascii_digit() {
            *pos += 1;
        }
    }
}

#[inline]
fn classify_after_number(bytes: &[u8], pos: &mut usize) -> TokenKind {
    let len = bytes.len();
    if *pos < len && bytes[*pos] == b'%' {
        *pos += 1;
        TokenKind::Percentage
    } else if *pos < len
        && (is_ident_start(bytes[*pos]) || bytes[*pos] == b'-' || bytes[*pos] == b'\\')
    {
        consume_ident(bytes, pos);
        TokenKind::Dimension
    } else {
        TokenKind::Number
    }
}

fn scan_string_tail(bytes: &[u8], pos: &mut usize, quote: u8) {
    let len = bytes.len();
    while *pos < len {
        let b = bytes[*pos];
        if b == b'\\' && *pos + 1 < len {
            *pos += 2;
        } else if b == quote {
            *pos += 1;
            return;
        } else {
            *pos += 1;
        }
    }
}

fn scan_html_comment_tail(bytes: &[u8], pos: &mut usize) {
    let len = bytes.len();
    while *pos + 2 < len {
        if bytes[*pos] == b'-' && bytes[*pos + 1] == b'-' && bytes[*pos + 2] == b'>' {
            *pos += 3;
            return;
        }
        *pos += 1;
    }

    *pos = len;
}

fn scan_comment_tail(bytes: &[u8], pos: &mut usize) {
    let len = bytes.len();
    while *pos + 1 < len {
        if bytes[*pos] == b'*' && bytes[*pos + 1] == b'/' {
            *pos += 2;
            return;
        }
        *pos += 1;
    }

    if *pos < len {
        *pos += 1;
    }
}
