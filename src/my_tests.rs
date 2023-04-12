#[cfg(test)]
mod my_tests {
    use crate::*;

    #[test]
    fn char_to_int_works() {
        assert_eq!(char_to_int('A'), 0);
        assert_eq!(char_to_int('Z'), 25);
        assert_eq!(char_to_int('a'), 0);
        assert_eq!(char_to_int('z'), 25);
        assert_eq!(char_to_int('m'), 12);
    }

    #[test]
    fn int_to_char_works() {
        assert_eq!(int_to_char(0, true), 'a');
        assert_eq!(int_to_char(25, false), 'Z');
    }
}