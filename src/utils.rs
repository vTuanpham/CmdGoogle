use crate::restore_terminal;
use std::panic;
use unicode_segmentation::UnicodeSegmentation;

pub fn setup_panic_hook() {
    panic::set_hook(Box::new(|_info| {
        let _ = restore_terminal();
    }));
}

pub trait StringExt {
    fn grapheme_len(&self) -> usize;
}

impl StringExt for String {
    fn grapheme_len(&self) -> usize {
        self.graphemes(true).count()
    }
}
