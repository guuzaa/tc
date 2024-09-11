use std::io::{self, Read, Write};
use tiktoken_rs::cl100k_base;

pub struct CountOptions {
    pub show_lines: bool,
    pub show_words: bool,
    pub show_bytes: bool,
    pub show_tokens: bool,
}

pub fn process_input<R, W>(reader: &mut R, writer: &mut W, options: &CountOptions) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let buffer_string = String::from_utf8_lossy(&buffer);
    let mut line_count = 0;
    let mut word_count = 0;

    let lines = buffer.split(|&b| b == b'\n');
    let mut lines_iter = lines.peekable();

    while let Some(line) = lines_iter.next() {
        if lines_iter.peek().is_none() && line.is_empty() {
            break;
        }
        if options.show_lines {
            line_count += 1;
        }
        if options.show_words {
            word_count += line
                .split(|&b| b.is_ascii_whitespace())
                .filter(|&w| !w.is_empty())
                .count();
        }
    }

    let mut output = String::new();
    if options.show_lines {
        output.push_str(&format!("{:8}", line_count));
    }
    if options.show_words {
        output.push_str(&format!("{:8}", word_count));
    }
    if options.show_bytes {
        let char_count = buffer_string.chars().count();
        output.push_str(&format!("{:8}", char_count));
    }
    if options.show_tokens {
        let bpe = cl100k_base().unwrap();
        let token_count = bpe.encode_ordinary(&buffer_string).len();
        output.push_str(&format!("{:8}", token_count));
    }

    writer.write_all(output.as_bytes())?;
    writer.write_all(b"\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_empty_input() {
        let input = b"";
        let mut reader = Cursor::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(output, b"       0       0       0\n");
    }

    #[test]
    fn test_single_word() {
        let input = b"hello";
        let mut reader = Cursor::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       1       1       5\n"
        );
    }

    #[test]
    fn test_multiple_words() {
        let input = b"hello world\nrust is great";
        let mut reader = Cursor::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       2       5      25\n"
        );
    }

    #[test]
    fn test_show_lines_only() {
        let input = b"hello\nworld\n";
        let mut reader = Cursor::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: false,
            show_bytes: false,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "       2\n");
    }

    #[test]
    fn test_show_words_only() {
        let input = b"hello world rust";
        let mut reader = Cursor::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: true,
            show_bytes: false,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "       3\n");
    }

    #[test]
    fn test_show_bytes_only() {
        let input = b"hello\n";
        let mut reader = Cursor::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: false,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "       6\n");
    }

    #[test]
    fn test_utf8_characters() {
        let input = "Hello, 世界!\n";
        let mut reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       1       2      11\n"
        );
    }

    #[test]
    fn test_multi_byte_characters() {
        let input = "🚀 Rust 💻\n";
        let mut reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       1       3       9\n"
        );
    }

    #[test]
    fn test_korean_characters() {
        let input = "안녕하세요 세계!\n";
        let mut reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       1       2      10\n"
        );
    }

    #[test]
    fn test_japanese_characters() {
        let input = "こんにちは 世界！\n";
        let mut reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       1       2      10\n"
        );
    }

    #[test]
    fn test_mixed_languages() {
        let input = "Hello 안녕 こんにちは World!\n";
        let mut reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       1       4      22\n"
        );
    }

    #[test]
    fn test_korean_multiline() {
        let input = "안녕하세요\n세계입니다\n";
        let mut reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       2       2      12\n"
        );
    }

    #[test]
    fn test_japanese_multiline() {
        let input = "こんにちは\n世界です\n";
        let mut reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: false,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       2       2      11\n"
        );
    }

    #[test]
    fn test_show_tokens() {
        let input = b"Hello, world!";
        let mut reader = Cursor::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: false,
            show_bytes: false,
            show_tokens: true,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "       4\n");
    }

    #[test]
    fn test_all_options() {
        let input = b"Hello, world!\nThis is a test.";
        let mut reader = Cursor::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
            show_tokens: true,
        };
        process_input(&mut reader, &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "       2       6      29       9\n"
        );
    }
}
