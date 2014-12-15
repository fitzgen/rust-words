use std::io;
use std::io::stdio;
use std::io::IoErrorKind;

/// An iterator that yields each word from stdin until EOF is reached.
pub struct WordReader {
    reader: stdio::StdinReader
}

enum CharType {
    Character(char),
    WhiteSpace,
    EOF
}

fn is_eof(err : &io::IoError) -> bool {
    err.kind == IoErrorKind::EndOfFile
}

impl WordReader {
    /// Create a new WordReader instance.
    pub fn new(r: stdio::StdinReader) -> WordReader {
        WordReader { reader: r }
    }

    fn read_char(&mut self) -> CharType {
        match self.reader.read_char() {
            Ok(c) if c.is_whitespace() => CharType::WhiteSpace,
            Ok(c)                      => CharType::Character(c),
            Err(ref e) if is_eof(e)    => CharType::EOF,
            Err(_)                     => panic!("IO ERROR!"),
        }
    }
}

/// Iterator implementation for WordReader.
impl Iterator<String> for WordReader {
    fn next(&mut self) -> Option<String> {
        let mut word = String::new();

        // Eat initial whitespace.
        loop {
            match self.read_char() {
                CharType::EOF          => return None,
                CharType::WhiteSpace   => continue,
                CharType::Character(c) => {
                    word.push(c);
                    break;
                },
            }
        }

        // Read characters into `word` until we hit EOF or whitespace.
        loop {
            match self.read_char() {
                CharType::WhiteSpace | CharType::EOF => break,
                CharType::Character(c)               => word.push(c),
            }
        }

        if word.len() > 0u  { Some(word) } else { None }
    }
}
