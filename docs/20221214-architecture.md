# 2022-12-14 architecture

## Intro
Storing all the fundamental tile data in static hashmaps indexed to a small, easily-compressed
tile code seems optimal here. The small size makes it quick to randomize and the data structure
permits very efficient lookups so long as we keep our memory footprint small.

phm for perfect hasmaps here seems appropriate as the data size is small and static.

## Tile key/value storage

static hashmap MJ_TILES
    key => u8 (tilecode)
    data => TileInfo where TileInfo = { name, unicode character, suit, face }

- Including the name seems generally useful for the honors and including it as a special case
rather than adding a name field to all tiles seems overly complex.

- Including the associated unicode character is important for one of our early goals, a terminal
implementation of riichi mahjong.
    - Colored unicode would take some extra effort, but it'd be worth it for red fives in terminal

- Currently suit is held as a char that's merely associated with a small enum (repr u8);
while it's a bit quicker to type a char, i don't think we're winning there on memory footprint
or useability -- it's probably best to use the enum directly and exercise its associated methods
to produce a char.

- Face value as a u8 seems fine. Holding numbers as an enum seems more trouble than it's worth,
so better to assert sane values throughout.

## Problems:

- We will almost certainly run faster if we use usize rather than u8! Using a u8 for the tilecode
that we're shuffling makes some degree of sense though I'm uncertain how much value it provides --
using it for other stuff (like the enum, suit, face, unicode char) is sacrificing compute
for a minute amount of memory. Once we get some performance tests online I'd love to compare,
but for now it would be best practice to use usize..

- How do we want to deal with red fives?

- How are we going to model the wall, hands, and discard pile?

- How are we going to encrypt for multiplayer?

## Tile operations

- Dora Indicator Flow

Match statement or hashmap? That's the question. For now a match statement is okay, but for a small
amount of static data like this a hashmap is probably where we want to go.

