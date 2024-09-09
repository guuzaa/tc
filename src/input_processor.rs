use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

pub struct CountOptions {
    pub show_lines: bool,
    pub show_words: bool,
    pub show_bytes: bool,
}

pub async fn process_input<R, W>(
    reader: &mut R,
    writer: &mut W,
    options: &CountOptions,
) -> io::Result<()>
where
    R: AsyncBufReadExt + AsyncReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    let mut buffer = String::new();
    while reader.read_line(&mut buffer).await? > 0 {
        line_count += 1;
        word_count += buffer.split_whitespace().count();
        char_count += buffer.chars().count();
        buffer.clear();
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

    writer.write_all(output.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    Ok(())
}

// Update tests to use tokio's runtime
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::BufReader;

    #[tokio::test]
    async fn test_empty_input() {
        let input = b"";
        let mut reader = BufReader::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(output, b"      0       0       0\n");
    }

    #[tokio::test]
    async fn test_single_word() {
        let input = b"hello";
        let mut reader = BufReader::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       1       5\n"
        );
    }

    #[tokio::test]
    async fn test_multiple_words() {
        let input = b"hello world\nrust is great";
        let mut reader = BufReader::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      2       5      25\n"
        );
    }

    #[tokio::test]
    async fn test_show_lines_only() {
        let input = b"hello\nworld\n";
        let mut reader = BufReader::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: false,
            show_bytes: false,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "      2\n");
    }

    #[tokio::test]
    async fn test_show_words_only() {
        let input = b"hello world rust";
        let mut reader = BufReader::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: true,
            show_bytes: false,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "       3\n");
    }

    #[tokio::test]
    async fn test_show_bytes_only() {
        let input = b"hello\n";
        let mut reader = BufReader::new(&input[..]);
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: false,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "       6\n");
    }

    #[tokio::test]
    async fn test_utf8_characters() {
        let input = "Hello, 世界!\n";
        let mut reader = BufReader::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       2      11\n"
        );
    }

    #[tokio::test]
    async fn test_multi_byte_characters() {
        let input = "🚀 Rust 💻\n";
        let mut reader = BufReader::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       3       9\n"
        );
    }

    #[tokio::test]
    async fn test_korean_characters() {
        let input = "안녕하세요 세계!\n";
        let mut reader = BufReader::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       2      10\n"
        );
    }

    #[tokio::test]
    async fn test_japanese_characters() {
        let input = "こんにちは 世界！\n";
        let mut reader = BufReader::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       2      10\n"
        );
    }

    #[tokio::test]
    async fn test_mixed_languages() {
        let input = "Hello 안녕 こんにちは World!\n";
        let mut reader = BufReader::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      1       4      22\n"
        );
    }

    #[tokio::test]
    async fn test_korean_multiline() {
        let input = "안녕하세요\n세계입니다\n";
        let mut reader = BufReader::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      2       2      12\n"
        );
    }

    #[tokio::test]
    async fn test_japanese_multiline() {
        let input = "こんにちは\n世界です\n";
        let mut reader = BufReader::new(input.as_bytes());
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        };
        process_input(&mut reader, &mut output, &options)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "      2       2      11\n"
        );
    }
}
