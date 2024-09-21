use crate::cmd::TokenizerModel;
use crate::counts::{CountOptions, InputCounts};
use rust_i18n::t;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use tiktoken_rs::{cl100k_base, o200k_base, p50k_base, p50k_edit, r50k_base};

pub fn process_inputs<W>(files: &[String], writer: &mut W, options: &CountOptions) -> io::Result<()>
where
    W: Write,
{
    let mut total_counts = InputCounts::default();
    let mut file_count = 0;
    let mut error_count = 0;

    if files.is_empty() {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin.lock());
        if let Err(err) = process_input(&mut reader, writer, options, None) {
            error_count += 1;
            match err.kind() {
                io::ErrorKind::WriteZero => {
                    eprintln!("{}", t!("error_writing_stdout"));
                }
                _ => {
                    eprintln!("{}", t!("error_reading_stdin"));
                }
            }
        }
    } else {
        for filename in files {
            match File::open(filename) {
                Ok(mut file) => match process_input(&mut file, writer, options, Some(filename)) {
                    Ok(counts) => {
                        total_counts += counts;
                    }
                    Err(err) => {
                        error_count += 1;
                        match err.kind() {
                            io::ErrorKind::WriteZero => {
                                eprintln!("{}", t!("error_writing_stdout"));
                            }
                            _ => {
                                eprintln!(
                                    "{}",
                                    t!(
                                        "error_reading_file",
                                        filename = filename,
                                        error = err.kind()
                                    )
                                );
                            }
                        }
                    }
                },
                Err(err) => {
                    error_count += 1;
                    match err.kind() {
                        io::ErrorKind::NotFound => {
                            eprintln!("{}", t!("error_not_found", filename = filename));
                        }
                        io::ErrorKind::PermissionDenied => {
                            eprintln!("{}", t!("error_permission_denied", filename = filename));
                        }
                        _ => {
                            eprintln!(
                                "{}",
                                t!(
                                    "error_opening_file",
                                    filename = filename,
                                    error = err.kind()
                                )
                            );
                        }
                    }
                }
            }
            file_count += 1;
        }
    }
    if file_count > 1 {
        print_counts(
            writer,
            &total_counts,
            options,
            Some(&format!("{}", t!("total"))),
        )?;
    }

    if error_count > 0 {
        Err(io::Error::new(io::ErrorKind::Other, ""))
    } else {
        Ok(())
    }
}

fn process_input<R, W>(
    reader: &mut R,
    writer: &mut W,
    options: &CountOptions,
    filename: Option<&str>,
) -> io::Result<InputCounts>
where
    R: Read,
    W: Write,
{
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let counts = count_input(&buffer, options);
    print_counts(writer, &counts, options, filename)?;
    Ok(counts)
}

fn count_input(buffer: &[u8], options: &CountOptions) -> InputCounts {
    let buffer_string = String::from_utf8_lossy(buffer);
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    let mut token_count = 0;

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

    if options.show_chars {
        char_count = buffer_string.chars().count();
    }

    if options.show_tokens {
        let tokenizer = match options.tokenizer_model {
            TokenizerModel::GPT3 => r50k_base().unwrap(),
            TokenizerModel::Edit => p50k_edit().unwrap(),
            TokenizerModel::Code => p50k_base().unwrap(),
            TokenizerModel::ChatGPT => cl100k_base().unwrap(),
            TokenizerModel::GPT4O => o200k_base().unwrap(),
        };
        token_count = tokenizer.encode_ordinary(&buffer_string).len();
    }

    InputCounts {
        lines: line_count,
        words: word_count,
        chars: char_count,
        tokens: token_count,
    }
}

fn print_counts<W: Write>(
    writer: &mut W,
    counts: &InputCounts,
    options: &CountOptions,
    filename: Option<&str>,
) -> io::Result<()> {
    let mut output = String::new();
    let format_len = if options.count_enabled_options() == 1 {
        0
    } else {
        8
    };

    if options.show_lines {
        output.push_str(&format!("{:format_len$}", counts.lines));
    }
    if options.show_words {
        output.push_str(&format!("{:format_len$}", counts.words));
    }
    if options.show_chars {
        output.push_str(&format!("{:format_len$}", counts.chars));
    }
    if options.show_tokens {
        output.push_str(&format!("{:format_len$}", counts.tokens));
    }

    if let Some(name) = filename {
        output.push_str(&format!(" {}", name));
    }

    if writeln!(writer, "{}", output.trim_end()).is_err() {
        Err(io::Error::new(io::ErrorKind::WriteZero, ""))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let test_file = "tc_test_tmp_empty_input.txt";
        let input = b"";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            output,
            format!("       0       0       0 {}\n", test_file).as_bytes()
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_single_word() {
        let test_file = "tc_test_tmp_single_word.txt";
        let input = b"hello";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       1       1       5 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_multiple_words() {
        let test_file = "tc_test_tmp_multiple_words.txt";
        let input = b"hello world\nrust is great";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       2       5      25 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_show_lines_only() {
        let test_file = "tc_test_tmp_show_lines_only.txt";
        let input = b"hello\nworld\n";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: false,
            show_chars: false,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("2 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_show_words_only() {
        let test_file = "tc_test_tmp_show_words_only.txt";
        let input = b"hello world rust";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: true,
            show_chars: false,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("3 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_show_chars_only() {
        let test_file = "tc_test_tmp_show_chars_only.txt";
        let input = b"hello\n";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: false,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("6 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_utf8_characters() {
        let test_file = "tc_test_tmp_utf8_characters.txt";
        let input = "Hello, 世界!\n";
        std::fs::write(test_file, input.as_bytes()).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       1       2      11 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_multi_byte_characters() {
        let test_file = "tc_test_tmp_multi_byte_characters.txt";
        let input = "🚀 Rust 💻\n";
        std::fs::write(test_file, input.as_bytes()).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       1       3       9 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_korean_characters() {
        let test_file = "tc_test_tmp_korean_characters.txt";
        let input = "안녕하세요 세계!\n";
        std::fs::write(test_file, input.as_bytes()).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       1       2      10 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_japanese_characters() {
        let test_file = "tc_test_tmp_japanese_characters.txt";
        let input = "こんにちは 世界！\n";
        std::fs::write(test_file, input.as_bytes()).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       1       2      10 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_mixed_languages() {
        let test_file = "tc_test_tmp_mixed_languages.txt";
        let input = "Hello 안녕 こんにちは World!\n";
        std::fs::write(test_file, input.as_bytes()).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       1       4      22 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_korean_multiline() {
        let test_file = "tc_test_tmp_korean_multiline.txt";
        let input = "안녕하세요\n세계입니다\n";
        std::fs::write(test_file, input.as_bytes()).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       2       2      12 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_japanese_multiline() {
        let test_file = "tc_test_tmp_japanese_multiline.txt";
        let input = "こんにちは\n世界です\n";
        std::fs::write(test_file, input.as_bytes()).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: false,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       2       2      11 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_show_tokens() {
        let test_file = "tc_test_tmp_show_tokens.txt";
        let input = "Hello, world!\n世界です";
        std::fs::write(test_file, input.as_bytes()).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: false,
            show_chars: false,
            show_tokens: true,
            tokenizer_model: TokenizerModel::Edit,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("11 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_all_options() {
        let test_file = "tc_test_tmp_all_options.txt";
        let input = b"Hello, world!\nThis is a test.";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: true,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       2       6      29      10 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_show_tokens_with_different_models() {
        let test_file = "tc_test_tmp_show_tokens_with_model.txt";
        let input = b"Hello, world!";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: false,
            show_chars: false,
            show_tokens: true,
            tokenizer_model: TokenizerModel::GPT3,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("4 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();

        let test_file = "tc_test_tmp_show_tokens_with_model.txt";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: false,
            show_words: false,
            show_chars: false,
            show_tokens: true,
            tokenizer_model: TokenizerModel::GPT4O,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("4 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_all_options_with_model() {
        let test_file = "tc_test_tmp_all_options_with_model.txt";
        let input = b"Hello, world!\nThis is a test.";
        std::fs::write(test_file, input).unwrap();
        let mut output = Vec::new();
        let options = CountOptions {
            show_lines: true,
            show_words: true,
            show_chars: true,
            show_tokens: true,
            tokenizer_model: TokenizerModel::GPT4O,
        };
        process_inputs(&[test_file.to_string()], &mut output, &options).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("       2       6      29       9 {}\n", test_file)
        );
        std::fs::remove_file(test_file).unwrap();
    }
}
