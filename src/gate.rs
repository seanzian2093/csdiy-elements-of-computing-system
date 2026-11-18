//! # Chapter 1: Boolean Logic
//!
//! This module implements the foundational Boolean logic gates that form the basis
//! of all digital circuits in the computer architecture we're building.

/// ## Basic Signals
///
/// A single bit: the smallest signal in any digital circuit.
///
/// `false` is logic 0, `true` is logic 1.
pub type Bit = bool;

/// ## Basic Gates
///
/// ### Nand Gate
/// The starting point of our computer architecture is Nand gate, from which all other
/// gates and chips are built. The Nand gate is designed to compute the following
/// Boolean function:
/// |  a  |  b  | Nand(a,b) |
/// |:---:|:---:|:---------:|
/// |  0  |  0  |     1     |
/// |  0  |  1  |     1     |
/// |  1  |  0  |     1     |
/// |  1  |  1  |     0     |
///
/// This gate is considered primitive and thus there is no need to implement it.
/// Instead, we use Rust's built-in boolean operators to simulate it.
pub fn nand(a: Bit, b: Bit) -> Bit {
    !(a & b)
}

/// ### Not Gate
/// This single-input Not gate, also known as an "converter", converts its input
/// from 0 to 1 and vice versa. Its Boolean function is defined as follows:
/// |  a  | Not(a) |
/// |:---:|:------:|
/// |  0  |   1    |
/// |  1  |   0    |
///
/// We implement the Not gate using the Nand gate as follows:
/// |  a  |  a  | Nand(a,a) | Not(a) |
/// |:---:|:---:|:---------:|:------:|
/// |  0  |  0  |     1     |   1    |
/// |  1  |  1  |     0     |   0    |
pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

/// ### And Gate
/// The And gate is a two-input gate that outputs 1 only when both inputs are 1, and 0 otherwise.
/// Boolean function:
/// |  a  |  b  | And(a,b) |
/// |:---:|:---:|:--------:|
/// |  0  |  0  |    0     |
/// |  0  |  1  |    0     |
/// |  1  |  0  |    0     |
/// |  1  |  1  |    1     |
///
/// We implement the And gate using the Nand and Not gates as follows:
/// |  a  |  b  | Nand(a,b) | Not(Nand(a,b)) |
/// |:---:|:---:|:---------:|:--------------:|
/// |  0  |  0  |     1     |        0       |
/// |  0  |  1  |     1     |        0       |
/// |  1  |  0  |     1     |        0       |
/// |  1  |  1  |     0     |        1       |
pub fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

/// ### Or Gate
/// The Or gate returns 1 if at least one of its inputs is 1, and 0 otherwise.
/// Boolean function:
/// |  a  |  b  | Or(a,b) |
/// |:---:|:---:|:-------:|
/// |  0  |  0  |    0    |
/// |  0  |  1  |    1    |
/// |  1  |  0  |    1    |
/// |  1  |  1  |    1    |
///
/// We implement the Or gate using Nand gates as follows (by De Morgan's Law):
///
/// First approach (using Not and Nand):
/// |  a  |  b  | w1=Not(a) | w2=Not(b) | Nand(w1,w2) | Or(a,b) |
/// |:---:|:---:|:---------:|:---------:|:----------:|:-------:|
/// |  0  |  0  |     1     |     1     |      0     |    0    |
/// |  0  |  1  |     1     |     0     |      1     |    1    |
/// |  1  |  0  |     0     |     1     |      1     |    1    |
/// |  1  |  1  |     0     |     0     |      1     |    1    |
///
/// More efficient implementation (using only Nand gates):
/// |  a  |  b  | w1=Nand(a,a) | w2=Nand(b,b) | Nand(w1,w2) | Or(a,b) |
/// |:---:|:---:|:------------:|:------------:|:-----------:|:-------:|
/// |  0  |  0  |      1       |      1       |      0      |    0    |
/// |  0  |  1  |      1       |      0       |      1      |    1    |
/// |  1  |  0  |      0       |      1       |      1      |    1    |
/// |  1  |  1  |      0       |      0       |      1      |    1    |
pub fn or(a: Bit, b: Bit) -> Bit {
    nand(nand(a, a), nand(b, b))
}

/// ### Xor Gate
/// The Xor gate, also known as "exclusive or", returns 1 if exactly one of its inputs is 1, and 0 otherwise.
/// Boolean function:
/// |  a  |  b  | Xor(a,b) |
/// |:---:|:---:|:--------:|
/// |  0  |  0  |    0     |
/// |  0  |  1  |    1     |
/// |  1  |  0  |    1     |
/// |  1  |  1  |    0     |
///
/// We implement the Xor gate using Nand gates as follows:
/// |  a  |  b  | w1=Nand(a,b) | w2=Nand(a,w1) | w3=Nand(b,w1) | Xor(a,b)=Nand(w2,w3) |
/// |:---:|:---:|:------------:|:-------------:|:-------------:|:--------------------:|
/// |  0  |  0  |      1       |       1       |       0       |          0           |
/// |  0  |  1  |      1       |       1       |       1       |          1           |
/// |  1  |  0  |      1       |       0       |       1       |          1           |
/// |  1  |  1  |      0       |       1       |       0       |          0           |
pub fn xor(a: Bit, b: Bit) -> Bit {
    let w1 = nand(a, b);
    let w2 = nand(a, w1);
    let w3 = nand(b, w1);
    nand(w2, w3)
}

/// ### Half Adder
/// A half adder adds two single-bit inputs and returns the sum and carry bits.
/// The returned tuple is `(sum, carry)`.
///
/// |  a  |  b  | sum | carry |
/// |:---:|:---:|:---:|:-----:|
/// |  0  |  0  |  0  |   0   |
/// |  0  |  1  |  1  |   0   |
/// |  1  |  0  |  1  |   0   |
/// |  1  |  1  |  0  |   1   |
///
/// Interpreting `a + b` as ordinary binary addition, the result has two positions:
/// the rightmost bit is the least-significant bit (LSB), returned as `sum`, and the
/// leftmost bit is the most-significant bit (MSB), returned as `carry`. For example,
/// `1 + 1 = 10` in binary, so the outputs are `(sum, carry) = (0, 1)`.
///
/// The sum is the exclusive OR of the inputs, and the carry is set only when both
/// inputs are 1:
/// `sum = xor(a, b)` and `carry = and(a, b)`.
pub fn half_adder(a: Bit, b: Bit) -> (Bit, Bit) {
    (xor(a, b), and(a, b))
}

/// ### Full Adder
/// A full adder adds two single-bit inputs and an incoming carry bit.
/// The returned tuple is `(sum, carry)`.
///
/// |  a  |  b  |  c  | sum | carry |
/// |:---:|:---:|:---:|:---:|:-----:|
/// |  0  |  0  |  0  |  0  |   0   |
/// |  0  |  0  |  1  |  1  |   0   |
/// |  0  |  1  |  0  |  1  |   0   |
/// |  0  |  1  |  1  |  0  |   1   |
/// |  1  |  0  |  0  |  1  |   0   |
/// |  1  |  0  |  1  |  0  |   1   |
/// |  1  |  1  |  0  |  0  |   1   |
/// |  1  |  1  |  1  |  1  |   1   |
///
/// As with a half adder, `sum` is the least-significant bit and `carry` is the
/// most-significant bit of the two-bit result. For example, `1 + 1 + 1 = 11` in
/// binary, so the outputs are `(sum, carry) = (1, 1)`.
///
/// The full adder is built from two half adders and an Or gate: the first half adder
/// adds `a` and `b`; the second adds that partial sum to `c`. The two carry bits are
/// combined to produce `carry`.
pub fn full_adder(a: Bit, b: Bit, c: Bit) -> (Bit, Bit) {
    let (partial_sum, carry_ab) = half_adder(a, b);
    let (sum, carry_c_sum) = half_adder(partial_sum, c);
    (sum, or(carry_ab, carry_c_sum))
}

/// ### Multiplexer (Mux)
/// The Multiplexer (Mux) is a three-input gate that uses one of the inputs, called "selection bit", to select and output one of the other two inputs, called "data bits".
/// THus a better name could be "selector", i.e.,:
/// | sel | output |
/// |:---:|:------:|
/// |  0  |   a    |
/// |  1  |   b    |
///
/// Boolean function:
/// |  a  |  b  | sel | Mux(sel,a,b) |
/// |:---:|:---:|:---:|:------------:|
/// |  0  |  0  |  0  |      0       |
/// |  0  |  1  |  0  |      0       |
/// |  1  |  0  |  0  |      1       |
/// |  1  |  1  |  0  |      1       |
/// |  0  |  0  |  1  |      0       |
/// |  0  |  1  |  1  |      1       |
/// |  1  |  0  |  1  |      0       |
/// |  1  |  1  |  1  |      1       |
///
/// We implement the Mux gate using Nand gates as follows:
/// |  a  |  b  | sel | Not(sel) | w1=And(a,Not(sel)) | w2=And(b,sel) | Mux(sel,a,b)=Or(w1,w2) |
/// |:---:|:---:|:---:|:--------:|:------------------:|:-------------:|:----------------------:|
/// |  0  |  0  |  0  |    1     |         0          |       0       |           0            |
/// |  0  |  1  |  0  |    1     |         0          |       0       |           0            |
/// |  1  |  0  |  0  |    1     |         1          |       0       |           1            |
/// |  1  |  1  |  0  |    1     |         1          |       0       |           1            |
/// |  0  |  0  |  1  |    0     |         0          |       0       |           0            |
/// |  0  |  1  |  1  |    0     |         0          |       1       |           1            |
/// |  1  |  0  |  1  |    0     |         0          |       0       |           0            |
/// |  1  |  1  |  1  |    0     |         0          |       1       |           1            |
///
pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    let not_sel = not(sel);
    let w1 = and(a, not_sel);
    let w2 = and(b, sel);
    or(w1, w2)
}

/// ### Demultiplexer (DMux)
/// A demultiplexer performs the opposite function of a multiplexer - it takes a single input and channels it to one of two possible outputs
/// according to a selector bit that specifies which output to choose.
///
/// | sel | `out[0]` | `out[1]` |
/// |:---:|:--------:|:--------:|
/// |  0  |  input   |    0     |
/// |  1  |    0     |  input   |
///
/// Boolean function:
/// | input | sel | DMux(input,sel) = (`out[0]`, `out[1]`) |
/// |:-----:|:---:|:--------------------------------------:|
/// |   0   |  0  |           (0, 0)                       |
/// |   0   |  1  |           (0, 0)                       |
/// |   1   |  0  |           (1, 0)                       |
/// |   1   |  1  |           (0, 1)                       |
///
/// We implement the DMux gate using the And and Not gates as follows:
/// | input | sel | Not(sel) | `out[0]`=And(input,Not(sel)) | `out[1]`=And(input,sel) |
/// |:-----:|:---:|:--------:|:----------------------------:|:-----------------------:|
/// |   0   |  0  |    1     |             0                |            0            |
/// |   0   |  1  |    0     |             0                |            0            |
/// |   1   |  0  |    1     |             1                |            0            |
/// |   1   |  1  |    0     |             0                |            1            |
///
/// Note that the two outputs are mutually exclusive: exactly one of them can be 1,
/// and only when the input is 1. This is the mirror image of the Mux, which ORs
/// these two terms together into a single output.
pub fn dmux(input: Bit, sel: Bit) -> (Bit, Bit) {
    let not_sel = not(sel);
    let out0 = and(input, not_sel);
    let out1 = and(input, sel);
    (out0, out1)
}

/// ## 16-bit Gates
///
/// A 16-bit word is simply a vector of 16 single bits. Every 16-bit gate below is
/// built from its single-bit counterpart by wiring 16 identical single-bit chips in
/// parallel: bit `i` of each operand feeds chip `i`, and chip `i`'s output becomes
/// bit `i` of the result.
///
/// | bit  |   a    |   b    |
/// |:----:|:------:|:------:|
/// |  0   | `a[0]` | `b[0]` |
/// |  …   |   …    |   …    |
/// |  15  | `a[15]`| `b[15]`|
///
/// This mirrors the Nand2tetris convention, where `And16`, `Or16`, `Not16` and
/// `Mux16` are each a bundle of 16 of the single-bit gates defined above.
///
/// The number of bits in one word.
pub const WORD_SIZE: usize = 16;

/// A 16-bit word: a run of 16 single bits, indexed by significance from `0..15`.
///
/// Bit `0` is the least-significant bit (LSB), and bit `15` is the most-significant
/// bit (MSB). This is array/index order; conventional binary notation writes the MSB
/// on the left and the LSB on the right. For example, the word with only bit `0` set
/// is written `0000_0000_0000_0001`.
/// A `Bits` value is a bit pattern; the Hack ALU interprets it as a 16-bit
/// two's-complement signed value when determining whether its output is negative.
pub type Bits = [Bit; WORD_SIZE];

/// Errors returned when parsing a binary string into a [`Bits`] word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitStringError {
    /// The input contains no bits.
    Empty,
    /// A character other than `0`, `1`, or a separator was found.
    InvalidCharacter { character: char, byte_index: usize },
    /// A separator is not between two bits.
    InvalidSeparator { byte_index: usize },
    /// The input contains more bits than fit in a word.
    TooManyBits { max: usize },
}

impl std::fmt::Display for BitStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "bit string must contain at least one bit"),
            Self::InvalidCharacter {
                character,
                byte_index,
            } => write!(
                f,
                "invalid character {character:?} at byte index {byte_index}; expected '0', '1', or '_'"
            ),
            Self::InvalidSeparator { byte_index } => write!(
                f,
                "misplaced '_' at byte index {byte_index}; underscores must separate bits"
            ),
            Self::TooManyBits { max } => {
                write!(f, "bit string exceeds the {max}-bit word width")
            }
        }
    }
}

impl std::error::Error for BitStringError {}

/// Parse conventional binary notation into a [`Bits`] word.
///
/// The string is written most-significant bit first, as in `1011`, i.e. leftmost is
/// MSB, rightmost is LSB; underscores are allowed only between bits, as in `0000_0000_0000_1011`.
/// Short strings are zero-extended on the left, while strings longer than [`WORD_SIZE`]
/// are rejected.
/// The string's written order is opposite to the array's index order: its rightmost digit
/// becomes `Bits[0]`, the least-significant bit.
///
/// ```
/// # use csdiy_eocs::gate::{and16, parse_bits};
/// let a = parse_bits("1011").expect("valid bit string");
/// let b = parse_bits("0000_0000_0000_0011").expect("valid bit string");
/// let result = and16(a, b);
/// assert_eq!(result, parse_bits("0011").expect("valid bit string"));
/// ```
pub fn parse_bits(input: &str) -> Result<Bits, BitStringError> {
    let mut parsed = Vec::new();
    let mut previous_was_bit = false;

    for (byte_index, character) in input.char_indices() {
        match character {
            '0' | '1' => {
                if parsed.len() == WORD_SIZE {
                    return Err(BitStringError::TooManyBits { max: WORD_SIZE });
                }
                parsed.push(character == '1');
                previous_was_bit = true;
            }
            '_' => {
                if !previous_was_bit {
                    return Err(BitStringError::InvalidSeparator { byte_index });
                }
                previous_was_bit = false;
            }
            _ => {
                return Err(BitStringError::InvalidCharacter {
                    character,
                    byte_index,
                });
            }
        }
    }

    if parsed.is_empty() {
        return Err(BitStringError::Empty);
    }
    if !previous_was_bit {
        let byte_index = input.char_indices().last().map_or(0, |(index, _)| index);
        return Err(BitStringError::InvalidSeparator { byte_index });
    }

    // Reverse to convert from MSB-first (string order) to LSB-first (array index order)
    parsed.reverse();
    let mut bits: Bits = [false; WORD_SIZE];
    for (i, &bit) in parsed.iter().enumerate() {
        bits[i] = bit;
    }

    Ok(bits)
}

/// ### Not16
/// The 16-bit Not gate inverts every bit of its input, bit by bit.
///
/// It is 16 independent [Not](not) gates wired in parallel: `not16(a)[i] = not(a[i])`.
/// In particular `not16(a)` is the one's complement of `a`.
pub fn not16(a: Bits) -> Bits {
    a.map(not)
}

/// ### And16
/// The 16-bit And gate produces a word whose bit `i` is the And of the `i`-th bits of
/// its two operands. It is 16 independent [And](and) gates wired in parallel:
/// `and16(a, b)[i] = and(a[i], b[i])`.
pub fn and16(a: Bits, b: Bits) -> Bits {
    std::array::from_fn(|i| and(a[i], b[i]))
}

/// ### Or16
/// The 16-bit Or gate produces a word whose bit `i` is the Or of the `i`-th bits of
/// its two operands. It is 16 independent [Or](or) gates wired in parallel:
/// `or16(a, b)[i] = or(a[i], b[i])`.
pub fn or16(a: Bits, b: Bits) -> Bits {
    std::array::from_fn(|i| or(a[i], b[i]))
}

/// ### Mux16
/// The 16-bit Multiplexer selects, as a whole word, between two 16-bit data words
/// using a single selection bit. When `sel` is 0 the entire word `a` passes through,
/// and when `sel` is 1 the entire word `b` passes through:
///
/// | sel | Mux16(a, b, sel) |
/// |:---:|:----------------:|
/// |  0  |        a         |
/// |  1  |        b         |
///
/// It is 16 independent [Mux](mux) gates wired in parallel, all sharing the same
/// `sel`: `mux16(a, b, sel)[i] = mux(a[i], b[i], sel)`.
pub fn mux16(a: Bits, b: Bits, sel: Bit) -> Bits {
    std::array::from_fn(|i| mux(a[i], b[i], sel))
}

/// ### Add16
/// Adds two 16-bit words with a ripple-carry adder, starting at the least-significant
/// bit. The returned word contains the low 16 bits of the sum; any carry out of the
/// most-significant bit is discarded. Thus, the result is `(a + b) mod 2^16`.
pub fn add16(a: Bits, b: Bits) -> Bits {
    let mut sum = [false; WORD_SIZE];
    let mut carry = false;

    for index in 0..WORD_SIZE {
        let (bit_sum, next_carry) = full_adder(a[index], b[index], carry);
        sum[index] = bit_sum;
        carry = next_carry;
    }

    sum
}

/// ### Inc16
/// Increments a 16-bit word by 1 using a ripple carry, starting at the
/// least-significant bit. Any carry out of the most-significant bit is discarded,
/// so incrementing `1111_1111_1111_1111` wraps around to `0000_0000_0000_0000`.
pub fn inc16(a: Bits) -> Bits {
    let mut result = [false; WORD_SIZE];
    let mut carry = true;

    for index in 0..WORD_SIZE {
        let (sum, next_carry) = half_adder(a[index], carry);
        result[index] = sum;
        carry = next_carry;
    }

    result
}

/// ### Hack ALU
/// Computes one of 18 functions of the 16-bit inputs `x` and `y`, selected by six
/// control bits. Returns `(out, zr, ng)`, where `zr` is true exactly when `out` is
/// zero, and `ng` is true exactly when `out` is negative.
///
/// In the Hack platform, values are interpreted as 16-bit two's-complement signed
/// integers, i.e.`-x = 2^16 - x if x >0` i.e. the leftmost digital of negative number,
/// when written in binary form, is "1", e.g., `1000_0000_0000_0000` represents the
/// most negative number.
///
/// Thus the most-significant bit(leftmost in binary form or `Bits[15]` in `Bits`, i.e. reversed)
/// is the sign bit: `ng` is true exactly, when that bit is `1`, or equivalently when `out < 0`.
///
/// `zx` and `zy` replace `x` and `y` with zero when set; `nx` and `ny` bitwise-negate
/// those values when set. `f` selects bitwise AND when clear or addition when set.
/// `no` bitwise-negates the selected result when set.
///
/// | zx | nx | zy | ny | f | no |   out   |
/// |:--:|:--:|:--:|:--:|:-:|:--:|:-------:|
/// | 1  | 0  | 1  | 0  | 1 | 0  |   `0`   |
/// | 1  | 1  | 1  | 1  | 1 | 1  |   `1`   |
/// | 1  | 1  | 1  | 0  | 1 | 0  |  `-1`   |
/// | 0  | 0  | 1  | 1  | 0 | 0  |   `x`   |
/// | 1  | 1  | 0  | 0  | 0 | 0  |   `y`   |
/// | 0  | 0  | 1  | 1  | 0 | 1  |  `!x`   |
/// | 1  | 1  | 0  | 0  | 0 | 1  |  `!y`   |
/// | 0  | 0  | 1  | 1  | 1 | 1  |  `-x`   |
/// | 1  | 1  | 0  | 0  | 1 | 1  |  `-y`   |
/// | 0  | 1  | 1  | 1  | 1 | 1  | `x + 1` |
/// | 1  | 1  | 0  | 1  | 1 | 1  | `y + 1` |
/// | 0  | 0  | 1  | 1  | 1 | 0  | `x - 1` |
/// | 1  | 1  | 0  | 0  | 1 | 0  | `y - 1` |
/// | 0  | 0  | 0  | 0  | 1 | 0  | `x + y` |
/// | 0  | 1  | 0  | 0  | 1 | 1  | `x - y` |
/// | 0  | 0  | 0  | 1  | 1 | 1  | `y - x` |
/// | 0  | 0  | 0  | 0  | 0 | 0  | `x & y` |
/// | 0  | 1  | 0  | 1  | 0 | 1  | `x | y` |
pub fn alu(
    x: Bits,
    y: Bits,
    zx: Bit,
    nx: Bit,
    zy: Bit,
    ny: Bit,
    f: Bit,
    no: Bit,
) -> (Bits, Bit, Bit) {
    let zero = [false; WORD_SIZE];
    let x = mux16(x, zero, zx);
    let x = mux16(x, not16(x), nx);
    let y = mux16(y, zero, zy);
    let y = mux16(y, not16(y), ny);

    let out = mux16(and16(x, y), add16(x, y), f);
    let out = mux16(out, not16(out), no);
    let zr = not(out.iter().copied().fold(false, or));
    let ng = out[WORD_SIZE - 1];

    (out, zr, ng)
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use super::*;

    #[test]
    fn test_nand_table() {
        assert_eq!(nand(false, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(true, true), false);
    }

    #[test]
    fn test_not_table() {
        assert_eq!(not(false), true);
        assert_eq!(not(true), false);
    }

    #[test]
    fn test_and_table() {
        assert_eq!(and(false, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(true, false), false);
        assert_eq!(and(true, true), true);
    }

    #[test]
    fn test_or_table() {
        assert_eq!(or(false, false), false);
        assert_eq!(or(false, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(true, true), true);
    }

    #[test]
    fn test_xor_table() {
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(true, true), false);
    }

    #[test]
    fn test_half_adder_table() {
        assert_eq!(half_adder(false, false), (false, false));
        assert_eq!(half_adder(false, true), (true, false));
        assert_eq!(half_adder(true, false), (true, false));
        assert_eq!(half_adder(true, true), (false, true));
    }

    #[test]
    fn test_full_adder_table() {
        assert_eq!(full_adder(false, false, false), (false, false));
        assert_eq!(full_adder(false, false, true), (true, false));
        assert_eq!(full_adder(false, true, false), (true, false));
        assert_eq!(full_adder(false, true, true), (false, true));
        assert_eq!(full_adder(true, false, false), (true, false));
        assert_eq!(full_adder(true, false, true), (false, true));
        assert_eq!(full_adder(true, true, false), (false, true));
        assert_eq!(full_adder(true, true, true), (true, true));
    }

    #[test]
    fn test_mux_table() {
        assert_eq!(mux(false, false, false), false);
        assert_eq!(mux(false, true, false), false);
        assert_eq!(mux(true, false, false), true);
        assert_eq!(mux(true, true, false), true);
        assert_eq!(mux(false, false, true), false);
        assert_eq!(mux(false, true, true), true);
        assert_eq!(mux(true, false, true), false);
        assert_eq!(mux(true, true, true), true);
    }

    #[test]
    fn test_dmux_table() {
        assert_eq!(dmux(false, false), (false, false));
        assert_eq!(dmux(false, true), (false, false));
        assert_eq!(dmux(true, false), (true, false));
        assert_eq!(dmux(true, true), (false, true));
    }

    fn word(bit_string: &str) -> Bits {
        parse_bits(bit_string).expect("valid test bit string")
    }

    #[test]
    fn test_not16_table() {
        // Not16 inverts every bit, so each word maps to its complement.
        let zeros = word("0000_0000_0000_0000");
        let ones = word("1111_1111_1111_1111");
        let alt = word("0101_0101_0101_0101");
        let alt_x = word("1010_1010_1010_1010");
        for (input, expected) in [(zeros, ones), (ones, zeros), (alt, alt_x), (alt_x, alt)] {
            assert_eq!(not16(input), expected);
        }
    }

    #[test]
    fn test_and16_table() {
        // and16 sets a bit only where both operands have it set.
        let zeros = word("0000_0000_0000_0000");
        let ones = word("1111_1111_1111_1111");
        let low = word("0000_0000_1111_1111");
        let alt = word("0101_0101_0101_0101");
        let alt_x = word("1010_1010_1010_1010");
        for (a, b, expected) in [
            (zeros, ones, zeros),
            (ones, zeros, zeros),
            (ones, ones, ones),
            (low, ones, low), // AND with all-1s is the identity
            (alt, alt, alt),
            (alt, alt_x, zeros), // the two patterns never agree
        ] {
            assert_eq!(and16(a, b), expected);
        }
    }

    #[test]
    fn test_or16_table() {
        // or16 sets a bit wherever either operand has it set.
        let zeros = word("0000_0000_0000_0000");
        let ones = word("1111_1111_1111_1111");
        let low = word("0000_0000_1111_1111");
        let alt = word("0101_0101_0101_0101");
        let alt_x = word("1010_1010_1010_1010");
        for (a, b, expected) in [
            (zeros, zeros, zeros),
            (ones, zeros, ones),
            (zeros, ones, ones),
            (low, zeros, low), // OR with all-0s is the identity
            (alt, alt, alt),
            (alt, alt_x, ones), // the two patterns complement each other
        ] {
            assert_eq!(or16(a, b), expected);
        }
    }

    #[test]
    fn test_mux16_table() {
        // sel = 0 passes the whole word `a` through; sel = 1 passes `b`.
        let low = word("0000_0000_1111_1111");
        let alt = word("0101_0101_0101_0101");
        let alt_x = word("1010_1010_1010_1010");
        let ones = word("1111_1111_1111_1111");
        let zeros = word("0000_0000_0000_0000");
        for (a, b) in [(low, alt), (alt_x, ones), (zeros, ones)] {
            assert_eq!(mux16(a, b, false), a);
            assert_eq!(mux16(a, b, true), b);
        }
    }

    #[test]
    fn test_add16() {
        let zeros = word("0000_0000_0000_0000");
        let ones = word("1111_1111_1111_1111");
        assert_eq!(add16(zeros, zeros), zeros);
        assert_eq!(
            add16(word("0000_0000_1111_1111"), word("1")),
            word("0000_0001_0000_0000")
        );
        assert_eq!(
            add16(word("0001_0010_0011_0100"), word("0100_0011_0010_0001")),
            word("0101_0101_0101_0101")
        );
        assert_eq!(add16(ones, word("1")), zeros);
    }

    #[test]
    fn test_inc16() {
        assert_eq!(
            inc16(word("0000_0000_0000_0000")),
            word("0000_0000_0000_0001")
        );
        assert_eq!(
            inc16(word("0000_0000_0000_1111")),
            word("0000_0000_0001_0000")
        );
        assert_eq!(
            inc16(word("0000_0000_1111_1111")),
            word("0000_0001_0000_0000")
        );
        assert_eq!(
            inc16(word("1111_1111_1111_1111")),
            word("0000_0000_0000_0000")
        );
    }

    #[test]
    fn test_alu_functions_and_flags() {
        let x = word("0000_0000_0000_0101");
        let y = word("0000_0000_0000_0011");
        let zero = word("0000_0000_0000_0000");
        let cases = [
            (
                (true, false, true, false, true, false),
                "0000_0000_0000_0000",
            ),
            ((true, true, true, true, true, true), "0000_0000_0000_0001"),
            (
                (true, true, true, false, true, false),
                "1111_1111_1111_1111",
            ),
            (
                (false, false, true, true, false, false),
                "0000_0000_0000_0101",
            ),
            (
                (true, true, false, false, false, false),
                "0000_0000_0000_0011",
            ),
            (
                (false, false, true, true, false, true),
                "1111_1111_1111_1010",
            ),
            (
                (true, true, false, false, false, true),
                "1111_1111_1111_1100",
            ),
            (
                (false, false, true, true, true, true),
                "1111_1111_1111_1011",
            ),
            (
                (true, true, false, false, true, true),
                "1111_1111_1111_1101",
            ),
            ((false, true, true, true, true, true), "0000_0000_0000_0110"),
            ((true, true, false, true, true, true), "0000_0000_0000_0100"),
            (
                (false, false, true, true, true, false),
                "0000_0000_0000_0100",
            ),
            (
                (true, true, false, false, true, false),
                "0000_0000_0000_0010",
            ),
            (
                (false, false, false, false, true, false),
                "0000_0000_0000_1000",
            ),
            (
                (false, true, false, false, true, true),
                "0000_0000_0000_0010",
            ),
            (
                (false, false, false, true, true, true),
                "1111_1111_1111_1110",
            ),
            (
                (false, false, false, false, false, false),
                "0000_0000_0000_0001",
            ),
            (
                (false, true, false, true, false, true),
                "0000_0000_0000_0111",
            ),
        ];

        for ((zx, nx, zy, ny, f, no), expected) in cases {
            let expected = word(expected);
            let (out, zr, ng) = alu(x, y, zx, nx, zy, ny, f, no);
            assert_eq!(out, expected);
            assert_eq!(zr, expected == zero);
            assert_eq!(ng, expected[WORD_SIZE - 1]);
        }
    }

    #[test]
    fn test_parse_bits_order_and_zero_extension() {
        // let parsed = parse_bits("1011").unwrap();
        let parsed = dbg!(parse_bits("1011").unwrap());
        assert_eq!(parsed, word("0000_0000_0000_1011"));
        assert!(parsed[0]);
        assert!(parsed[1]);
        assert!(!parsed[2]);
        assert!(parsed[3]);
        assert!(parsed[4..].iter().all(|bit| !bit));
        assert_eq!(
            not16(word("0000_0000_0000_1011")),
            word("1111_1111_1111_0100")
        );
    }

    #[test]
    fn test_parse_bits_rejects_invalid_input() {
        assert_eq!(parse_bits(""), Err(BitStringError::Empty));
        assert_eq!(
            parse_bits("10x1"),
            Err(BitStringError::InvalidCharacter {
                character: 'x',
                byte_index: 2,
            })
        );
        assert_eq!(
            parse_bits("1__0"),
            Err(BitStringError::InvalidSeparator { byte_index: 2 })
        );
        assert_eq!(
            parse_bits("1000_"),
            Err(BitStringError::InvalidSeparator { byte_index: 4 })
        );
        assert_eq!(
            parse_bits("0000_0000_0000_0000_1"),
            Err(BitStringError::TooManyBits { max: WORD_SIZE })
        );
    }

    #[test]
    fn test_parse_bits_bit_extraction() {
        // Test that bits are correctly extracted from LSB to MSB
        let parsed = parse_bits("1").unwrap();
        assert!(parsed[0], "LSB should be 1");
        for (i, bit) in parsed.iter().enumerate().skip(1) {
            assert!(!bit, "All other bits should be 0 (bit {i})");
        }

        let parsed = parse_bits("10").unwrap();
        assert!(!parsed[0], "bit 0 should be 0");
        assert!(parsed[1], "bit 1 should be 1");
        for (i, bit) in parsed.iter().enumerate().skip(2) {
            assert!(!bit, "All other bits should be 0 (bit {i})");
        }

        let parsed = parse_bits("101").unwrap();
        assert!(parsed[0], "bit 0 should be 1");
        assert!(!parsed[1], "bit 1 should be 0");
        assert!(parsed[2], "bit 2 should be 1");
        for (i, bit) in parsed.iter().enumerate().skip(3) {
            assert!(!bit, "All other bits should be 0 (bit {i})");
        }
    }

    #[test]
    fn test_parse_bits_reversed_order() {
        // Test that input string "1101" maps to bit array where:
        // - first bit (input leftmost '1') ends up at highest index
        // - last bit (input rightmost '1') ends up at lowest index
        let parsed = parse_bits("1101").unwrap();
        // Input "1101" should result in: [1, 0, 1, 1, 0, 0, ...]
        // Index:                          0  1  2  3  4  5
        assert_eq!(parsed[0], true, "index 0 should be 1 (input rightmost)");
        assert_eq!(parsed[1], false, "index 1 should be 0");
        assert_eq!(parsed[2], true, "index 2 should be 1");
        assert_eq!(parsed[3], true, "index 3 should be 1 (input leftmost)");
    }

    #[test]
    fn test_parse_bits_sign_bit() {
        // Sign bit is at index 15 (MSB for 16-bit word)
        // Test setting and not setting the sign bit
        let positive = parse_bits("0111_1111_1111_1111").unwrap();
        assert!(
            !positive[WORD_SIZE - 1],
            "sign bit should be 0 for positive"
        );

        let negative = parse_bits("1111_1111_1111_1111").unwrap();
        assert!(negative[WORD_SIZE - 1], "sign bit should be 1 for negative");

        let sign_bit_only = parse_bits("1").unwrap();
        assert!(sign_bit_only[0], "LSB should be 1");
        assert!(!sign_bit_only[WORD_SIZE - 1], "sign bit should be 0");

        let with_sign_bit = parse_bits("1000_0000_0000_0000").unwrap();
        assert!(!with_sign_bit[0], "LSB should be 0");
        assert!(with_sign_bit[WORD_SIZE - 1], "sign bit should be 1");
    }

    #[test]
    fn test_parse_bits_alternating_patterns() {
        // Test alternating 0s and 1s
        let alt_01 = parse_bits("0101_0101_0101_0101").unwrap();
        for (i, _) in alt_01.iter().enumerate().take(WORD_SIZE) {
            if i % 2 == 0 {
                assert!(alt_01[i], "even indices should be 1");
            } else {
                assert!(!alt_01[i], "odd indices should be 0");
            }
        }

        let alt_10 = parse_bits("1010_1010_1010_1010").unwrap();
        for (i, _) in alt_10.iter().enumerate().take(WORD_SIZE) {
            if i % 2 == 0 {
                assert!(!alt_10[i], "even indices should be 0");
            } else {
                assert!(alt_10[i], "odd indices should be 1");
            }
        }
    }

    #[test]
    fn test_parse_bits_power_of_two() {
        // Test that powers of two set exactly one bit
        let one = parse_bits("1").unwrap();
        assert_eq!(one.iter().filter(|&&bit| bit).count(), 1);

        let two = parse_bits("10").unwrap();
        assert_eq!(two.iter().filter(|&&bit| bit).count(), 1);
        assert!(two[1], "bit at index 1 should be set");

        let four = parse_bits("100").unwrap();
        assert_eq!(four.iter().filter(|&&bit| bit).count(), 1);
        assert!(four[2], "bit at index 2 should be set");

        let max_power = parse_bits("1000_0000_0000_0000").unwrap();
        assert_eq!(max_power.iter().filter(|&&bit| bit).count(), 1);
        assert!(max_power[15], "bit at index 15 should be set");
    }

    #[test]
    fn test_parse_bits_with_separators_maintains_correctness() {
        // Test that separators don't affect bit interpretation
        let without_sep = parse_bits("1111000011110000").unwrap();
        let with_sep = parse_bits("1111_0000_1111_0000").unwrap();
        assert_eq!(without_sep, with_sep);

        let without_sep = parse_bits("101010101010101").unwrap();
        let with_sep = parse_bits("1010_1010_1010_101").unwrap();
        assert_eq!(without_sep, with_sep);
    }

    #[test]
    fn test_parse_bits_zero_extension() {
        // Short bit strings should be zero-extended on the left
        let one_bit = parse_bits("1").unwrap();
        let full_word = parse_bits("0000_0000_0000_0001").unwrap();
        assert_eq!(one_bit, full_word);

        let four_bits = parse_bits("1111").unwrap();
        let full_word_4 = parse_bits("0000_0000_0000_1111").unwrap();
        assert_eq!(four_bits, full_word_4);
    }

    #[test]
    fn test_parse_bits_round_trip_with_bit_operations() {
        // Parse a bit string, verify it matches expected array, and verify not16 inverts correctly
        let parsed = parse_bits("1010_0101_1010_0101").unwrap();

        let inverted = not16(parsed);
        let inverted_parsed = parse_bits("0101_1010_0101_1010").unwrap();

        assert_eq!(inverted, inverted_parsed);

        // Double negation should give the original
        assert_eq!(not16(inverted), parsed);
    }
}
