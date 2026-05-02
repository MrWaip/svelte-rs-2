pub fn sanitize_location(filename: &str) -> String {
    filename.replace('/', "/\u{200b}")
}
