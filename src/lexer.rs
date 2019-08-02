pub struct Lexer<'a> {
    input: &'a str,
    position: u32,
    read_position: u32,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(self, input: &str) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }
}