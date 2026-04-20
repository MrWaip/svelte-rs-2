/// Sanitize a filename for use in trace labels by inserting a zero-width space
/// after each `/` to prevent devtools from treating it as a clickable link.
pub fn sanitize_location(filename: &str) -> String {
    filename.replace('/', "/\u{200b}")
}

/// Compute 1-based line and column from source text and byte offset.
pub fn compute_line_col(source: &str, offset: u32) -> (usize, usize) {
    let offset = offset as usize;
    let bytes = source.as_bytes();
    let mut line = 1;
    let mut col = 0;
    for &byte in &bytes[..offset.min(bytes.len())] {
        if byte == b'\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}
