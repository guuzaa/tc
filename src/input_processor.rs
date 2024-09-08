use std::io::{self, Read, Write};

pub struct CountOptions {
    pub show_lines: bool,
    pub show_words: bool,
    pub show_bytes: bool,
}

pub fn process_input<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    options: &CountOptions,
) -> io::Result<()> {
    let mut line_count = 0;
    let mut word_count = 0;

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let char_count = String::from_utf8_lossy(&buffer).chars().count();

    let lines = buffer.split(|&b| b == b'\n');
    let mut lines_iter = lines.peekable();

    while let Some(line) = lines_iter.next() {
        if lines_iter.peek().is_none() && line.is_empty() {
            break;
        }

        line_count += 1;
        word_count += line
            .split(|&b| b.is_ascii_whitespace())
            .filter(|&w| !w.is_empty())
            .count();
    }

    let mut output = String::new();
    if options.show_lines {
        output.push_str(&format!("{:7}", line_count));
    }
    if options.show_words {
        output.push_str(&format!("{:8}", word_count));
    }
    if options.show_bytes {
        output.push_str(&format!("{:8}", char_count));
    }

    writeln!(writer, "{}", output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_empty_input() {
        let mut input = Cursor::new(b"");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      0       0       0\n"
        );
    }

    #[test]
    fn test_single_word() {
        let mut input = Cursor::new(b"hello");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       1       5\n"
        );
    }

    #[test]
    fn test_multiple_words() {
        let mut input = Cursor::new(b"hello world\nrust is great");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      2       5      25\n"
        );
    }

    #[test]
    fn test_show_lines_only() {
        let mut input = Cursor::new(b"hello\nworld\n");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: false,
            show_bytes: false,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(String::from_utf8(output).unwrap(), "      2\n");
    }

    #[test]
    fn test_show_words_only() {
        let mut input = Cursor::new(b"hello world rust");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: true,
            show_bytes: false,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(String::from_utf8(output).unwrap(), "       3\n");
    }

    #[test]
    fn test_show_bytes_only() {
        let mut input = Cursor::new(b"hello\n");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: false,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(String::from_utf8(output).unwrap(), "       6\n");
    }

    #[test]
    fn test_utf8_characters() {
        let mut input = Cursor::new("Hello, 世界!\n");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       2      11\n"
        );
    }

    #[test]
    fn test_multi_byte_characters() {
        let mut input = Cursor::new("🚀 Rust 💻\n");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       3       9\n"
        );
    }

    #[test]
    fn test_korean_characters() {
        let mut input = Cursor::new("안녕하세요 세계!\n");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       2      10\n"
        );
    }

    #[test]
    fn test_japanese_characters() {
        let mut input = Cursor::new("こんにちは 世界！\n");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       2      10\n"
        );
    }

    #[test]
    fn test_mixed_languages() {
        let mut input = Cursor::new("Hello 안녕 こんにちは World!\n");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       4      22\n"
        );
    }

    #[test]
    fn test_korean_multiline() {
        let mut input = Cursor::new("안녕하세요\n세계입니다\n");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      2       2      12\n"
        );
    }

    #[test]
    fn test_japanese_multiline() {
        let mut input = Cursor::new("こんにちは\n世界です\n");
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        let _ = process_input(&mut input, &mut output, &options);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      2       2      11\n"
        );
    }
}
