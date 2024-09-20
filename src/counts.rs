use crate::cmd::TokenizerModel;

pub struct CountOptions {
    pub show_lines: bool,
    pub show_words: bool,
    pub show_chars: bool,
    pub show_tokens: bool,
    pub tokenizer_model: TokenizerModel,
}

impl CountOptions {
    pub fn count_enabled_options(&self) -> u8 {
        self.show_lines as u8
            + self.show_words as u8
            + self.show_chars as u8
            + self.show_tokens as u8
    }
}

#[derive(Default)]
pub struct InputCounts {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub tokens: usize,
}

impl std::ops::AddAssign for InputCounts {
    fn add_assign(&mut self, other: Self) {
        self.lines += other.lines;
        self.words += other.words;
        self.chars += other.chars;
        self.tokens += other.tokens;
    }
}
