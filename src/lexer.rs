pub struct Lexer {
    source_path: String,
    source: String,
}

impl Lexer {
    fn new(source_path: String) -> Lexer {
        let source = fs::read_to_string(&source_path).expect("Error reading file");
        Lexer {
            source_path,
            source,
        }
    }
}
