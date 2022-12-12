#![allow(dead_code)]

// #![feature(const_char_convert)]
// #![feature(const_option)]

pub mod mj {
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
    #[derive(Copy, Clone, Debug, const PartialEq)]
    enum Suit {
        Man    = b'm',
        Pin    = b'p',
        Sou    = b's',
        Honor  = b'z',
    };

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
        suit: Suit,
        face: u8,
    }

    #[derive(Clone)]
    struct TileInfo {
        // name might unnecessary...
        // is only kinda useful for honors, and then can be handled other ways
        name: &'static str,
        unicode: char,
        suit: char,
        face: u8,
    }


    // lnori 2022-12-12
    // this could be put in an impl block with related constants and functions
    static MJ_TILES: phf::Map<u8, TileInfo> = phf_map! {
        // Man 1-9
        01  => { "1m", '\u{1F007}', 'm', 1},
        02  => { "2m", '\u{1F008}', 'm', 2},
        03  => { "3m", '\u{1F009}', 'm', 3},
        04  => { "4m", '\u{1F00A}', 'm', 4},
        05  => { "5m", '\u{1F00B}', 'm', 5},
        // fixme -- indicate these are red somehow
        105 => { "5m", '\u{1F00B}', 'm', 5},
        06  => { "6m", '\u{1F00C}', 'm', 6},
        07  => { "7m", '\u{1F00D}', 'm', 7},
        08  => { "8m", '\u{1F00E}', 'm', 8},
        09  => { "9m", '\u{1F00F}', 'm', 9},
        // Pin 1-9
        11  => { "1p", '\u{1F019}', 'p', 1},
        12  => { "2p", '\u{1F01A}', 'p', 2},
        13  => { "3p", '\u{1F01B}', 'p', 3},
        14  => { "4p", '\u{1F01C}', 'p', 4},
        15  => { "5p", '\u{1F01D}', 'p', 5},
        // fixme -- indicate these are red somehow
        115 => { "5p", '\u{1F01D}', 'p', 5},
        16  => { "6p", '\u{1F01E}', 'p', 6},
        17  => { "7p", '\u{1F01F}', 'p', 7},
        18  => { "8p", '\u{1F020}', 'p', 8},
        19  => { "9p", '\u{1F021}', 'p', 9},
        // Sou 1-9
        21  => { "1s", '\u{1F010}', 's', 1},
        22  => { "2s", '\u{1F011}', 's', 2},
        23  => { "3s", '\u{1F012}', 's', 3},
        24  => { "4s", '\u{1F013}', 's', 4},
        25  => { "5s", '\u{1F014}', 's', 5},
        // fixme -- indicate these are red somehow
        125 => { "5s", '\u{1F014}', 's', 5},
        26  => { "6s", '\u{1F015}', 's', 6},
        27  => { "7s", '\u{1F016}', 's', 7},
        28  => { "8s", '\u{1F017}', 's', 8},
        29  => { "9s", '\u{1F018}', 's', 9},
        // Honors
        31 => { "East Wind",  '\u{1F000}', 'z' , 1},
        41 => { "South Wind", '\u{1F001}', 'z', 2 },
        51 => { "West Wind", '\u{1F002}', 'z', 3},
        61 => { "North Wind", '\u{1F003}', 'z', 4},
        71 => { "Red Dragon", '\u{1F004}', 'z', 5},
        81 => { "Green Dragon", '\u{1F005}', 'z', 6},
        91 => { "White Dragon", '\u{1F006}', 'z', 7},
    };

    const fn TileCode_To_TileId (code: u8) -> TileId
    {
        // FIXME: can't use get(code) in constant expression? investigate
        const tile: TileInfo = MJ_TILES[code];
        const suit: Suit = Suit(tile.suit);
        const face: u8 = tile.face;
        return TileId(suit, face);
    }

    const fn DoraIndicatorFlow (id: TileId) -> TileId
    {
        if id.suit == Suit::Honor {
            const face: u8 = match id.face {
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
            return TileId(id.suit, face);
        } else {
            const face: u8 = match id.face {
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
            return TileId(id.suit, face);
        }
    }


    type MahjongTileset = [u8; 136];
    /*
    static MAHJONG_TILESET_UNSORTED_REDFIVE: MahjongTileset = [
        // 01-09 1-9m
        01, 01, 01, 01,
        02, 02, 02, 02,
        03, 03, 03, 03,
        04, 04, 04, 04,
        05, 05, 05, 105, // 105 0m
        06, 06, 06, 06,
        07, 07, 07, 07,
        08, 08, 08, 08,
        09, 09, 09, 09,

        // 11-19 1-9p
        11, 11, 11, 11,
        12, 12, 12, 12,
        13, 13, 13, 13,
        14, 14, 14, 14,
        15, 15, 15, 115, // 115 0p
        16, 16, 16, 16,
        17, 17, 17, 17,
        18, 18, 18, 18,
        19, 19, 19, 19,

        // 21-29 1-9s
        21, 21, 21, 21,
        22, 22, 22, 22,
        23, 23, 23, 23,
        24, 24, 24, 24,
        25, 25, 25, 125, // 125 0s
        26, 26, 26, 26,
        27, 27, 27, 27,
        28, 28, 28, 28,
        29, 29, 29, 29,

        // honors
        31, 31, 31, 31,
        41, 41, 41, 41,
        51, 51, 51, 51,
        61, 61, 61, 61,
        71, 71, 71, 71,
        81, 81, 81, 81,
        91, 91, 91, 91,
    ];
    */

    static MAHJONG_TILESET_UNSORTED_NORED: MahjongTileset = [
        // 01-09 1-9m
        01, 01, 01, 01,
        02, 02, 02, 02,
        03, 03, 03, 03,
        04, 04, 04, 04,
        05, 05, 05, 05,
        06, 06, 06, 06,
        07, 07, 07, 07,
        08, 08, 08, 08,
        09, 09, 09, 09,

        // 11-19 1-9p
        11, 11, 11, 11,
        12, 12, 12, 12,
        13, 13, 13, 13,
        14, 14, 14, 14,
        15, 15, 15, 15,
        16, 16, 16, 16,
        17, 17, 17, 17,
        18, 18, 18, 18,
        19, 19, 19, 19,

        // 21-29 1-9s
        21, 21, 21, 21,
        22, 22, 22, 22,
        23, 23, 23, 23,
        24, 24, 24, 24,
        25, 25, 25, 25,
        26, 26, 26, 26,
        27, 27, 27, 27,
        28, 28, 28, 28,
        29, 29, 29, 29,

        // honors
        31, 31, 31, 31,
        41, 41, 41, 41,
        51, 51, 51, 51,
        61, 61, 61, 61,
        71, 71, 71, 71,
        81, 81, 81, 81,
        91, 91, 91, 91,
    ];

    // i don't like that it mismatches the index...
    // this might just be better removed; if not, then justify its existence
    // and add it to the static array at the top.
    //
    // quarantine --
    const fn try_tilecode_to_tile(code: TileCode) -> Result<Tile, ()> {
        let val = code as usize;
        let m: Suit = Suit::Man;
        let p: Suit = Suit::Pin;
        let s: Suit = Suit::Sou;
        let z: Suit = Suit::Honor;

        let result = match val  {
            105 => Tile { suit: m, face: 0 },
            01 => Tile { suit: m, face: 1 },
            02 => Tile { suit: m, face: 2 },
            03 => Tile { suit: m, face: 3 },
            04 => Tile { suit: m, face: 4 },
            05 => Tile { suit: m, face: 5 },
            06 => Tile { suit: m, face: 6 },
            07 => Tile { suit: m, face: 7 },
            08 => Tile { suit: m, face: 8 },
            09 => Tile { suit: m, face: 9 },

            115 => Tile { suit: p, face: 0 },
            11 => Tile { suit: p, face: 1 },
            12 => Tile { suit: p, face: 2 },
            13 => Tile { suit: p, face: 3 },
            14 => Tile { suit: p, face: 4 },
            15 => Tile { suit: p, face: 5 },
            16 => Tile { suit: p, face: 6 },
            17 => Tile { suit: p, face: 7 },
            18 => Tile { suit: p, face: 8 },
            19 => Tile { suit: p, face: 9 },

            125 => Tile { suit: s, face: 0 },
            21 => Tile { suit: s, face: 1 },
            22 => Tile { suit: s, face: 2 },
            23 => Tile { suit: s, face: 3 },
            24 => Tile { suit: s, face: 4 },
            25 => Tile { suit: s, face: 5 },
            26 => Tile { suit: s, face: 6 },
            27 => Tile { suit: s, face: 7 },
            28 => Tile { suit: s, face: 8 },
            29 => Tile { suit: s, face: 9 },

            31 => Tile { suit: z, face: 1 },
            41 => Tile { suit: z, face: 2 },
            51 => Tile { suit: z, face: 3 },
            61 => Tile { suit: z, face: 4 },
            71 => Tile { suit: z, face: 5 },
            81 => Tile { suit: z, face: 6 },
            91 => Tile { suit: z, face: 7 },
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
        // 01-09 1-9m
        01, 01, 01, 01,
        02, 02, 02, 02,
        03, 03, 03, 03,
        04, 04, 04, 04,
        05, 05, 05, 105, // 105 0m
        06, 06, 06, 06,
        07, 07, 07, 07,
        08, 08, 08, 08,
        09, 09, 09, 09,

        // 11-19 1-9p
        11, 11, 11, 11,
        12, 12, 12, 12,
        13, 13, 13, 13,
        14, 14, 14, 14,
        15, 15, 15, 115, // 115 0p
        16, 16, 16, 16,
        17, 17, 17, 17,
        18, 18, 18, 18,
        19, 19, 19, 19,

        // 21-29 1-9s
        21, 21, 21, 21,
        22, 22, 22, 22,
        23, 23, 23, 23,
        24, 24, 24, 24,
        25, 25, 25, 125, // 125 0s
        26, 26, 26, 26,
        27, 27, 27, 27,
        28, 28, 28, 28,
        29, 29, 29, 29,

        // honors
        31, 31, 31, 31,
        41, 41, 41, 41,
        51, 51, 51, 51,
        61, 61, 61, 61,
        71, 71, 71, 71,
        81, 81, 81, 81,
        91, 91, 91, 91,
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
