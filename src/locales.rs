pub fn setup_localization() {
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en"));
    rust_i18n::set_locale(&locale);
}

#[cfg(test)]
mod tests {
    use rust_i18n::t;

    #[test]
    fn test_localization() {
        super::setup_localization();

        assert_eq!(t!("error_writing_stdout"), "tc: Error writing to stdout");
        assert_eq!(t!("error_reading_stdin"), "tc: Error reading stdin");
        assert_eq!(
            t!(
                "error_reading_file",
                filename = "test.txt",
                error = "Permission denied"
            ),
            "tc: test.txt: Error reading file: Permission denied"
        );
    }

    #[test]
    fn test_japanese_localization() {
        rust_i18n::set_locale("ja");

        assert_eq!(
            t!("error_writing_stdout"),
            "tc: 標準出力に書き込み中にエラーが発生しました"
        );
        assert_eq!(
            t!("error_reading_stdin"),
            "tc: 標準入力の読み込み中にエラーが発生しました"
        );
        assert_eq!(
            t!(
                "error_reading_file",
                filename = "test.txt",
                error = "アクセス拒否"
            ),
            "tc: test.txt: ファイルの読み込み中にエラーが発生しました: アクセス拒否"
        );
        assert_eq!(t!("total"), "合計");
        assert_eq!(
            t!(
                "error_opening_file",
                filename = "データ.txt",
                error = "ファイルが見つかりません"
            ),
            "tc: データ.txt: ファイルを開く際にエラーが発生しました: ファイルが見つかりません"
        );
        assert_eq!(
            t!("error_permission_denied", filename = "機密.doc"),
            "tc: 機密.doc: アクセスが拒否されました"
        );
        assert_eq!(
            t!("error_not_found", filename = "存在しない.txt"),
            "tc: 存在しない.txt: そのようなファイルはありません"
        );

        super::setup_localization();
    }
}
