# 📊 Rust 標記計數工具 (tc) 🦀

一個用 Rust 編寫的簡單高效的標記計數程式！🚀

[English](../README.md) | [简体中文](README-zh-CN.md) | 繁體中文 | [日本語](README-ja-JP.md) | [한국어](README-ko-KR.md) | [Deutsch](README-de-DE.md)

## 📝 描述

這個 Rust 實現的經典 `wc`（字數統計）命令列工具可以統計文字檔案或標準輸入中的行數、單字數、字元數，甚至是標記數。它快速、可靠，並且支援 Unicode！🌍✨

## 🎯 特性

- 統計行數 📏
- 統計單字數 🔤
- 統計字元數（包括多位元組 Unicode 字元）🔡
- 使用各種分詞器模型統計標記數 🔢
- 處理多個檔案 📚
- 從標準輸入讀取 🖥️
- 支援多種語言（英語、韓語、日語等）🌐

## 🛠️ 安裝

1. 確保您的系統已安裝 Rust。如果沒有，請從 [rust-lang.org](https://www.rust-lang.org/tools/install) 獲取 🦀

2. 複製此儲存庫：
   ```
   git clone https://github.com/guuzaa/tc.git
   cd tc
   ```

3. 建置專案：
   ```
   cargo build --release
   ```

4. 可執行檔將位於 `target/release/tc`

## 🚀 使用方法

### 選項：

- `-l, --lines`：顯示行數 📏
- `-w, --words`：顯示單字數 🔤
- `-c, --chars`：顯示字元數 🔡
- `-t, --tokens`：顯示標記數 🔢
- `--model <MODEL>`：選擇分詞器模型（預設：gpt3）

可用模型：
- `gpt3`: r50k_base
- `edit`: p50k_edit
- `code`: p50k_base
- `chatgpt`: cl100k_base
- `gpt4o`: o200k_base

如果未指定選項，將顯示所有計數（行數、單字數、字元數和標記數）。

### 範例：

1. 統計檔案中的行數、單字數和字元數：
   ```
   ./tc example.txt
   ```

2. 僅統計多個檔案中的單字數：
   ```
   ./tc -w file1.txt file2.txt file3.txt
   ```

3. 從標準輸入統計行數和字元數：
   ```
   echo "你好，世界！" | ./tc -lc
   ```

4. 使用 ChatGPT 分詞器統計標記數：
   ```
   ./tc -t --model chatgpt example.txt
   ```

5. 統計不同語言檔案中的所有內容：
   ```
   ./tc english.txt korean.txt japanese.txt
   ```

## 🧪 執行測試

要執行測試套件，請使用以下命令：
```
cargo test
```

## 🤝 貢獻

歡迎貢獻！隨時提交問題或拉取請求。🎉

## 📜 授權條款

本專案採用 MIT 授權條款。詳情請參閱 [LICENSE](../LICENSE) 檔案。📄

## 🙏 致謝

- Rust 社群提供的優秀工具和支援 🦀❤️
- 原始 Unix `wc` 命令的靈感 🖥️
- Cursor 編輯器 🤖

開始愉快的統計吧！🎉📊🚀
