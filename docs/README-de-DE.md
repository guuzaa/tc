# 📊 Rust Token-Zählprogramm (tc) 🦀

Ein einfaches und effizientes Token-Zählprogramm, geschrieben in Rust! 🚀

[English](../README.md) | [简体中文](README-zh-CN.md) | [繁體中文](README-zh-TW.md) | [日本語](README-ja-JP.md) | [한국어](README-ko-KR.md) | Deutsch

## 📝 Beschreibung

Diese Rust-Implementierung des klassischen `wc` (Wortzählung) Kommandozeilenwerkzeugs ermöglicht es Ihnen, Zeilen, Wörter, Zeichen und sogar Tokens in Textdateien oder aus der Standardeingabe zu zählen. Es ist schnell, zuverlässig und unterstützt Unicode! 🌍✨

## 🎯 Funktionen

- Zählt Zeilen 📏
- Zählt Wörter 🔤
- Zählt Zeichen (einschließlich mehrbytiger Unicode-Zeichen) 🔡
- Zählt Tokens mit verschiedenen Tokenizer-Modellen 🔢
- Verarbeitet mehrere Dateien 📚
- Liest aus der Standardeingabe 🖥️
- Unterstützt verschiedene Sprachen (Englisch, Koreanisch, Japanisch und mehr!) 🌐

## 🛠️ Installation

Es gibt zwei Möglichkeiten, tc zu installieren:

### Option 1: Installation aus dem Quellcode

1. Stellen Sie sicher, dass Rust auf Ihrem System installiert ist. Falls nicht, laden Sie es von [rust-lang.org](https://www.rust-lang.org/tools/install) herunter 🦀

2. Klonen Sie dieses Repository:
   ```
   git clone https://github.com/guuzaa/tc.git
   cd tc
   ```

3. Bauen Sie das Projekt:
   ```
   cargo build --release
   ```

4. Die ausführbare Datei finden Sie unter `target/release/tc`

### Option 2: Installation vorkompilierter Binärdateien

1. Gehen Sie zur [Releases-Seite](https://github.com/guuzaa/tc/releases) des tc-Repositories.

2. Laden Sie die neueste Version für Ihr Betriebssystem und Ihre Architektur herunter.

3. Entpacken Sie das heruntergeladene Archiv.

4. Verschieben Sie die `tc`-Ausführungsdatei in ein Verzeichnis in Ihrem System-PATH (z.B. `/usr/local/bin` auf Unix-ähnlichen Systemen).

5. Sie können tc jetzt von überall in Ihrem Terminal verwenden!

## 🚀 Verwendung

### Optionen:

- `-l, --lines`: Zeigt die Zeilenanzahl 📏
- `-w, --words`: Zeigt die Wortanzahl 🔤
- `-c, --chars`: Zeigt die Zeichenanzahl 🔡
- `-t, --tokens`: Zeigt die Token-Anzahl 🔢
- `--model <MODEL>`: Wählt das Tokenizer-Modell (Standard: gpt3)

Verfügbare Modelle:
- `gpt3`: r50k_base
- `edit`: p50k_edit
- `code`: p50k_base
- `chatgpt`: cl100k_base
- `gpt4o`: o200k_base

Wenn keine Optionen angegeben werden, werden alle Zählungen (Zeilen, Wörter, Zeichen und Tokens) angezeigt.

### Beispiele:

1. Zähle Zeilen, Wörter und Zeichen in einer Datei:
   ```
   tc beispiel.txt
   ```

2. Zähle nur Wörter in mehreren Dateien:
   ```
   tc -w datei1.txt datei2.txt datei3.txt
   ```

3. Zähle Zeilen und Zeichen aus der Standardeingabe:
   ```
   echo "Hallo, Welt!" | tc -lc
   ```

4. Zähle Tokens mit dem ChatGPT-Tokenizer:
   ```
   tc -t --model chatgpt beispiel.txt
   ```

5. Zähle alles in Dateien mit verschiedenen Sprachen:
   ```
   tc englisch.txt koreanisch.txt japanisch.txt
   ```

## 🤝 Beitragen

Beiträge sind willkommen! Zögern Sie nicht, Probleme zu melden oder Pull-Requests einzureichen. 🎉

## 📜 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert. Weitere Details finden Sie in der [LICENSE](../LICENSE) Datei. 📄

## 🙏 Danksagungen

- Der Rust-Community für ihre erstaunlichen Werkzeuge und Unterstützung 🦀❤️
- Dem ursprünglichen Unix `wc`-Befehl für die Inspiration 🖥️
- Dem Cursor-Editor 🤖

Viel Spaß beim Zählen! 🎉📊🚀