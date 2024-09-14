# 📊 Rust 토큰 카운트 프로그램 (tc) 🦀

Rust로 작성된 간단하고 효율적인 토큰 카운트 프로그램입니다! 🚀

[English](../README.md) | [简体中文](README-zh-CN.md) | [繁體中文](README-zh-TW.md) | [日本語](README-ja-JP.md) | 한국어 | [Deutsch](README-de-DE.md)

## 📝 설명

이 Rust로 구현된 고전적인 `wc` (단어 수 세기) 명령줄 도구는 텍스트 파일이나 표준 입력에서 줄 수, 단어 수, 문자 수, 심지어 토큰 수까지 세는 기능을 제공합니다. 빠르고 안정적이며 유니코드를 지원합니다! 🌍✨

## 🎯 기능

- 줄 수 세기 📏
- 단어 수 세기 🔤
- 문자 수 세기 (다중 바이트 유니코드 문자 포함) 🔡
- 다양한 토크나이저 모델을 사용한 토큰 수 세기 🔢
- 여러 파일 처리 📚
- 표준 입력에서 읽기 🖥️
- 다양한 언어 지원 (영어, 한국어, 일본어 등!) 🌐

## 🛠️ 설치

tc를 설치하는 두 가지 방법이 있습니다:

### 옵션 1: 소스에서 설치

1. 시스템에 Rust가 설치되어 있는지 확인하세요. 설치되어 있지 ���다면 [rust-lang.org](https://www.rust-lang.org/tools/install)에서 다운로드하세요 🦀

2. 이 저장소를 클론하세요:
   ```
   git clone https://github.com/guuzaa/tc.git
   cd tc
   ```

3. 프로젝트를 빌드하세요:
   ```
   cargo build --release
   ```

4. 실행 파일은 `target/release/tc`에 위치합니다

### 옵션 2: 미리 빌드된 바이너리 설치

1. tc 저장소의 [Releases 페이지](https://github.com/guuzaa/tc/releases)로 이동하세요.

2. 운영 체제와 아키텍처에 맞는 최신 릴리스를 다운로드하세요.

3. 다운로드한 아카이브를 압축 해제하세요.

4. `tc` 실행 파일을 시스템 PATH에 있는 디렉토리로 이동하세요 (예: Unix 계열 시스템의 경우 `/usr/local/bin`).

5. 이제 터미널 어디에서나 tc를 사용할 수 있습니다!

## 🚀 사용법

### 옵션:

- `-l, --lines`: 줄 수 표시 📏
- `-w, --words`: 단어 수 표시 🔤
- `-c, --chars`: 문자 수 표시 🔡
- `-t, --tokens`: 토큰 수 표시 🔢
- `--model <MODEL>`: 토크나이저 모델 선택 (기본값: gpt3)

사용 가능한 모델:
- `gpt3`: r50k_base
- `edit`: p50k_edit
- `code`: p50k_base
- `chatgpt`: cl100k_base
- `gpt4o`: o200k_base

옵션을 지정하지 않으면 모든 수 (줄 수, 단어 수, 문자 수, ���큰 수)가 표시됩니다.

### 예시:

1. 파일의 줄 수, 단어 수, 문자 수 세기:
   ```
   tc example.txt
   ```

2. 여러 파일의 단어 수만 세기:
   ```
   tc -w file1.txt file2.txt file3.txt
   ```

3. 표준 입력에서 줄 수와 문자 수 세기:
   ```
   echo "안녕하세요, 세상!" | tc -lc
   ```

4. ChatGPT 토크나이저를 사용하여 토큰 수 세기:
   ```
   tc -t --model chatgpt example.txt
   ```

5. 다양한 언어로 된 파일의 모든 수 세기:
   ```
   tc english.txt korean.txt japanese.txt
   ```

## 🤝 기여

기여는 언제나 환영합니다! 문제를 보고하거나 풀 리퀘스트를 제출해 주세요. 🎉

## 📜 라이선스

이 프로젝트는 MIT 라이선스 하에 라이선스가 부여되었습니다. 자세한 내용은 [LICENSE](../LICENSE) 파일을 참조하세요. 📄

## 🙏 감사의 말

- 놀라운 도구와 지원을 제공해 주신 Rust 커뮤니티에 감사드립니다 🦀❤️
- 영감을 준 원래의 Unix `wc` 명령어에 감사드립니다 🖥️
- Cursor 에디터에 감사드립니다 🤖

즐거운 카운팅 되세요! 🎉📊🚀