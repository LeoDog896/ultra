use std::iter::FromIterator;

use super::{CharIndex, ToChar};

#[derive(Clone, Debug)]
pub struct Rotor {
    mapping: Vec<char>,
    inverse: Vec<char>,
    notches: Vec<usize>,
    pub offset: usize,
}

impl Rotor {
    /// Creates a new `Rotor`, where `mapping` is a 26-character `&str`
    /// containing some ordering of all letters in the alphabet, and
    /// `notches` is a `&str` where each character in the string
    /// corresponds to a single notch in the rotor.
    ///
    /// For a mapping beginning with "EKM", the rotor would map `A` to
    /// `E`, `B` to `K`, and so forth. If the rotor is advanced once,
    /// `A` would be mapped to `K`, and `B` would be mapped to `M`.
    pub fn new(mapping: &str, notches: &str) -> Rotor {
        let mapping: Vec<char> = mapping.chars().collect();

        if mapping.len() != 26 {
            panic!("Rotor mappings must be 26 characters long.");
        }

        let mut inverse = vec!['A'; 26];

        for (i, &c) in mapping.iter().enumerate() {
            inverse[c.index() % 26] = i.to_char();
        }

        Rotor {
            mapping: mapping,
            inverse: inverse,
            notches: Vec::from_iter(notches.chars().map(|c| c.index())),
            offset: 0,
        }
    }

    /// Returns the substitution of a given character
    /// based on the current offset of the rotor.
    pub fn substitute(&self, c: char) -> char {
        self.mapping[(c.index() + self.offset) % 26]
    }

    /// Returns the substitution of a given character when run through
    /// the rotor in reverse (on the path back from the reflector).
    pub fn invert(&self, c: char) -> char {
        let index = self.inverse[c.index()].index();
        ((index + 26 - self.offset) % 26).to_char()
    }

    /// Advances this rotor one position.
    pub fn advance(&mut self) {
        self.offset = (self.offset + 1) % 26;
    }

    /// Returns true if the rotor is currently in a notch position.
    pub fn notch_position(&self) -> bool {
        self.notches.iter().any(|&n| n == self.offset)
    }
}


#[cfg(test)]
mod tests {
    use super::Rotor;

    #[test]
    fn char_substitution() {
        let rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "A");
        assert_eq!(rotor.substitute('A'), 'E');
        assert_eq!(rotor.substitute('B'), 'K');
    }

    #[test]
    fn step_rotor() {
        // Initialize
        let mut rotor = Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "B");
        assert_eq!(rotor.substitute('A'), 'A');

        // Step the rotor one position
        rotor.advance();
        assert_eq!(rotor.offset, 1);
        assert_eq!(rotor.substitute('A'), 'B');

        // Moving from B to C should advance the next rotor
        rotor.advance();
        assert_eq!(rotor.offset, 2);
        assert_eq!(rotor.substitute('A'), 'C');
    }

    #[test]
    fn inverse_mapping() {
        // Rotor I of the Enigma
        let rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "A");
        let inverse: String = rotor.inverse.into_iter().collect();
        assert_eq!(&inverse, "UWYGADFPVZBECKMTHXSLRINQOJ");
    }

    #[test]
    fn matching_inverses() {
        let mut rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "B");
        for i in 65u8..91u8 {
            let c = i as char;
            assert_eq!(c, rotor.invert(rotor.substitute(c)));
            rotor.advance();
        }
    }

    #[test]
    fn step_inverse() {
        let mut rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "B");
        assert_eq!(rotor.invert('E'), 'A');
        rotor.advance();
        assert_eq!(rotor.invert('K'), 'A');
        rotor.advance();
        assert_eq!(rotor.invert('M'), 'A');
    }

    #[test]
    #[should_panic(expected = "Rotor mappings must be 26 characters long.")]
    fn invalid_rotor() {
        Rotor::new("ABC", "A");
    }
}
