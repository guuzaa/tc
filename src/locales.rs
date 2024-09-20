use sys_locale;

pub fn setup_localization() {
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en"));
    rust_i18n::set_locale(&locale);
}
