# 📊 Token Count (tc) 🦀

A simple and efficient token count program written in Rust! 🚀

English | [简体中文](docs/README-zh-CN.md) | [繁體中文](docs/README-zh-TW.md) | [日本語](docs/README-ja-JP.md) | [한국어](docs/README-ko-KR.md) | [Deutsch](docs/README-de-DE.md)

## 📝 Description

This Rust implementation of the classic `wc` (word count) command-line tool allows you to count lines, words, characters, and even tokens in text files or from standard input. It's fast, reliable, and supports Unicode! 🌍✨

## 🎯 Features

- Count lines 📏
- Count words 🔤
- Count characters (including multi-byte Unicode characters) 🔡
- Count tokens using various tokenizer models 🔢
- Process multiple files 📚
- Read from standard input 🖥️
- Supports various languages (English, Korean, Japanese, and more!) 🌐

## 🛠️ Installation

There are two ways to install tc:

### Option 1: Install from source

1. Make sure you have Rust installed on your system. If not, get it from [rust-lang.org](https://www.rust-lang.org/tools/install) 🦀

2. Clone this repository:
   ```
   git clone https://github.com/guuzaa/tc.git
   cd tc
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. The executable will be available at `target/release/tc`

### Option 2: Install pre-built binaries

1. Go to the [Releases page](https://github.com/guuzaa/tc/releases) of the tc repository.

2. Download the latest release for your operating system and architecture.

3. Extract the downloaded archive.

4. Move the `tc` executable to a directory in your system's PATH (e.g., `/usr/local/bin` on Unix-like systems).

5. You can now use tc from anywhere in your terminal!

## 🚀 Usage

### Options:

- `-l, --lines`: Show line count 📏
- `-w, --words`: Show word count 🔤
- `-c, --chars`: Show character count 🔡
- `-t, --tokens`: Show token count 🔢
- `--model <MODEL>`: Choose tokenizer model (default: gpt3)

Available models:
- `gpt3`: r50k_base
- `edit`: p50k_edit
- `code`: p50k_base
- `chatgpt`: cl100k_base
- `gpt4o`: o200k_base

If no options are specified, all counts (lines, words, characters, and tokens) will be shown.

### Examples:

1. Count lines, words, and characters in a file:
   ```
   tc example.txt
   ```

2. Count only words in multiple files:
   ```
   tc -w file1.txt file2.txt file3.txt
   ```

3. Count lines and characters from standard input:
   ```
   echo "Hello, World!" | tc -lc
   ```

4. Count tokens using the ChatGPT tokenizer:
   ```
   tc -t --model chatgpt example.txt
   ```

5. Count everything in files with different languages:
   ```
   tc english.txt korean.txt japanese.txt
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