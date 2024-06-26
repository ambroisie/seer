import enum

import gdb
import gdb.printing


def optional(constructor, val):
    try:
        return constructor(val["Some"]["__0"])
    except gdb.error:
        return None


def print_opt(val):
    return "(None)" if val is None else str(val)


class Square(object):
    """
    Python representation of a 'seer::board::square::Square' raw value.
    """

    FILES = list(map(lambda n: chr(ord("A") + n), range(8)))
    RANKS = list(map(lambda n: str(n + 1), range(8)))

    def __init__(self, val):
        if isinstance(val, Square):
            val = val._val
        self._val = val

    @classmethod
    def from_gdb(cls, val):
        return cls(int(val))

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
        if isinstance(val, Bitboard):
            val = val._val
        self._val = val

    @classmethod
    def from_gdb(cls, val):
        return cls(int(val["__0"]))

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

    @classmethod
    def from_gdb(cls, val):
        return cls(int(val))

    def __str__(self):
        return self.name.title().replace("_", "")


class Color(enum.IntEnum):
    """
    Python representation of a 'seer::board::color::Color' raw value.
    """

    # Should be kept in sync with the enum in `color.rs`
    WHITE = 0
    BLACK = 1

    @classmethod
    def from_gdb(cls, val):
        return cls(int(val))

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

    @classmethod
    def from_gdb(cls, val):
        return cls(int(val))

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

    @classmethod
    def from_gdb(cls, val):
        return cls(int(val))

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

    @classmethod
    def from_gdb(cls, val):
        return cls(int(val))

    def __str__(self):
        return self.name.title()


class Move(object):
    """
    Wrapper around GDB's representation of a 'seer::board::move::Move'
    in memory.
    """

    def __init__(self, start, destination, promotion):
        self._start = Square(start)
        self._destination = Square(destination)
        self._promotion = Piece(promotion)

    @classmethod
    def from_gdb(cls, val):
        start = Square(int(val["start"]))
        destination = Square(int(val["destination"]))
        promotion = optional(Piece.from_gdb, val["promotion"])
        cls(start, destination, promotion)

    @property
    def start(self):
        return self._start

    @property
    def destination(self):
        return self._destination

    @property
    def promotion(self):
        return self._promotion

    def __str__(self):
        KEYS = [
            "start",
            "destination",
            "promotion",
        ]
        indent = lambda s: "    " + s

        values = [key + ": " + print_opt(getattr(self, key)) + ",\n" for key in KEYS]
        return "Move{\n" + "".join(map(indent, values)) + "}"


class ChessBoard(object):
    """
    Wrapper around GDB's representation of a 'seer::board::chess_board::ChessBoard'
    in memory.
    """

    def __init__(
        self,
        piece_occupancy,
        color_occupancy,
        castle_rights,
        half_move_clock,
        total_plies,
        side,
        en_passant,
    ):
        self._piece_occupancy = list(map(Bitboard, piece_occupancy))
        self._color_occupancy = list(map(Bitboard, color_occupancy))
        self._castle_rights = list(map(CastleRights, castle_rights))
        self._half_move_clock = int(half_move_clock)
        self._total_plies = int(total_plies)
        self._side = Color(side)
        self._en_passant = None if en_passant is None else Square(en_passant)

    @classmethod
    def from_gdb(cls, val):
        return cls(
            [Bitboard.from_gdb(val["piece_occupancy"][p]) for p in Piece],
            [Bitboard.from_gdb(val["color_occupancy"][c]) for c in Color],
            [CastleRights.from_gdb(val["castle_rights"][c]) for c in Color],
            int(val["half_move_clock"]),
            int(val["total_plies"]),
            Color.from_gdb(val["side"]),
            optional(Square.from_gdb, val["en_passant"]),
        )

    def at(self, square):
        for piece in Piece:
            if not self._piece_occupancy[piece].at(square):
                continue
            for color in Color:
                if not self._color_occupancy[color].at(square):
                    continue
                return (piece, color)
        return None

    def pretty_str(self):
        def pretty_piece(piece, color):
            return [
                ("♚", "♔"),
                ("♛", "♕"),
                ("♜", "♖"),
                ("♝", "♗"),
                ("♞", "♘"),
                ("♟", "♙"),
            ][piece][color]

        board = [
            [self.at(Square.from_file_rank(file, rank)) for file in File]
            for rank in Rank
        ]

        res = []
        res.append("   A B C D E F G H   ")
        for n, line in reversed(list(enumerate(board, start=1))):
            strings = [str(n) + " "]
            strings.extend(" " if p is None else pretty_piece(*p) for p in line)
            strings.append(" " + str(n))
            res.append("|".join(strings))
        res.append("   A B C D E F G H   ")
        res += [
            "Half-move clock: " + str(self._half_move_clock),
            "Total plies: " + str(self._total_plies),
            "Side to play: " + str(self._side),
            "En passant: " + print_opt(self._en_passant),
        ]
        return "\n".join(res)


class SquarePrinter(object):
    "Print a seer::board::square::Square"

    def __init__(self, val):
        self._val = Square.from_gdb(val)

    def to_string(self):
        return str(self._val)


class BitboardPrinter(object):
    "Print a seer::board::bitboard::Bitboard"

    def __init__(self, val):
        self._val = Bitboard.from_gdb(val)

    def to_string(self):
        return "Bitboard{" + str(self._val)[1:-1] + "}"


class CastleRightsPrinter(object):
    "Print a seer::board::castle_rights::CastleRights"

    def __init__(self, val):
        self._val = CastleRights.from_gdb(val)

    def to_string(self):
        return str(self._val)


class ColorPrinter(object):
    "Print a seer::board::color::Color"

    def __init__(self, val):
        self._val = Color.from_gdb(val)

    def to_string(self):
        return str(self._val)


class FilePrinter(object):
    "Print a seer::board::file::File"

    def __init__(self, val):
        self._val = File.from_gdb(val)

    def to_string(self):
        return str(self._val)


class RankPrinter(object):
    "Print a seer::board::rank::Rank"

    def __init__(self, val):
        self._val = Rank.from_gdb(val)

    def to_string(self):
        return str(self._val)


class PiecePrinter(object):
    "Print a seer::board::piece::Piece"

    def __init__(self, val):
        self._val = Piece.from_gdb(val)

    def to_string(self):
        return str(self._val)


class MovePrinter(object):
    "Print a seer::board::move::Move"

    def __init__(self, val):
        self._val = Move.from_gdb(val)

    def to_string(self):
        return str(self._val)


class PrintBoard(gdb.Command):
    """
    Pretty-print a 'seer::board::chess_board::ChessBoard' as a 2D textual chess board.
    """

    def __init__(self):
        super(PrintBoard, self).__init__(
            "print-board", gdb.COMMAND_USER, gdb.COMPLETE_EXPRESSION
        )

    def invoke(self, arg, from_tty):
        board = ChessBoard.from_gdb(gdb.parse_and_eval(arg))
        print(board.pretty_str())


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


def register_commands():
    PrintBoard()


gdb.printing.register_pretty_printer(gdb.current_objfile(), build_pretty_printer(), True)
register_commands()
