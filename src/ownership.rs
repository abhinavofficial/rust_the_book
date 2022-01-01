pub mod ownership {
    pub fn string_slicing(which_word: u8, string: &str) -> &str {
        let bytes = string.as_bytes();
        let mut word_counter: u8 = 0;
        let mut prev_word_index: usize = 0;
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                word_counter = word_counter + 1;
                if word_counter == which_word {
                    return &string[prev_word_index..i];
                } else {
                    prev_word_index = i;
                }
            }
        }
        // If space is not fetched it means this is the first or last word. We have reached the end of string. This is final string
        word_counter = word_counter + 1;
        if word_counter == which_word {
            return &string[prev_word_index..];
        } else {
            "does not exist"
        }
    }
}
