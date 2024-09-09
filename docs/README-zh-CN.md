# 📊 Rust 标记计数工具 (tc) 🦀

一个用 Rust 编写的简单高效的标记计数程序！🚀

[English](../README.md) | 简体中文 | [繁體中文](README-zh-TW.md) | [日本語](README-ja-JP.md) | [한국어](README-ko-KR.md) | [Deutsch](README-de-DE.md)
## 📝 描述

这个 Rust 实现的经典 `tc`（标记计数）命令行工具可以统计文本文件或标准输入中的行数、单词数和字符数。它快速、可靠，并且支持 Unicode！🌍✨

## 🎯 特性

- 统计行数 📏
- 统计单词数 🔤
- 统计字符数（包括多字节 Unicode 字符）🔡
- 统计标记数（尚未准备就绪）🔤
- 处理多个文件 📚
- 从标准输入读取 🖥️
- 支持多种语言（英语、韩语、日语等）🌐

## 🛠️ 安装

1. 确保您的系统已安装 Rust。如果没有，请从 [rust-lang.org](https://www.rust-lang.org/tools/install) 获取 🦀

2. 克隆此仓库：
   ```
   git clone https://github.com/guuzaa/tc.git
   cd tc
   ```

3. 构建项目：
   ```
   cargo build --release
   ```

4. 可执行文件将位于 `target/release/tc`

## 🚀 使用方法

### 选项：

- `-l, --lines`：显示行数 📏
- `-w, --words`：显示单词数 🔤
- `-c, --chars`：显示字符数 🔡

如果未指定选项，将显示所有计数（行数、单词数和字符数）。

### 示例：

1. 统计文件中的行数、单词数和字符数：
   ```
   ./tc example.txt
   ```

2. 仅统计多个文件中的单词数：
   ```
   ./tc -w file1.txt file2.txt file3.txt
   ```

3. 从标准输入统计行数和字符数：
   ```
   echo "你好，世界！" | ./tc -l -c
   ```

4. 统计不同语言文件中的所有内容：
   ```
   ./tc english.txt korean.txt japanese.txt
   ```

## 🧪 运行测试

要运行测试套件，请使用以下命令：
```
cargo test
```


## 🤝 贡献

欢迎贡献！随时提交问题或拉取请求。🎉

## 📜 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](../LICENSE) 文件。📄

## 🙏 致谢

- Rust 社区提供的优秀工具和支持 🦀❤️
- 原始 Unix `wc` 命令的灵感 🖥️
- Cursor 编辑器 🤖

开始愉快的统计吧！🎉📊🚀