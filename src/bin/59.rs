/* Problem 59: XOR decryption
 *
 * Each character on a computer is assigned a unique code and the preferred standard is ASCII
 * (American Standard Code for Information Interchange). For example, uppercase A = 65, asterisk (*)
 * = 42, and lowercase k = 107.
 *
 * A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte
 * with a given value, taken from a secret key. The advantage with the XOR function is that using
 * the same encryption key on the cipher text, restores the plain text; for example, 65 XOR 42 =
 * 107, then 107 XOR 42 = 65.
 *
 * For unbreakable encryption, the key is the same length as the plain text message, and the key is
 * made up of random bytes. The user would keep the encrypted message and the encryption key in
 * different locations, and without both "halves", it is impossible to decrypt the message.
 *
 * Unfortunately, this method is impractical for most users, so the modified method is to use a
 * password as a key. If the password is shorter than the message, which is likely, the key is
 * repeated cyclically throughout the message. The balance for this method is using a sufficiently
 * long password key for security, but short enough to be memorable.
 *
 * Your task has been made easy, as the encryption key consists of three lower case characters.
 * Using cipher.txt, a file containing the encrypted ASCII codes, and the knowledge that the plain
 * text must contain common English words, decrypt the message and find the sum of the ASCII values
 * in the original text. */

#![feature(slicing_syntax)]

use std::str::from_str;

const CIPHER: &'static str = include_str!("../../data/59-cipher.txt");
const KEY_LEN: uint = 3;
const COMMON_WORDS: &'static [&'static str] = &["the", "be", "to", "of", "and"];

fn main() {
    let cipher: Vec<u8> = CIPHER.split(',')
        .map(|num| from_str(num.trim_right_chars('\n')).unwrap())
        .collect();

    let mut buffer = Vec::with_capacity(cipher.len());
    let mut key_gen = KeysGenerator::new();

    loop {
        let key = key_gen.next().expect("No more keys to try!");
        buffer.clear();

        let decryptor = Decryptor::new(cipher[], key);
        buffer.extend(decryptor);

        let buffer_slice = buffer[];
        let string = buffer_slice.as_str_ascii();

        if COMMON_WORDS.iter().all(|&word| string.contains(word)) {
            break;
        }
    }

    let result: u32 = buffer[].iter().fold(0, |total, chr| {
       total + (chr.as_byte() as u32)
    });

    println!("{}", result);
}

struct Decryptor<'a> {
    source: &'a [u8],
    key: &'a [LowerCaseCharacter],

    position: uint,
}

impl<'a> Decryptor<'a> {
    fn new<'a>(source: &'a [u8], key: &'a [LowerCaseCharacter]) -> Decryptor<'a> {
        Decryptor { source: source, key: key, position: 0 }
    }
}

impl<'a> Iterator<Ascii> for Decryptor<'a> {
    fn next(&mut self) -> Option<Ascii> {
        let pos = self.position;

        self.source.get(pos).map(|&value| {
            let result = value ^ self.key[pos % KEY_LEN].byte;
            self.position += 1;

            result.to_ascii()
        })
    }
}

struct LowerCaseCharacter {
    byte: u8
}

const FIRST_LOWERCASE: u8 = b'a';
const LAST_LOWERCASE:  u8 = b'z';

impl LowerCaseCharacter {
    fn from_byte(byte: u8) -> LowerCaseCharacter {
        if byte < FIRST_LOWERCASE || byte > LAST_LOWERCASE {
            panic!("Invalid byte: {}", byte);
        } else {
            LowerCaseCharacter { byte: byte }
        }
    }

    fn first() -> LowerCaseCharacter {
        LowerCaseCharacter::from_byte(FIRST_LOWERCASE)
    }

    fn next(&self) -> Option<LowerCaseCharacter> {
        if self.byte == LAST_LOWERCASE {
            None
        } else {
            Some(LowerCaseCharacter::from_byte(self.byte + 1))
        }
    }
}

struct KeysGenerator {
    last: Option<[LowerCaseCharacter, ..KEY_LEN]>
}

impl KeysGenerator {
    fn new() -> KeysGenerator {
        KeysGenerator { last: None }
    }

    fn next<'a>(&'a mut self) -> Option<&'a [LowerCaseCharacter]> {
        let last = match self.last {
            Some(ref mut last) => last,

            None => {
                self.last = Some([LowerCaseCharacter::first(), ..KEY_LEN]);
                return self.last.as_ref().map(|result| result.as_slice())
            }
        };

        for index in range(0u, KEY_LEN) {
            match last[index].next() {
                Some(new_value) => {
                    last[index] = new_value;
                    return Some(last.as_slice());
                },

                None => {
                    last[index] = LowerCaseCharacter::first();
                    continue;
                }
            }
        }

        None
    }
}
