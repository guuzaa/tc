# 📊 Rust Word Count (wc) 🦀

A simple and efficient word count program written in Rust! 🚀

English | [简体中文](docs/README-zh-CN.md) | [繁體中文](docs/README-zh-TW.md) | [日本語](docs/README-ja-JP.md) | [한국어](docs/README-ko-KR.md) | [Deutsch](docs/README-de-DE.md)

## 📝 Description

This Rust implementation of the classic `wc` (word count) command-line tool allows you to count lines, words, and characters in text files or from standard input. It's fast, reliable, and supports Unicode! 🌍✨

## 🎯 Features

- Count lines 📏
- Count words 🔤
- Count characters (including multi-byte Unicode characters) 🔡
- Process multiple files 📚
- Read from standard input 🖥️
- Supports various languages (English, Korean, Japanese, and more!) 🌐

## 🛠️ Installation

1. Make sure you have Rust installed on your system. If not, get it from [rust-lang.org](https://www.rust-lang.org/tools/install) 🦀

2. Clone this repository:
   ```
   git clone https://github.com/guuzaa/wc.rs.git
   cd wc.rs
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. The executable will be available at `target/release/wc`

## 🚀 Usage

### Options:

- `-l, --lines`: Show line count 📏
- `-w, --words`: Show word count 🔤
- `-c, --chars`: Show character count 🔡

If no options are specified, all counts (lines, words, and characters) will be shown.

### Examples:

1. Count lines, words, and characters in a file:
   ```
   ./wc example.txt
   ```

2. Count only words in multiple files:
   ```
   ./wc -w file1.txt file2.txt file3.txt
   ```

3. Count lines and characters from standard input:
   ```
   echo "Hello, World!" | ./wc -l -c
   ```

4. Count everything in files with different languages:
   ```
   ./wc english.txt korean.txt japanese.txt
   ```

## 🧪 Running Tests

To run the test suite, use the following command:
```
cargo test
```

## 🤝 Contributing

Contributions are welcome! Feel free to submit issues or pull requests. 🎉

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details. 📄

## 🙏 Acknowledgements

- The Rust community for their amazing tools and support 🦀❤️
- The original Unix `wc` command for inspiration 🖥️
- The editor Cursor 🤖

Happy counting! 🎉📊🚀