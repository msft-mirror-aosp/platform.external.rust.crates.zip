//! Convert a string in IBM codepage 437 to UTF-8

/// Trait to convert IBM codepage 437 to the target type
pub trait FromCp437 {
    /// Target type
    type Target;

    /// Function that does the conversion from cp437.
    /// Generally allocations will be avoided if all data falls into the ASCII range.
    #[allow(clippy::wrong_self_convention)]
    fn from_cp437(self) -> Self::Target;
}

impl<'a> FromCp437 for &'a [u8] {
    type Target = ::std::borrow::Cow<'a, str>;

    fn from_cp437(self) -> Self::Target {
        if self.iter().all(|c| *c < 0x80) {
            ::std::str::from_utf8(self).unwrap().into()
        } else {
            self.iter().map(|c| to_char(*c)).collect::<String>().into()
        }
    }
}

impl FromCp437 for Vec<u8> {
    type Target = String;

    fn from_cp437(self) -> Self::Target {
        if self.iter().all(|c| *c < 0x80) {
            String::from_utf8(self).unwrap()
        } else {
            self.into_iter().map(to_char).collect()
        }
    }
}

fn to_char(input: u8) -> char {
    let output = match input {
        0x00..=0x7f => input as u32,
        0x80 => 0x00c7,
        0x81 => 0x00fc,
        0x82 => 0x00e9,
        0x83 => 0x00e2,
        0x84 => 0x00e4,
        0x85 => 0x00e0,
        0x86 => 0x00e5,
        0x87 => 0x00e7,
        0x88 => 0x00ea,
        0x89 => 0x00eb,
        0x8a => 0x00e8,
        0x8b => 0x00ef,
        0x8c => 0x00ee,
        0x8d => 0x00ec,
        0x8e => 0x00c4,
        0x8f => 0x00c5,
        0x90 => 0x00c9,
        0x91 => 0x00e6,
        0x92 => 0x00c6,
        0x93 => 0x00f4,
        0x94 => 0x00f6,
        0x95 => 0x00f2,
        0x96 => 0x00fb,
        0x97 => 0x00f9,
        0x98 => 0x00ff,
        0x99 => 0x00d6,
        0x9a => 0x00dc,
        0x9b => 0x00a2,
        0x9c => 0x00a3,
        0x9d => 0x00a5,
        0x9e => 0x20a7,
        0x9f => 0x0192,
        0xa0 => 0x00e1,
        0xa1 => 0x00ed,
        0xa2 => 0x00f3,
        0xa3 => 0x00fa,
        0xa4 => 0x00f1,
        0xa5 => 0x00d1,
        0xa6 => 0x00aa,
        0xa7 => 0x00ba,
        0xa8 => 0x00bf,
        0xa9 => 0x2310,
        0xaa => 0x00ac,
        0xab => 0x00bd,
        0xac => 0x00bc,
        0xad => 0x00a1,
        0xae => 0x00ab,
        0xaf => 0x00bb,
        0xb0 => 0x2591,
        0xb1 => 0x2592,
        0xb2 => 0x2593,
        0xb3 => 0x2502,
        0xb4 => 0x2524,
        0xb5 => 0x2561,
        0xb6 => 0x2562,
        0xb7 => 0x2556,
        0xb8 => 0x2555,
        0xb9 => 0x2563,
        0xba => 0x2551,
        0xbb => 0x2557,
        0xbc => 0x255d,
        0xbd => 0x255c,
        0xbe => 0x255b,
        0xbf => 0x2510,
        0xc0 => 0x2514,
        0xc1 => 0x2534,
        0xc2 => 0x252c,
        0xc3 => 0x251c,
        0xc4 => 0x2500,
        0xc5 => 0x253c,
        0xc6 => 0x255e,
        0xc7 => 0x255f,
        0xc8 => 0x255a,
        0xc9 => 0x2554,
        0xca => 0x2569,
        0xcb => 0x2566,
        0xcc => 0x2560,
        0xcd => 0x2550,
        0xce => 0x256c,
        0xcf => 0x2567,
        0xd0 => 0x2568,
        0xd1 => 0x2564,
        0xd2 => 0x2565,
        0xd3 => 0x2559,
        0xd4 => 0x2558,
        0xd5 => 0x2552,
        0xd6 => 0x2553,
        0xd7 => 0x256b,
        0xd8 => 0x256a,
        0xd9 => 0x2518,
        0xda => 0x250c,
        0xdb => 0x2588,
        0xdc => 0x2584,
        0xdd => 0x258c,
        0xde => 0x2590,
        0xdf => 0x2580,
        0xe0 => 0x03b1,
        0xe1 => 0x00df,
        0xe2 => 0x0393,
        0xe3 => 0x03c0,
        0xe4 => 0x03a3,
        0xe5 => 0x03c3,
        0xe6 => 0x00b5,
        0xe7 => 0x03c4,
        0xe8 => 0x03a6,
        0xe9 => 0x0398,
        0xea => 0x03a9,
        0xeb => 0x03b4,
        0xec => 0x221e,
        0xed => 0x03c6,
        0xee => 0x03b5,
        0xef => 0x2229,
        0xf0 => 0x2261,
        0xf1 => 0x00b1,
        0xf2 => 0x2265,
        0xf3 => 0x2264,
        0xf4 => 0x2320,
        0xf5 => 0x2321,
        0xf6 => 0x00f7,
        0xf7 => 0x2248,
        0xf8 => 0x00b0,
        0xf9 => 0x2219,
        0xfa => 0x00b7,
        0xfb => 0x221a,
        0xfc => 0x207f,
        0xfd => 0x00b2,
        0xfe => 0x25a0,
        0xff => 0x00a0,
    };
    ::std::char::from_u32(output).unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn to_char_valid() {
        for i in 0x00_u32..0x100 {
            super::to_char(i as u8);
        }
    }

    #[test]
    fn ascii() {
        for i in 0x00..0x80 {
            assert_eq!(super::to_char(i), i as char);
        }
    }

    #[test]
    fn example_slice() {
        use super::FromCp437;
        let data = b"Cura\x87ao";
        assert!(::std::str::from_utf8(data).is_err());
        assert_eq!(data.from_cp437(), "Curaçao");
    }

    #[test]
    fn example_vec() {
        use super::FromCp437;
        let data = vec![0xCC, 0xCD, 0xCD, 0xB9];
        assert!(String::from_utf8(data.clone()).is_err());
        assert_eq!(&data.from_cp437(), "╠══╣");
    }
}
