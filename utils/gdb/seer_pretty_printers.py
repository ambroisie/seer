import enum

import gdb.printing


class Square(object):
    """
    Python representation of a 'seer::board::square::Square' raw value.
    """

    FILES = list(map(lambda n: chr(ord("A") + n), range(8)))
    RANKS = list(map(lambda n: str(n + 1), range(8)))

    def __init__(self, val):
        self._val = val

    @classmethod
    def from_file_rank(cls, file, rank):
        return cls(file * 8 + rank)

    def __str__(self):
        return self.FILES[self.file] + self.RANKS[self.rank]

    @property
    def rank(self):
        return int(self._val) % 8

    @property
    def file(self):
        return int(self._val) // 8


class Bitboard(object):
    """
    Python representation of a 'seer::board::bitboard::Bitboard' raw value.
    """

    def __init__(self, val):
        self._val = val

    def __str__(self):
        return "[" + ", ".join(map(str, self.squares)) + "]"

    def at(self, square):
        return bool(self._val & (1 << square._val))

    @property
    def squares(self):
        n = self._val
        while n:
            b = n & (~n + 1)
            yield Square(b.bit_length() - 1)
            n ^= b


class CastleRights(enum.IntEnum):
    """
    Python representation of a 'seer::board::castle_rights::CastleRights' raw value.
    """

    # Should be kept in sync with the enum in `color.rs`
    NO_SIDE = 0
    KING_SIDE = 1
    QUEEN_SIDE = 2
    BOTH_SIDES = 3

    def __str__(self):
        return self.name.title().replace("_", "")


class Color(enum.IntEnum):
    """
    Python representation of a 'seer::board::color::Color' raw value.
    """

    # Should be kept in sync with the enum in `color.rs`
    WHITE = 0
    BLACK = 1

    def __str__(self):
        return self.name.title()


class File(enum.IntEnum):
    """
    Python representation of a 'seer::board::file::File' raw value.
    """

    # Should be kept in sync with the enum in `file.rs`
    A = 0
    B = 1
    C = 2
    D = 3
    E = 4
    F = 5
    G = 6
    H = 7

    def __str__(self):
        return self.name.title()


class Rank(enum.IntEnum):
    """
    Python representation of a 'seer::board::rank::Rank' raw value.
    """

    # Should be kept in sync with the enum in `rank.rs`
    First = 0
    Second = 1
    Third = 2
    Fourth = 3
    Fifth = 4
    Sixth = 5
    Seventh = 6
    Eighth = 7

    def __str__(self):
        return self.name.title()


class Piece(enum.IntEnum):
    """
    Python representation of a 'seer::board::piece::Piece' raw value.
    """

    # Should be kept in sync with the enum in `piece.rs`
    KING = 0
    QUEEN = 1
    ROOK = 2
    BISHOP = 3
    KNIGHT = 4
    PAWN = 5

    def __str__(self):
        return self.name.title()


class Move(object):
    """
    Wrapper around GDB's representation of a 'seer::board::move::Move'
    in memory.
    """

    # Should be kept in sync with the values in `move.rs`
    PIECE_SHIFT = 0
    PIECE_MASK = 0b111
    START_SHIFT = 3
    START_MASK = 0b11_1111
    DESTINATION_SHIFT = 9
    DESTINATION_MASK = 0b11_1111
    CAPTURE_SHIFT = 15
    CAPTURE_MASK = 0b111
    PROMOTION_SHIFT = 18
    PROMOTION_MASK = 0b111
    EN_PASSANT_SHIFT = 21
    EN_PASSANT_MASK = 0b1
    DOUBLE_STEP_SHIFT = 22
    DOUBLE_STEP_MASK = 0b1
    CASTLING_SHIFT = 23
    CASTLING_MASK = 0b1

    def __init__(self, val):
        self._val = val

    @property
    def piece(self):
        return Piece(self._val >> self.PIECE_SHIFT & self.PIECE_MASK)

    @property
    def start(self):
        return Square(self._val >> self.START_SHIFT & self.START_MASK)

    @property
    def destination(self):
        return Square(self._val >> self.DESTINATION_SHIFT & self.DESTINATION_MASK)

    @property
    def capture(self):
        index = self._val >> self.CAPTURE_SHIFT & self.CAPTURE_MASK
        if index == 7:
            return None
        return Piece(index)

    @property
    def promotion(self):
        index = self._val >> self.PROMOTION_SHIFT & self.PROMOTION_MASK
        if index == 7:
            return None
        return Piece(index)

    @property
    def en_passant(self):
        return bool(self._val >> self.EN_PASSANT_SHIFT & self.EN_PASSANT_MASK)

    @property
    def double_step(self):
        return bool(self._val >> self.DOUBLE_STEP_SHIFT & self.DOUBLE_STEP_MASK)

    @property
    def castling(self):
        return bool(self._val >> self.CASTLING_SHIFT & self.CASTLING_MASK)

    def __str__(self):
        KEYS = [
            "piece",
            "start",
            "destination",
            "capture",
            "promotion",
            "en_passant",
            "double_step",
            "castling",
        ]
        print_opt = lambda val: "(None)" if val is None else str(val)
        indent = lambda s: "    " + s

        values = [key + ": " + print_opt(getattr(self, key)) + ",\n" for key in KEYS]
        return "Move{\n" + "".join(map(indent, values)) + "}"


class SquarePrinter(object):
    "Print a seer::board::square::Square"

    def __init__(self, val):
        self._val = Square(val)

    def to_string(self):
        return str(self._val)


class BitboardPrinter(object):
    "Print a seer::board::bitboard::Bitboard"

    def __init__(self, val):
        self._val = Bitboard(int(val["__0"]))

    def to_string(self):
        return "Bitboard{" + str(self._val)[1:-1] + "}"


class CastleRightsPrinter(object):
    "Print a seer::board::castle_rights::CastleRights"

    def __init__(self, val):
        self._val = CastleRights(int(val))

    def to_string(self):
        return str(self._val)


class ColorPrinter(object):
    "Print a seer::board::color::Color"

    def __init__(self, val):
        self._val = Color(int(val))

    def to_string(self):
        return str(self._val)


class FilePrinter(object):
    "Print a seer::board::file::File"

    def __init__(self, val):
        self._val = File(int(val))

    def to_string(self):
        return str(self._val)


class RankPrinter(object):
    "Print a seer::board::rank::Rank"

    def __init__(self, val):
        self._val = Rank(int(val))

    def to_string(self):
        return str(self._val)


class PiecePrinter(object):
    "Print a seer::board::piece::Piece"

    def __init__(self, val):
        self._val = Piece(int(val))

    def to_string(self):
        return str(self._val)


class MovePrinter(object):
    "Print a seer::board::move::Move"

    def __init__(self, val):
        self._val = Move(int(val["__0"]))

    def to_string(self):
        return str(self._val)


def build_pretty_printer():
    pp = gdb.printing.RegexpCollectionPrettyPrinter('seer')

    pp.add_printer('Square', '^seer::board::square::Square$', SquarePrinter)
    pp.add_printer('Bitboard', '^seer::board::bitboard::Bitboard$', BitboardPrinter)
    pp.add_printer('CastleRights', '^seer::board::castle_rights::CastleRights$', CastleRightsPrinter)
    pp.add_printer('Color', '^seer::board::color::Color$', ColorPrinter)
    pp.add_printer('File', '^seer::board::file::File$', FilePrinter)
    pp.add_printer('Rank', '^seer::board::rank::Rank$', RankPrinter)
    pp.add_printer('Piece', '^seer::board::piece::Piece$', ColorPrinter)
    pp.add_printer('Move', '^seer::board::move::Move$', MovePrinter)

    return pp

gdb.printing.register_pretty_printer(gdb.current_objfile(), build_pretty_printer(), True)
