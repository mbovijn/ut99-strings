use std::str;

#[derive(Default)]
pub struct StringBuilder {
    characters: Vec<u8>,
}

impl StringBuilder {
    pub fn handle_new_byte(&mut self, byte: u8) -> Option<String> {
        if StringBuilder::is_printable_ascii(byte) {
            self.characters.push(byte);
            None
        } else {
            self.build_string_if_possible()
        }
    }

    pub fn build_string_if_possible(&mut self) -> Option<String> {
        if !self.characters.is_empty() {
            let string = str::from_utf8(&self.characters).unwrap().to_owned();
            self.characters.clear();
            Some(string)
        } else {
            None
        }
    }

    fn is_printable_ascii(byte: u8) -> bool {
        byte.is_ascii() && !byte.is_ascii_control()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_accept_non_ascii_bytes() {
        //given
        let mut builder = StringBuilder::default();

        //when
        let string = builder.handle_new_byte(0x19);

        //then
        assert_eq!(builder.characters.len(), 0);
        assert!(string.is_none());
    }

    #[test]
    fn should_accept_ascii_bytes() {
        //given
        let mut builder = StringBuilder::default();

        //when
        let string = builder.handle_new_byte(0x42);

        //then
        assert_eq!(builder.characters.len(), 1);
        assert!(string.is_none());
    }

    #[test]
    fn should_return_string_if_non_ascii_byte_after_ascii_byte() {
        //given
        let mut builder = StringBuilder::default();
        builder.handle_new_byte(0x42);

        //when
        let string = builder.handle_new_byte(0x00);

        //then
        assert_eq!(builder.characters.len(), 0);
        assert_eq!(string, Some("B".to_owned()));
    }

    #[test]
    fn should_not_return_string_if_two_non_ascii_bytes_in_a_row() {
        //given
        let mut builder = StringBuilder::default();
        builder.handle_new_byte(0x00);

        //when
        let string = builder.handle_new_byte(0x00);

        //then
        assert_eq!(builder.characters.len(), 0);
        assert!(string.is_none());
    }
}
