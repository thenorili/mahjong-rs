#![allow(dead_code)]

// #![feature(const_char_convert)]
// #![feature(const_option)]

pub mod tiles {
    /*
     * Mahjong is a tile game. This module seeks to provide
     * data structures, constants, and functions
     * for dealing with a set of riichi mahjong tiles with
     * Rust code, unicode, and text at least.
     *
     * Randomization is essential and should certainly be
     * handled by a dependency.
     *
     * Unicode doesn't support red tiles currently -- let's do this
     * without red tiles for now.
     *
     * The Unicode indexing of tiles will be used throughout.
     */

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

    #[derive(Clone)]
    struct TileId {
        suit: Suit;
        face: u8;
    }

    #[derive(Clone)]
    struct Tile_Info {
        // name might unnecessary...
        // is only kinda useful for honors, and then can be handled other ways
        name: &'static str;
        unicode: char;
        suit: char;
        face: u8;
    }

    // lnori 2022-12-12
    // this could be put in an impl block with related constants and functions
    static MJ_TILES: phf::Map<u8, Tile_Info> = {
        // Man 1-9
        0x01  => { "1m", '\u{1F007}', 'm', 1},
        0x02  => { "2m", '\u{1F008}', 'm', 2},
        0x03  => { "3m", '\u{1F009}', 'm', 3},
        0x04  => { "4m", '\u{1F00A}', 'm', 4},
        0x05  => { "5m", '\u{1F00B}', 'm', 5},
        // fixme -- indicate these are red somehow
        0x105 => { "5m", '\u{1F00B}', 'm', 5},
        0x06  => { "6m", '\u{1F00C}', 'm', 6},
        0x07  => { "7m", '\u{1F00D}', 'm', 7},
        0x08  => { "8m", '\u{1F00E}', 'm', 8},
        0x09  => { "9m", '\u{1F00F}', 'm', 9},
        // Pin 1-9
        0x11  => { "1p", '\u{1F019}', 'p', 1},
        0x12  => { "2p", '\u{1F01A}', 'p', 2},
        0x13  => { "3p", '\u{1F01B}', 'p', 3},
        0x14  => { "4p", '\u{1F01C}', 'p', 4},
        0x15  => { "5p", '\u{1F01D}', 'p', 5},
        // fixme -- indicate these are red somehow
        0x115 => { "5p", '\u{1F01D}', 'p', 5},
        0x16  => { "6p", '\u{1F01E}', 'p', 6},
        0x17  => { "7p", '\u{1F01F}', 'p', 7},
        0x18  => { "8p", '\u{1F020}', 'p', 8},
        0x19  => { "9p", '\u{1F021}', 'p', 9},
        // Sou 1-9
        0x21  => { "1s", '\u{1F010}', 's', 1},
        0x22  => { "2s", '\u{1F011}', 's', 2},
        0x23  => { "3s", '\u{1F012}', 's', 3},
        0x24  => { "4s", '\u{1F013}', 's', 4},
        0x25  => { "5s", '\u{1F014}', 's', 5},
        // fixme -- indicate these are red somehow
        0x125 => { "5s", '\u{1F014}', 's', 5},
        0x26  => { "6s", '\u{1F015}', 's', 6},
        0x27  => { "7s", '\u{1F016}', 's', 7},
        0x28  => { "8s", '\u{1F017}', 's', 8},
        0x29  => { "9s", '\u{1F018}', 's', 9},
        // Honors
        0x31 => { "East Wind",  '\u{1F000}', 'z' , 1},
        0x41 => { "South Wind", '\u{1F001}', 'z', 2 },
        0x51 => { "West Wind", '\u{1F002}', 'z', 3},
        0x61 => { "North Wind", '\u{1F003}', 'z', 4},
        0x71 => { "Red Dragon", '\u{1F004}', 'z', 5},
        0x81 => { "Green Dragon", '\u{1F005}', 'z', 6},
        0x91 => { "White Dragon", '\u{1F006}', 'z', 7},
    };

    const fn TileCode_To_TileId (code: u8) -> TileId
    {
        // FIXME: can't use get(code) in constant expression? investigate
        const suit: Suit = Suit(MJ_TILES.get(code).suit);
        const face: u8 = MJ_TILES.get(code).face;
        return TileId(suit, face);
    }

    const fn DoraIndicatorFlow (id: TileId) -> TileId
    {
        const id: TileId = TileCode_To_Tileid(id);
        if id.suit == Suit::Honor {
            let const u8 face = match id.face {
                // E->S->W->N->E
                1 => 2,
                2 => 3,
                3 => 4,
                4 => 1,
                // 6G->5R->7W->6G
                5 => 7,
                6 => 5,
                7 => 6,
                _ => panic!("Invalid tile"),
            };
            return const TileId(id.suit, face);
        } else {
            let const u8 face = match id.face {
                1 => 2,
                2 => 3,
                3 => 4,
                4 => 5,
                5 => 6,
                6 => 7,
                7 => 8,
                8 => 9,
                9 => 1,
                _ => panic!(),
            };
            return const TileId(id.suit, face);
        }
    }


    type Mahjong_Tileset = [u8; 136]
    /*
    static MAHJONG_TILESET_UNSORTED_REDFIVE: Mahjong_Tileset = [
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
    */

    static MAHJONG_TILESET_UNSORTED_NORED: Mahjong_Tileset = [
        // 0x01-0x09 1-9m
        0x01, 0x01, 0x01, 0x01,
        0x02, 0x02, 0x02, 0x02,
        0x03, 0x03, 0x03, 0x03,
        0x04, 0x04, 0x04, 0x04,
        0x05, 0x05, 0x05, 0x05,
        0x06, 0x06, 0x06, 0x06,
        0x07, 0x07, 0x07, 0x07,
        0x08, 0x08, 0x08, 0x08,
        0x09, 0x09, 0x09, 0x09,

        // 0x11-0x19 1-9p
        0x11, 0x11, 0x11, 0x11,
        0x12, 0x12, 0x12, 0x12,
        0x13, 0x13, 0x13, 0x13,
        0x14, 0x14, 0x14, 0x14,
        0x15, 0x15, 0x15, 0x15,
        0x16, 0x16, 0x16, 0x16,
        0x17, 0x17, 0x17, 0x17,
        0x18, 0x18, 0x18, 0x18,
        0x19, 0x19, 0x19, 0x19,

        // 0x21-0x29 1-9s
        0x21, 0x21, 0x21, 0x21,
        0x22, 0x22, 0x22, 0x22,
        0x23, 0x23, 0x23, 0x23,
        0x24, 0x24, 0x24, 0x24,
        0x25, 0x25, 0x25, 0x25,
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

    // i don't like that it mismatches the index...
    // this might just be better removed; if not, then justify its existence
    // and add it to the static array at the top.
    //
    // quarantine --
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

    // QUARANTINE END --


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
