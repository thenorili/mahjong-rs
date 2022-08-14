#![allow(dead_code)]

// #![feature(const_char_convert)]
// #![feature(const_option)]

pub mod tiles {
    /*
    enum Winds {
        EastWind,
        SouthWind,
        WestWind,
        NorthWind,
    }
    enum Dragons {
        WhiteDragon,
        GreenDragon,
        RedDragon,
    }
    enum Honor {
        Dragon,
        Wind,
    }
    struct SuitTile {
        suit: Suit,
        // red 5s are 0m, 0s, 0p
        num: usize,
    }
    */
    // maybe just use b'm', s, p, z instead of int!
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Suit {
        Man    = b'm',
        Pin    = b'p',
        Sou    = b's',
        Honor  = b'z',
    }
    impl From<char> for Suit {
        fn from(c: char) -> Suit {
            match c {
                'm' => Suit::Man,
                'p' => Suit::Pin,
                's' => Suit::Sou,
                'z' => Suit::Honor,
                _ => panic!(),
            }
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<char> for Suit {
        fn into(self) -> char {
            self as u8 as char
        }
    }

    impl Suit {
        fn try_from(c: char) -> Result<Self, ()> {
            match c {
                'm' => Ok(Suit::Man),
                'p' => Ok(Suit::Pin),
                's' => Ok(Suit::Sou),
                'z' => Ok(Suit::Honor),
                _ => Err(()),
            }
        }
    }


    impl std::str::FromStr for Suit {
        type Err = ();

        fn from_str(s: &str) -> Result<Suit, ()> {
            match s {
                "m" => Ok(Suit::Man),
                "s" => Ok(Suit::Sou),
                "p" => Ok(Suit::Pin),
                "z" => Ok(Suit::Honor),
                _ => Err(()),
            }
        }
    }

    // so if it's a part of the way the tiles are expressed
    // Unicode doesn't support red tiles currently,
    // the 'redness' of the tiles needs to be tagged separately
    static MAHJONG_TILES_UNICODE: [char; 34] = [
        // East Wind
        '\u{1F000}',
        // South Wind
        '\u{1F001}',
        // West Wind
        '\u{1F002}',
        // North Wind
        '\u{1F003}',
        // Red Dragon
        '\u{1F004}',
        // Green Dragon
        '\u{1F005}',
        // White Dragon
        '\u{1F006}',
        // Man 1-9
        '\u{1F007}',
        '\u{1F008}',
        '\u{1F009}',
        '\u{1F00A}',
        '\u{1F00B}',
        '\u{1F00C}',
        '\u{1F00D}',
        '\u{1F00E}',
        '\u{1F00F}',
        // Sou 1-9
        '\u{1F010}',
        '\u{1F011}',
        '\u{1F012}',
        '\u{1F013}',
        '\u{1F014}',
        '\u{1F015}',
        '\u{1F016}',
        '\u{1F017}',
        '\u{1F018}',
        // Pin 1-9
        '\u{1F019}',
        '\u{1F01A}',
        '\u{1F01B}',
        '\u{1F01C}',
        '\u{1F01D}',
        '\u{1F01E}',
        '\u{1F01F}',
        '\u{1F020}',
        '\u{1F021}',
    ];
    enum TileCode {
        M0 = 0x105,
        M1 = 0x01,
        M2 = 0x02,
        M3 = 0x03,
        M4 = 0x04,
        M5 = 0x05,
        M6 = 0x06,
        M7 = 0x07,
        M8 = 0x08,
        M9 = 0x09,

        P0 = 0x115,
        P1 = 0x11,
        P2 = 0x12,
        P3 = 0x13,
        P4 = 0x14,
        P5 = 0x15,
        P6 = 0x16,
        P7 = 0x17,
        P8 = 0x18,
        P9 = 0x19,

        S0 = 0x125,
        S1 = 0x21,
        S2 = 0x22,
        S3 = 0x23,
        S4 = 0x24,
        S5 = 0x25,
        S6 = 0x26,
        S7 = 0x27,
        S8 = 0x28,
        S9 = 0x29,

        Z1 = 0x31,
        Z2 = 0x41,
        Z3 = 0x51,
        Z4 = 0x61,
        Z5 = 0x71,
        Z6 = 0x81,
        Z7 = 0x91,
    }
    const fn try_tilecode_to_tile(code: TileCode) -> Result<Tile, ()> {
        let val = code as usize;
        let m: Suit = Suit::Man;
        let p: Suit = Suit::Pin;
        let s: Suit = Suit::Sou;
        let z: Suit = Suit::Honor;

        let result = match val  {
            0x105 => Tile { suit: m, face: 0 },
            0x01 => Tile { suit: m, face: 1 },
            0x02 => Tile { suit: m, face: 2 },
            0x03 => Tile { suit: m, face: 3 },
            0x04 => Tile { suit: m, face: 4 },
            0x05 => Tile { suit: m, face: 5 },
            0x06 => Tile { suit: m, face: 6 },
            0x07 => Tile { suit: m, face: 7 },
            0x08 => Tile { suit: m, face: 8 },
            0x09 => Tile { suit: m, face: 9 },

            0x115 => Tile { suit: p, face: 0 },
            0x11 => Tile { suit: p, face: 1 },
            0x12 => Tile { suit: p, face: 2 },
            0x13 => Tile { suit: p, face: 3 },
            0x14 => Tile { suit: p, face: 4 },
            0x15 => Tile { suit: p, face: 5 },
            0x16 => Tile { suit: p, face: 6 },
            0x17 => Tile { suit: p, face: 7 },
            0x18 => Tile { suit: p, face: 8 },
            0x19 => Tile { suit: p, face: 9 },

            0x125 => Tile { suit: s, face: 0 },
            0x21 => Tile { suit: s, face: 1 },
            0x22 => Tile { suit: s, face: 2 },
            0x23 => Tile { suit: s, face: 3 },
            0x24 => Tile { suit: s, face: 4 },
            0x25 => Tile { suit: s, face: 5 },
            0x26 => Tile { suit: s, face: 6 },
            0x27 => Tile { suit: s, face: 7 },
            0x28 => Tile { suit: s, face: 8 },
            0x29 => Tile { suit: s, face: 9 },

            0x31 => Tile { suit: z, face: 1 },
            0x41 => Tile { suit: z, face: 2 },
            0x51 => Tile { suit: z, face: 3 },
            0x61 => Tile { suit: z, face: 4 },
            0x71 => Tile { suit: z, face: 5 },
            0x81 => Tile { suit: z, face: 6 },
            0x91 => Tile { suit: z, face: 7 },
            _    => return Err(())
        };
        Ok(result)
    }
    const fn tilecode_to_tile(code: TileCode) -> Tile {
        if let Ok(tile) = try_tilecode_to_tile(code) {
            return tile;
        } else {
            panic!();
        }
    }

    static TILESET: [usize; 136] = [
        // 0x01-0x09 1-9m
        0x01, 0x01, 0x01, 0x01,
        0x02, 0x02, 0x02, 0x02,
        0x03, 0x03, 0x03, 0x03,
        0x04, 0x04, 0x04, 0x04,
        0x05, 0x05, 0x05, 0x105, // 0x105 0m
        0x06, 0x06, 0x06, 0x06,
        0x07, 0x07, 0x07, 0x07,
        0x08, 0x08, 0x08, 0x08,
        0x09, 0x09, 0x09, 0x09,

        // 0x11-0x19 1-9p
        0x11, 0x11, 0x11, 0x11,
        0x12, 0x12, 0x12, 0x12,
        0x13, 0x13, 0x13, 0x13,
        0x14, 0x14, 0x14, 0x14,
        0x15, 0x15, 0x15, 0x115, // 0x115 0p
        0x16, 0x16, 0x16, 0x16,
        0x17, 0x17, 0x17, 0x17,
        0x18, 0x18, 0x18, 0x18,
        0x19, 0x19, 0x19, 0x19,

        // 0x21-0x29 1-9s
        0x21, 0x21, 0x21, 0x21,
        0x22, 0x22, 0x22, 0x22,
        0x23, 0x23, 0x23, 0x23,
        0x24, 0x24, 0x24, 0x24,
        0x25, 0x25, 0x25, 0x125, // 0x125 0s
        0x26, 0x26, 0x26, 0x26,
        0x27, 0x27, 0x27, 0x27,
        0x28, 0x28, 0x28, 0x28,
        0x29, 0x29, 0x29, 0x29,

        // honors
        0x31, 0x31, 0x31, 0x31,
        0x41, 0x41, 0x41, 0x41,
        0x51, 0x51, 0x51, 0x51,
        0x61, 0x61, 0x61, 0x61,
        0x71, 0x71, 0x71, 0x71,
        0x81, 0x81, 0x81, 0x81,
        0x91, 0x91, 0x91, 0x91,
    ];

    /*
    type mj_notation = &'static
    static MAHJONG_TILES_NOTATION: [&'static str; 37] = [

    ]
    */
    #[derive(Copy, Clone)]
    struct UnicodeTile {
        tile: char,
        // could use ANSI escape code to render red fives!
        // red='\E[31;40mRED_TEXT_BLACK_BG'
        // not totally portable unfortunately...
        red: bool,
    }
    #[derive(Copy, Clone)]
    struct Tile {
        suit: Suit,
        face: usize,
    }
    const fn tile_is_valid(tile: Tile) -> bool {
        match tile.suit {
            Suit::Honor => tile.face >= 1 && tile.face <= 7,
            _ => tile.face <= 9,
        }
    }

    // TODO: just use try_into here, then uncomment
    // not really high priority work tbh... maybe back off and plan work
    /*
    // Maybe error handling would be good!!!
    fn tile_from_notation(txt: &'static str) -> Tile {
        let letters: Vec<char> = txt.chars().take(2).collect();
        let suit = match letters[0] {
            'm' => 'm',
            's' => 's',
            'p' => 'p',
            'z' => 'z',
            _   => panic!(),
        };
        let face = letters[1].to_digit(/*RADIX*/ 10).unwrap();
        assert!(face <= 9);
        return Tile { suit, face: face as usize };
    }
    */

    // This whole thing should just be a static set of data...
    // but if it's const it should be fine, as long as it's more readable
    // than a static array... idk! lol! this was probably a silly exercise ^^

    // really think it would be more readable and checkable to just have a static
    // array of e.g. 'm1' or 'm', '1'... then parse that into something more meaningful
    // This seems overwrought and silly.
    const fn new_tileset(red_fives: bool) -> [Tile; 136] {
        let mut tileset: [Tile; 136] = [ Tile { face: 666, suit: Suit::Man }; 136 ];
        let mut index: usize = 0;
        let mut suit_iter: usize = 0;
        // max value of enum Suit
        let enum_honors = 3;
        while suit_iter <= enum_honors {
            let mut copy: usize = 0;
            let suit: Suit = match suit_iter {
                0 => Suit::Man,
                1 => Suit::Pin,
                2 => Suit::Sou,
                3 => Suit::Honor,
                _ => panic!(),
            };
            if suit_iter < enum_honors {
                while copy < 4 {
                    let mut face: usize = 1;
                    while face <= 9 {
                        if copy == 0 && face == 5 && red_fives {
                            tileset[index] = Tile { face: 0, suit };
                        } else {
                            tileset[index] = Tile { face, suit };
                        }
                        index += 1;
                        face += 1;
                    }
                    copy += 1;
                }
            } else {
                while copy < 4 {
                    let mut face: usize = 1;
                    while face <= 7 {
                        tileset[index] = Tile { face, suit };
                        index += 1;
                        face += 1;
                    }
                    copy += 1;
                }
            }
            suit_iter += 1;
        }
        tileset
    }
    #[test]
    fn tileset_initialized() {
        let tileset = new_tileset(/* red_fives */true);
        for tile in tileset {
            // at the beginning of new_tileset, the face values were mangled...
            // so if tile_is_valid returns true for each tile, we've at least touched each tile.
            assert!(tile_is_valid(tile));
        }
    }
    // TODO: use hashmap to test that the correct number of each tiles were created
    //          maybe just test against static data
    //          still pretty peeved that I put a lot of time into a nothing burger =-=
    // Q: should I just process the red fives as fives at some level? which level?
    //      just separate parameter 'face_value'? or special logic?
    // TODO: sorting (will make testing tileset correctness trivial)
    // TODO: shuffling
    // TODO: create walls
    // TODO: deal to hands from walls
    // TODO: discard from hands to discard pile
    //
    // TODO: parse hands into tile groups for chii/pon/kan
    // TODO: organize this ad-hoc planning into a real project plan C:
    // TODO: detect yaku
    // TODO: detect tenpai
    // TODO: calculate tiles-away-from-tenpai
    // TODO: test that compares const fn to TILESET
    //          smh at redundancy
}