# 📊 Rust Wortzählprogramm (wc) 🦀

Ein einfaches und effizientes Wortzählprogramm, geschrieben in Rust! 🚀

[English](../README.md) | [简体中文](README-zh-CN.md) | [繁體中文](README-zh-TW.md) | [日本語](README-ja-JP.md) | [한국어](README-ko-KR.md) | Deutsch

## 📝 Beschreibung

Diese Rust-Implementierung des klassischen `wc` (Wortzählung) Kommandozeilenwerkzeugs ermöglicht es Ihnen, Zeilen, Wörter und Zeichen in Textdateien oder aus der Standardeingabe zu zählen. Es ist schnell, zuverlässig und unterstützt Unicode! 🌍✨

## 🎯 Funktionen

- Zählt Zeilen 📏
- Zählt Wörter 🔤
- Zählt Zeichen (einschließlich mehrbytiger Unicode-Zeichen) 🔡
- Verarbeitet mehrere Dateien 📚
- Liest aus der Standardeingabe 🖥️
- Unterstützt verschiedene Sprachen (Englisch, Koreanisch, Japanisch und mehr!) 🌐

## 🛠️ Installation

1. Stellen Sie sicher, dass Rust auf Ihrem System installiert ist. Falls nicht, laden Sie es von [rust-lang.org](https://www.rust-lang.org/tools/install) herunter 🦀

2. Klonen Sie dieses Repository:
   ```
   git clone https://github.com/guuzaa/wc.rs.git
   cd wc.rs
   ```

3. Bauen Sie das Projekt:
   ```
   cargo build --release
   ```

4. Die ausführbare Datei finden Sie unter `target/release/wc`

## 🚀 Verwendung

### Optionen:

- `-l, --lines`: Zeigt die Zeilenanzahl 📏
- `-w, --words`: Zeigt die Wortanzahl 🔤
- `-c, --chars`: Zeigt die Zeichenanzahl 🔡

Wenn keine Optionen angegeben werden, werden alle Zählungen (Zeilen, Wörter und Zeichen) angezeigt.

### Beispiele:

1. Zähle Zeilen, Wörter und Zeichen in einer Datei:
   ```
   ./wc beispiel.txt
   ```

2. Zähle nur Wörter in mehreren Dateien:
   ```
   ./wc -w datei1.txt datei2.txt datei3.txt
   ```

3. Zähle Zeilen und Zeichen aus der Standardeingabe:
   ```
   echo "Hallo, Welt!" | ./wc -l -c
   ```

4. Zähle alles in Dateien mit verschiedenen Sprachen:
   ```
   ./wc englisch.txt koreanisch.txt japanisch.txt
   ```

## 🧪 Tests ausführen

Um die Testsuite auszuführen, verwenden Sie den folgenden Befehl:
```
cargo test
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