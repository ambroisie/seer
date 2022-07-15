import gdb.printing

class Square(object):
    """
    Wrapper around GDB's representation of a 'seer::board::square::Square' in
    memory.
    """

    FILES = list(map(lambda n: chr(ord('A') + n), range(8)))
    RANKS = list(map(lambda n: str(n + 1), range(8)))

    def __init__(self, val):
        self._val = val

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
    Wrapper around GDB's representation of a 'seer::board::bitboard::Bitboard'
    in memory.
    """

    def __init__(self, val):
        self._val = val

    def __str__(self):
        return "[" + ", ".join(map(str, self.squares)) + "]"

    @property
    def squares(self):
        n = int(self._val["__0"])
        while n:
            b = n & (~n+1)
            yield Square(b.bit_length() - 1)
            n ^= b

class SquarePrinter(object):
    "Print a seer::board::square::Square"

    def __init__(self, val):
        self._val = Square(val)

    def to_string(self):
        return str(self._val)

    def display_hint(self):
        return 'string'

class BitboardPrinter(object):
    "Print a seer::board::bitboard::Bitboard"

    def __init__(self, val):
        self._val = Bitboard(val)

    def to_string(self):
        return "Bitboard{" + str(self._val)[1:-1] + "}"

    def display_hint(self):
        return 'string'

def build_pretty_printer():
    pp = gdb.printing.RegexpCollectionPrettyPrinter('seer')

    pp.add_printer('BigNum', '^seer::board::square::Square$', SquarePrinter)
    pp.add_printer('BigNum', '^seer::board::bitboard::Bitboard$', BitboardPrinter)

    return pp

gdb.printing.register_pretty_printer(gdb.current_objfile(), build_pretty_printer(), True)
