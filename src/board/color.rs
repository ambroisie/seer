use super::{Direction, Rank};

/// An enum representing the color of a player.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// The number of [Color] variants.
    pub const NUM_VARIANTS: usize = 2;

    const ALL: [Self; Self::NUM_VARIANTS] = [Self::White, Self::Black];

    /// Iterate over all colors in order.
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.iter().cloned()
    }

    /// Convert from a color index into a [Color] type.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline(always)]
    pub fn from_index(index: usize) -> Self {
        Self::try_from_index(index).expect("index out of bouds")
    }

    /// Convert from a color index into a [Color] type. Returns [None] if the index is out of
    /// bounds.
    pub fn try_from_index(index: usize) -> Option<Self> {
        if index < Self::NUM_VARIANTS {
            // SAFETY: we know the value is in-bounds
            Some(unsafe { Self::from_index_unchecked(index) })
        } else {
            None
        }
    }

    /// Convert from a color index into a [Color] type, no bounds checking.
    ///
    /// # Safety
    ///
    /// Should only be called with values that can be output by [Color::index()].
    #[inline(always)]
    pub unsafe fn from_index_unchecked(index: usize) -> Self {
        std::mem::transmute(index as u8)
    }

    /// Return the index of a given [Color].
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }

    /// Return the first [Rank] for pieces of the given [Color], where its pieces start.
    #[inline(always)]
    pub fn first_rank(self) -> Rank {
        match self {
            Self::White => Rank::First,
            Self::Black => Rank::Eighth,
        }
    }

    /// Return the second [Rank] for pieces of the given [Color], where its pawns start.
    #[inline(always)]
    pub fn second_rank(self) -> Rank {
        match self {
            Self::White => Rank::Second,
            Self::Black => Rank::Seventh,
        }
    }

    /// Return the third [Rank] for pieces of the given [Color], where its pawns move to after a
    /// one-square move on the start position.
    #[inline(always)]
    pub fn third_rank(self) -> Rank {
        match self {
            Self::White => Rank::Third,
            Self::Black => Rank::Sixth,
        }
    }

    /// Return the fourth [Rank] for pieces of the given [Color], where its pawns move to after a
    /// two-square move.
    #[inline(always)]
    pub fn fourth_rank(self) -> Rank {
        match self {
            Self::White => Rank::Fourth,
            Self::Black => Rank::Fifth,
        }
    }

    /// Return the seventh [Rank] for pieces of the given [Color], which is the rank before a pawn
    /// gets promoted.
    #[inline(always)]
    pub fn seventh_rank(self) -> Rank {
        match self {
            Self::White => Rank::Seventh,
            Self::Black => Rank::Second,
        }
    }

    /// Which way do pawns advance for this color.
    #[inline(always)]
    pub fn forward_direction(self) -> Direction {
        match self {
            Self::White => Direction::North,
            Self::Black => Direction::South,
        }
    }

    /// Which way do the opponent's pawns advance for this color.
    #[inline(always)]
    pub fn backward_direction(self) -> Direction {
        (!self).forward_direction()
    }
}

impl std::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_index() {
        assert_eq!(Color::from_index(0), Color::White);
        assert_eq!(Color::from_index(1), Color::Black);
    }

    #[test]
    fn index() {
        assert_eq!(Color::White.index(), 0);
        assert_eq!(Color::Black.index(), 1);
    }

    #[test]
    fn not() {
        assert_eq!(!Color::White, Color::Black);
        assert_eq!(!Color::Black, Color::White);
    }
}
