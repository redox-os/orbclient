#![allow(unused)]
use std::u8;
use std::str;
use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use syscall::{Result, Error, ENOENT};

use input::{ModKeys, MOD_LSHIFT, MOD_RSHIFT, MOD_ALT_GR};

const NUM_MOD_COMBOS: usize = 4;
const NUM_SCANCODES: usize = 58;
#[allow(unused)]
pub struct Keymap {
    map: [[char; NUM_MOD_COMBOS]; NUM_SCANCODES],
}

// Basic parsing as as temporary solution
#[allow(unused)]
impl Keymap {
    /// `path` must be a CSV file with separator '\t'.
    pub fn from_file(path: &str) -> Result<Keymap> {
        let mut map = [[0 as char; NUM_MOD_COMBOS]; NUM_SCANCODES];

        let mut string = String::new();
        match File::open(path) {
            Ok(mut file) => match file.read_to_string(&mut string) {
                Ok(_) => {},
                Err(err) => // Could not read to string
                    return Err(Error::new(ENOENT))
            },
            Err(err) => // Could not open file
                return Err(Error::new(ENOENT))
            
        }

        for (i, line) in string.lines().enumerate() {
            if i >= NUM_SCANCODES { break; }
            for (j, element) in line.split('\t').enumerate() {
                if j >= NUM_MOD_COMBOS { break; }
                map[i][j] = to_char(&element.as_bytes());
            }
        }

        Ok(Keymap {
            map: map
        })
    }

    pub fn get_char(&self, keycode: u8, modifiers: ModKeys) -> char {
        self.map[keycode as usize][Keymap::mods_to_index(modifiers)]
    }

    fn mods_to_index(m: ModKeys) -> usize {
        // (alt_gr << 1) | (shift)
        (((m.contains(MOD_ALT_GR) as u8) << 1) | (m.intersects(MOD_LSHIFT | MOD_RSHIFT) as u8)) as usize
    }
}

/// Parse single character from text.
#[allow(unused)]
fn to_char(text: &[u8]) -> char {
    match text.len() {
        1 => text[0] as char,

        2 => match text[1] {
                // Explicit hex string with one digit
                b'0' ... b'9' | b'A' ... b'F' => {
                    u8::from_str_radix(str::from_utf8(&text[1..2]).unwrap_or("0"), 16).unwrap_or(0) as char
                }
                b'n' => '\n',
                b't' => '\t',
                // Quote, single quote or backslash, or some character I haven't yet thought about
                c => c as char,
            },
        3 => {
            u8::from_str_radix(str::from_utf8(&text[1..3]).unwrap_or("0"), 16).unwrap_or(0) as char
        }
        _ => 0 as char,
    }
}


impl Default for Keymap {
    /// 'English' layout.
    fn default() -> Keymap {
        Keymap {
            map: [
                ['\0', '\0', '\0', '\0'],
                ['\x1B', '\x1B', '\0', '\0'],
                ['1', '!', '\0', '\0'],
                ['2', '@', '\0', '\0'],
                ['3', '#', '\0', '\0'],
                ['4', '$', '\0', '\0'],
                ['5', '%', '\0', '\0'],
                ['6', '^', '\0', '\0'],
                ['7', '&', '\0', '\0'],
                ['8', '*', '\0', '\0'],
                ['9', '(', '\0', '\0'],
                ['0', ')', '\0', '\0'],
                ['-', '_', '\0', '\0'],
                ['=', '+', '\0', '\0'],
                ['\x7F', '\x7F', '\0', '\0'],
                ['\t', '\t', '\0', '\0'],
                ['q', 'Q', '\0', '\0'],
                ['w', 'W', '\0', '\0'],
                ['e', 'E', '\0', '\0'],
                ['r', 'R', '\0', '\0'],
                ['t', 'T', '\0', '\0'],
                ['y', 'Y', '\0', '\0'],
                ['u', 'U', '\0', '\0'],
                ['i', 'I', '\0', '\0'],
                ['o', 'O', '\0', '\0'],
                ['p', 'P', '\0', '\0'],
                ['[', '{', '\0', '\0'],
                [']', '}', '\0', '\0'],
                ['\n', '\n', '\0', '\0'],
                ['\0', '\0', '\0', '\0'],
                ['a', 'A', '\0', '\0'],
                ['s', 'S', '\0', '\0'],
                ['d', 'D', '\0', '\0'],
                ['f', 'F', '\0', '\0'],
                ['g', 'G', '\0', '\0'],
                ['h', 'H', '\0', '\0'],
                ['j', 'J', '\0', '\0'],
                ['k', 'K', '\0', '\0'],
                ['l', 'L', '\0', '\0'],
                [';', ':', '\0', '\0'],
                ['\'', '"', '\0', '\0'],
                ['`', '~', '\0', '\0'],
                ['\0', '\0', '\0', '\0'],
                ['\\', '|', '\0', '\0'],
                ['z', 'Z', '\0', '\0'],
                ['x', 'X', '\0', '\0'],
                ['c', 'C', '\0', '\0'],
                ['v', 'V', '\0', '\0'],
                ['b', 'B', '\0', '\0'],
                ['n', 'N', '\0', '\0'],
                ['m', 'M', '\0', '\0'],
                [',', '<', '\0', '\0'],
                ['.', '>', '\0', '\0'],
                ['/', '?', '\0', '\0'],
                ['\0', '\0', '\0', '\0'],
                ['\0', '\0', '\0', '\0'],
                ['\0', '\0', '\0', '\0'],
                [' ', ' ', '\0', '\0']
            ]
        }
    }
}
