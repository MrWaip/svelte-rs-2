use svelte_span::Span;

// ---------------------------------------------------------------------------
// CSS whitespace lookup (space, tab, newline, carriage return, form feed)
// ---------------------------------------------------------------------------

static CSS_WS: [bool; 256] = {
    let mut t = [false; 256];
    t[0x20] = true; // space
    t[0x09] = true; // tab
    t[0x0A] = true; // LF
    t[0x0D] = true; // CR
    t[0x0C] = true; // FF
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

/// Byte length of a UTF-8 character from its leading byte.
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

// ---------------------------------------------------------------------------
// Token types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    /// CSS identifier (e.g. `div`, `color`, `-webkit-`, `--custom`)
    Ident,
    /// `@` followed by an identifier (e.g. `@media`, `@keyframes`)
    AtKeyword,
    /// `#` followed by ident chars (e.g. `#foo`)
    Hash,
    /// Quoted string: `"..."` or `'...'`
    String,
    /// Numeric literal: `123`, `1.5`
    Number,
    /// Number followed by `%` (e.g. `50%`)
    Percentage,
    /// Number followed by an identifier (e.g. `10px`, `2n`)
    Dimension,
    /// One or more CSS whitespace characters
    Whitespace,
    /// CSS comment `/* ... */`
    Comment,
    /// HTML comment `<!-- ... -->` (consumed as a single token).
    Cdo,
    /// `-->`
    Cdc,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    /// `,`
    Comma,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// Single-character delimiter not matched by any other token.
    Delim(u8),
    /// End of input (always the last token).
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

// ---------------------------------------------------------------------------
// Scanner checkpoint (for save/restore backtracking)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(crate) struct ScannerCheckpoint {
    pos: usize,
    prev_end: u32,
}

// ---------------------------------------------------------------------------
// Scanner
// ---------------------------------------------------------------------------

pub(crate) struct Scanner<'src> {
    src: &'src str,
    tokens: Box<[Token]>,
    pos: usize,
    /// Byte offset of the end of the last consumed token.
    pub(crate) prev_end: u32,
}

impl<'src> Scanner<'src> {
    pub fn new(src: &'src str) -> Self {
        let tokens = tokenize(src).into_boxed_slice();
        Self {
            src,
            tokens,
            pos: 0,
            prev_end: 0,
        }
    }

    // -- navigation ---------------------------------------------------------

    #[inline(always)]
    pub fn peek(&self) -> Token {
        self.tokens[self.pos]
    }

    #[inline(always)]
    pub fn peek_at(&self, offset: usize) -> Token {
        let idx = self.pos + offset;
        if idx < self.tokens.len() {
            self.tokens[idx]
        } else {
            *self.tokens.last().unwrap() // Eof
        }
    }

    #[inline(always)]
    pub fn bump(&mut self) -> Token {
        let tok = self.tokens[self.pos];
        if tok.kind != TokenKind::Eof {
            self.prev_end = tok.span.end;
            self.pos += 1;
        }
        tok
    }

    #[inline(always)]
    pub fn at_end(&self) -> bool {
        self.tokens[self.pos].kind == TokenKind::Eof
    }

    // -- matching -----------------------------------------------------------

    #[inline(always)]
    pub fn at(&self, kind: TokenKind) -> bool {
        self.tokens[self.pos].kind == kind
    }

    #[inline(always)]
    pub fn at_delim(&self, ch: u8) -> bool {
        self.tokens[self.pos].kind == TokenKind::Delim(ch)
    }

    #[inline(always)]
    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn eat_delim(&mut self, ch: u8) -> bool {
        if self.at_delim(ch) {
            self.bump();
            true
        } else {
            false
        }
    }

    // -- whitespace ---------------------------------------------------------

    /// Skip `Whitespace` tokens only.
    pub fn skip_whitespace(&mut self) {
        while self.at(TokenKind::Whitespace) {
            self.bump();
        }
    }

    /// Skip `Whitespace`, `Comment`, `Cdo`, and `Cdc` tokens.
    pub fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek().kind {
                TokenKind::Whitespace | TokenKind::Comment | TokenKind::Cdo | TokenKind::Cdc => {
                    self.bump();
                }
                _ => break,
            }
        }
    }

    // -- spans & text -------------------------------------------------------

    /// Byte offset of the current token's start.
    #[inline(always)]
    pub fn current_start(&self) -> u32 {
        self.tokens[self.pos].span.start
    }

    /// Zero-width span at the current token's start (for error reporting).
    #[inline(always)]
    pub fn span_at(&self) -> Span {
        let s = self.current_start();
        Span::new(s, s)
    }

    /// Span from `start` byte offset to the end of the last consumed token.
    #[inline(always)]
    pub fn span_from(&self, start: u32) -> Span {
        Span::new(start, self.prev_end)
    }

    /// Source text of an arbitrary span.
    #[inline]
    pub fn source_text(&self, span: Span) -> &'src str {
        &self.src[span.start as usize..span.end as usize]
    }

    /// Source text of the current (not yet consumed) token.
    #[inline]
    pub fn current_raw(&self) -> &'src str {
        let sp = self.tokens[self.pos].span;
        &self.src[sp.start as usize..sp.end as usize]
    }

    /// Source text of a span with `skip` leading bytes removed.
    /// Useful for extracting the name from Hash (`#foo`) or AtKeyword (`@media`).
    #[inline]
    pub fn text_after(&self, span: Span, skip: u32) -> &'src str {
        &self.src[(span.start + skip) as usize..span.end as usize]
    }

    // -- save / restore -----------------------------------------------------

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

    // -- lookahead ----------------------------------------------------------

    /// Lookahead: is the current block item a nested rule (`{`) or a
    /// declaration (`;`/`}`)?  Scans forward through tokens without
    /// consuming, tracking paren depth.
    pub fn block_item_is_rule(&self) -> bool {
        let mut i = self.pos;
        let mut paren_depth: u32 = 0;
        while i < self.tokens.len() {
            match self.tokens[i].kind {
                TokenKind::LParen => paren_depth += 1,
                TokenKind::RParen => paren_depth = paren_depth.saturating_sub(1),
                TokenKind::LBrace if paren_depth == 0 => return true,
                TokenKind::Semicolon | TokenKind::RBrace if paren_depth == 0 => return false,
                TokenKind::Eof => return false,
                _ => {}
            }
            i += 1;
        }
        false
    }

    /// Predicate: is the current token the start of a combinator?
    #[inline(always)]
    pub fn is_combinator_start(&self) -> bool {
        match self.peek().kind {
            TokenKind::Delim(b'+' | b'~' | b'>') => true,
            TokenKind::Delim(b'|') => self.peek_at(1).kind == TokenKind::Delim(b'|'),
            _ => false,
        }
    }

    // -- recovery -----------------------------------------------------------

    /// Skip to the next unquoted `}` or end of input, consuming the `}`.
    pub fn skip_to_block_end(&mut self) {
        let mut depth: u32 = 0;
        loop {
            match self.peek().kind {
                TokenKind::LBrace => {
                    depth += 1;
                    self.bump();
                }
                TokenKind::RBrace => {
                    if depth == 0 {
                        self.bump();
                        return;
                    }
                    depth -= 1;
                    self.bump();
                }
                TokenKind::Eof => return,
                _ => {
                    self.bump();
                }
            }
        }
    }

    /// Skip to next `;` or `}` (without consuming `}`), or end of input.
    pub fn skip_to_semicolon_or_block_end(&mut self) {
        loop {
            match self.peek().kind {
                TokenKind::Semicolon => {
                    self.bump();
                    return;
                }
                TokenKind::RBrace | TokenKind::Eof => return,
                _ => {
                    self.bump();
                }
            }
        }
    }

    /// Skip an entire rule: selector part + `{ ... }`.
    pub fn skip_rule(&mut self) {
        loop {
            match self.peek().kind {
                TokenKind::LBrace => {
                    self.bump();
                    self.skip_to_block_end();
                    return;
                }
                TokenKind::Semicolon => {
                    self.bump();
                    return;
                }
                TokenKind::RBrace | TokenKind::Eof => return,
                _ => {
                    self.bump();
                }
            }
        }
    }
}

// ===========================================================================
// Tokenizer
// ===========================================================================

fn tokenize(src: &str) -> Vec<Token> {
    let bytes = src.as_bytes();
    let len = bytes.len();
    // Heuristic: average ~4 bytes per token in typical CSS.
    let mut tokens = Vec::with_capacity(len / 4 + 1);
    let mut pos: usize = 0;

    while pos < len {
        let start = pos as u32;
        let b = bytes[pos];

        // -- whitespace -----------------------------------------------------
        if is_css_ws(b) {
            pos += 1;
            while pos < len && is_css_ws(bytes[pos]) {
                pos += 1;
            }
            tokens.push(Token::new(TokenKind::Whitespace, start, pos as u32));
            continue;
        }

        match b {
            // -- strings ----------------------------------------------------
            b'"' | b'\'' => {
                pos += 1;
                scan_string_tail(bytes, &mut pos, b);
                tokens.push(Token::new(TokenKind::String, start, pos as u32));
            }

            // -- comments or Delim(/) ---------------------------------------
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

            // -- HTML comment <!-- ... --> as single token -------------------
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

            // -- @ → AtKeyword or Delim -------------------------------------
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

            // -- # → Hash or Delim ------------------------------------------
            b'#' => {
                pos += 1;
                if pos < len && (is_ident_char(bytes[pos]) || bytes[pos] == b'\\') {
                    consume_ident(bytes, &mut pos);
                    tokens.push(Token::new(TokenKind::Hash, start, pos as u32));
                } else {
                    tokens.push(Token::new(TokenKind::Delim(b'#'), start, pos as u32));
                }
            }

            // -- numeric starting with digit --------------------------------
            b'0'..=b'9' => {
                consume_number(bytes, &mut pos);
                let kind = classify_after_number(bytes, &mut pos);
                tokens.push(Token::new(kind, start, pos as u32));
            }

            // -- . → numeric (.5) or Delim ----------------------------------
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

            // -- + → numeric or Delim ---------------------------------------
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

            // -- - → CDC, numeric, ident, or Delim --------------------------
            b'-' => {
                // CDC: -->
                if pos + 2 < len && bytes[pos + 1] == b'-' && bytes[pos + 2] == b'>' {
                    pos += 3;
                    tokens.push(Token::new(TokenKind::Cdc, start, pos as u32));
                }
                // Negative number
                else if starts_number_after_sign(bytes, pos) {
                    consume_number(bytes, &mut pos);
                    let kind = classify_after_number(bytes, &mut pos);
                    tokens.push(Token::new(kind, start, pos as u32));
                }
                // Ident starting with -
                else if pos + 1 < len
                    && (is_ident_start(bytes[pos + 1])
                        || bytes[pos + 1] == b'-'
                        || bytes[pos + 1] == b'\\')
                {
                    consume_ident(bytes, &mut pos);
                    tokens.push(Token::new(TokenKind::Ident, start, pos as u32));
                }
                // Just a delimiter
                else {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delim(b'-'), start, pos as u32));
                }
            }

            // -- ident-start characters -------------------------------------
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | 0x80..=0xFF => {
                consume_ident(bytes, &mut pos);
                tokens.push(Token::new(TokenKind::Ident, start, pos as u32));
            }

            // -- backslash → escaped ident or Delim -------------------------
            b'\\' => {
                if pos + 1 < len && !is_css_ws(bytes[pos + 1]) {
                    consume_ident(bytes, &mut pos);
                    tokens.push(Token::new(TokenKind::Ident, start, pos as u32));
                } else {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delim(b'\\'), start, pos as u32));
                }
            }

            // -- simple punctuation -----------------------------------------
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

            // -- everything else → Delim ------------------------------------
            _ => {
                pos += 1;
                tokens.push(Token::new(TokenKind::Delim(b), start, pos as u32));
            }
        }
    }

    tokens.push(Token::new(TokenKind::Eof, len as u32, len as u32));
    tokens
}

// ---------------------------------------------------------------------------
// Tokenizer helpers
// ---------------------------------------------------------------------------

/// Check if `+` or `-` at `pos` starts a number:
/// sign followed by digit, or sign followed by `.` + digit.
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

/// Consume a CSS identifier (ident continuation chars + escapes).
/// Assumes `pos` is at the start of an ident or the first char of an ident
/// (may be `-`, `_`, letter, non-ASCII, or `\`).
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

/// Consume one CSS escape sequence (after the `\` has been consumed).
#[inline]
fn consume_escape(bytes: &[u8], pos: &mut usize) {
    if bytes[*pos].is_ascii_hexdigit() {
        let hex_start = *pos;
        while *pos < bytes.len() && *pos - hex_start < 6 && bytes[*pos].is_ascii_hexdigit() {
            *pos += 1;
        }
        // Optional single whitespace after hex escape.
        if *pos < bytes.len() && is_css_ws(bytes[*pos]) {
            *pos += 1;
        }
    } else {
        // Any other character (multi-byte aware).
        *pos += utf8_char_len(bytes[*pos]);
    }
}

/// Consume a numeric value: optional sign, digits, optional `.` + digits.
#[inline]
fn consume_number(bytes: &[u8], pos: &mut usize) {
    let len = bytes.len();
    // Optional sign
    if *pos < len && (bytes[*pos] == b'+' || bytes[*pos] == b'-') {
        *pos += 1;
    }
    // Integer part
    while *pos < len && bytes[*pos].is_ascii_digit() {
        *pos += 1;
    }
    // Decimal part
    if *pos < len && bytes[*pos] == b'.' && *pos + 1 < len && bytes[*pos + 1].is_ascii_digit() {
        *pos += 1; // skip '.'
        while *pos < len && bytes[*pos].is_ascii_digit() {
            *pos += 1;
        }
    }
}

/// After consuming a number, classify as Number, Percentage, or Dimension.
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

/// Scan string contents after the opening quote has been consumed.
fn scan_string_tail(bytes: &[u8], pos: &mut usize, quote: u8) {
    let len = bytes.len();
    while *pos < len {
        let b = bytes[*pos];
        if b == b'\\' && *pos + 1 < len {
            *pos += 2; // skip escape pair
        } else if b == quote {
            *pos += 1;
            return;
        } else {
            *pos += 1;
        }
    }
    // Unterminated string — span covers to end of input.
}

/// Scan HTML comment body after `<!--` has been consumed. Advances past `-->`.
fn scan_html_comment_tail(bytes: &[u8], pos: &mut usize) {
    let len = bytes.len();
    while *pos + 2 < len {
        if bytes[*pos] == b'-' && bytes[*pos + 1] == b'-' && bytes[*pos + 2] == b'>' {
            *pos += 3;
            return;
        }
        *pos += 1;
    }
    // Unterminated — consume to end.
    *pos = len;
}

/// Scan comment body after `/*` has been consumed. Advances past `*/`.
fn scan_comment_tail(bytes: &[u8], pos: &mut usize) {
    let len = bytes.len();
    while *pos + 1 < len {
        if bytes[*pos] == b'*' && bytes[*pos + 1] == b'/' {
            *pos += 2;
            return;
        }
        *pos += 1;
    }
    // Unterminated comment — consume remaining bytes.
    if *pos < len {
        *pos += 1;
    }
}
