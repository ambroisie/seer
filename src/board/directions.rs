/// A direction on the board. Either along the rook, bishop, or knight directions
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,

    NorthWest,
    SouthWest,
    SouthEast,
    NorthEast,

    NorthNorthWest,
    NorthWestWest,
    SouthWestWest,
    SouthSouthWest,
    SouthSouthEast,
    SouthEastEast,
    NorthEastEast,
    NorthNorthEast,
}
