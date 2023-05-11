use crate::encoder;
use crate::string_builder::StringBuilder;

pub fn create_string_extractor(
    reader: impl IntoIterator<Item = u8>,
) -> impl Iterator<Item = String> {
    StringExtractor::new(reader.into_iter()).flatten()
}

struct StringExtractor<T: Iterator<Item = u8>> {
    reader: T,
    previous_byte: u8,
    string_builders: Vec<StringBuilder>, // one for each bit-offset, so 8 in total
}

impl<T: Iterator<Item = u8>> StringExtractor<T> {
    fn new(mut reader: T) -> Self {
        let mut string_builders: Vec<StringBuilder> =
            (0..8).map(|_| StringBuilder::default()).collect();

        let byte = reader.next().unwrap_or_default();
        string_builders[0].handle_new_byte(byte);

        Self {
            reader,
            previous_byte: byte,
            string_builders,
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for StringExtractor<T> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let byte = self.reader.next();
            if byte.is_none() {
                let strings: Vec<String> = self
                    .string_builders
                    .iter_mut()
                    .filter_map(|string_builder| string_builder.build_string_if_possible())
                    .collect();
                return match strings.is_empty() {
                    true => None,
                    false => Some(strings),
                };
            }

            let strings: Vec<String> = self
                .string_builders
                .iter_mut()
                .enumerate()
                .filter_map(|(index, string_builder)| {
                    let byte = encoder::decode_byte([self.previous_byte, byte.unwrap()], index);
                    string_builder.handle_new_byte(byte)
                })
                .collect();

            self.previous_byte = byte.unwrap();

            if !strings.is_empty() {
                return Some(strings);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[test]
    fn empty_input() {
        //given
        let input = [];

        //when
        let extractor = create_string_extractor(input);
        let output: Vec<String> = extractor.collect();

        //then
        assert!(output.is_empty());
    }

    #[test]
    fn single_ascii_byte_input() {
        //given
        let input = [0x42];

        //when
        let extractor = create_string_extractor(input);
        let output: Vec<String> = extractor.collect();

        //then
        assert_eq!(vec!["B"], output);
    }

    #[test]
    fn single_non_ascii_byte_input() {
        //given
        let input = [0x04];

        //when
        let extractor = create_string_extractor(input);
        let output: Vec<String> = extractor.collect();

        //then
        assert!(output.is_empty());
    }

    #[parameterized(offset = {0, 1, 2, 3, 4, 5, 6, 7})]
    fn two_ascii_bytes_at_the_start() {
        // TODO
    }

    #[parameterized(offset = {0, 1, 2, 3, 4, 5, 6, 7})]
    fn two_ascii_bytes_in_the_middle() {
        // TODO
    }

    #[parameterized(offset = {0, 1, 2, 3, 4, 5, 6, 7})]
    fn two_ascii_bytes_at_the_end() {
        // TODO
    }
}
