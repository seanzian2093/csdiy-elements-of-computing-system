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
/// | a | b | Nand(a,b) |
/// |---|---|-----------|
/// | 0 | 0 |     1     |
/// | 0 | 1 |     1     |
/// | 1 | 0 |     1     |
/// | 1 | 1 |     0     |
/// 
/// This gate is considered primitive and thus there is no need to implement it.
/// Instead, we use Rust's built-in boolean operators to implement it.
pub fn nand(a: Bit, b: Bit) -> Bit {
    !(a & b)
}

/// ### Not Gate
/// This single-input Not gate, also known as an "converter", converts its input
/// from 0 to 1 and vice versa. Its Boolean function is defined as follows:
/// | a | Not(a) |
/// |---|--------|
/// | 0 |   1    |
/// | 1 |   0    |
/// 
/// We implement the Not gate using the Nand gate as follows:
/// | a | a | Nand(a,a) | Not(a) |
/// |---|---|-----------|--------|
/// | 0 | 0 |     1     |   1    |
/// | 1 | 1 |     0     |   0    |
pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

/// ### And Gate
/// The And gate is a two-input gate that outputs 1 only when both inputs are 1, and 0 otherwise.
/// Boolean function:
/// | a | b | And(a,b) |
/// |---|---|----------|
/// | 0 | 0 |    0     |
/// | 0 | 1 |    0     |
/// | 1 | 0 |    0     |
/// | 1 | 1 |    1     |
/// 
/// We implement the And gate using the Nand and Not gates as follows:
/// | a | b | Nand(a,b) | Not(Nand(a,b)) |
/// |---|---|-----------|----------------|
/// | 0 | 0 |     1     |        0       |
/// | 0 | 1 |     1     |        0       |
/// | 1 | 0 |     1     |        0       |
/// | 1 | 1 |     0     |        1       |
pub fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

/// ### Or Gate
/// The Or gate returns 1 if at least one of its inputs is 1, and 0 otherwise.
/// Boolean function:
/// | a | b | Or(a,b) |
/// |---|---|---------|
/// | 0 | 0 |    0    |
/// | 0 | 1 |    1    |
/// | 1 | 0 |    1    |
/// | 1 | 1 |    1    |
/// 
/// We implement the Or gate using Nand gates as follows (by De Morgan's Law):
/// 
/// First approach (using Not and Nand):
/// | a | b | Not(a) | Not(b) | Nand(Not(a), Not(b)) | Or(a,b) |
/// |---|---|--------|--------|--------------------|---------|
/// | 0 | 0 |   1    |   1    |         0          |    0    |
/// | 0 | 1 |   1    |   0    |         1          |    1    |
/// | 1 | 0 |   0    |   1    |         1          |    1    |
/// | 1 | 1 |   0    |   0    |         1          |    1    |
///
/// More efficient implementation (using only Nand gates):
/// | a | b | Nand(a,a) | Nand(b,b) | Nand(Nand(a,a), Nand(b,b)) | Or(a,b) |
/// |---|---|-----------|-----------|-------|---------|
/// | 0 | 0 |     1     |     1     |   0   |    0    |
/// | 0 | 1 |     1     |     0     |   1   |    1    |
/// | 1 | 0 |     0     |     1     |   1   |    1    |
/// | 1 | 1 |     0     |     0     |   1   |    1    |
pub fn or(a: Bit, b: Bit) -> Bit {
    nand(nand(a, a), nand(b, b))
}

/// ### Xor Gate
/// The Xor gate, also known as "exclusive or", returns 1 if exactly one of its inputs is 1, and 0 otherwise.
/// Boolean function:
/// | a | b | Xor(a,b) |
/// |---|---|----------|
/// | 0 | 0 |    0     |
/// | 0 | 1 |    1     |
/// | 1 | 0 |    1     |
/// | 1 | 1 |    0     |
///
/// We implement the Xor gate using Nand gates as follows:
/// | a | b | w1=Nand(a,b) | w2=Nand(a, w1) | w3=Nand(b, w1) | Xor(a,b)=Nand(w2, w3) |
/// |---|---|--------------|----------------|----------------|-----------------------|
/// | 0 | 0 |     1        |        1       |       0        |           0           |
/// | 0 | 1 |     1        |        1       |       1        |           1           |      
/// | 1 | 0 |     1        |        0       |       1        |           1           |
/// | 1 | 1 |     0        |        1       |       0        |           0           |
pub fn xor(a: Bit, b: Bit) -> Bit {
    let w1 = nand(a, b);
    let w2 = nand(a, w1);
    let w3 = nand(b, w1);
    nand(w2, w3)
}


/// ### Multiplexer (Mux)
/// The Multiplexer (Mux) is a three-input gate that uses one of the inputs, called "selection bit", to select and output one of the other two inputs, called "data bits".
/// THus a better name could be "selector", i.e.,: 
/// | sel | output |
/// |-----|--------|
/// |  0  |   a    |
/// |  1  |   b    |
/// 
/// Boolean function:
/// | a | b | sel | Mux(sel,a,b) |
/// |---|---|-----|--------------|
/// | 0 | 0 |  0  |      0       |
/// | 0 | 1 |  0  |      0       |
/// | 1 | 0 |  0  |      1       |
/// | 1 | 1 |  0  |      1       |
/// | 0 | 0 |  1  |      0       |
/// | 0 | 1 |  1  |      1       |
/// | 1 | 0 |  1  |      0       |
/// | 1 | 1 |  1  |      1       |
/// 
/// We implement the Mux gate using Nand gates as follows:
/// | a | b | sel | Not(sel) | w1=And(a, Not(sel)) | w2=And(b, sel) | Mux(sel,a,b)=Or(w1, w2) |
/// |---|---|-----|----------|--------------------|----------------|-----------------------|
/// | 0 | 0 |  0  |     1    |         0          |       0        |           0           |
/// | 0 | 1 |  0  |     1    |         0          |       0        |           0           |
/// | 1 | 0 |  0  |     1    |         1          |       0        |           1           |
/// | 1 | 1 |  0  |     1    |         1          |       0        |           1           |
/// | 0 | 0 |  1  |     0    |         0          |       0        |           0           |
/// | 0 | 1 |  1  |     0    |         0          |       1        |           1           |
/// | 1 | 0 |  1  |     0    |         0          |       0        |           0           |
/// | 1 | 1 |  1  |     0    |         0          |       1        |           1           |
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
/// | sel | out[0] | out[1] |
/// |-----|--------|--------|
/// |  0  | input  |   0    |
/// |  1  |   0    | input  |
///
/// Boolean function:
/// | input | sel | DMux(input,sel) = (out[0], out[1]) |
/// |-------|-----|------------------------------------|
/// |   0   |  0  |          (0, 0)                    |
/// |   0   |  1  |          (0, 0)                    |
/// |   1   |  0  |          (1, 0)                    |
/// |   1   |  1  |          (0, 1)                    |
///
/// We implement the DMux gate using the And and Not gates as follows:
/// | input | sel | Not(sel) | out[0]=And(input, Not(sel)) | out[1]=And(input, sel) |
/// |-------|-----|----------|-----------------------------|------------------------|
/// |   0   |  0  |    1     |              0              |            0           |
/// |   0   |  1  |    0     |              0              |            0           |
/// |   1   |  0  |    1     |              1              |            0           |
/// |   1   |  1  |    0     |              0              |            1           |
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
/// A 16-bit word is 16 single bits, held in one fixed-width value. Every 16-bit gate
/// below is
/// built from its single-bit counterpart by wiring 16 identical single-bit chips in
/// parallel: bit `i` of each operand feeds chip `i`, and chip `i`'s output becomes
/// bit `i` of the result.
///
/// | bit | a      | b      |
/// |-----|--------|--------|
/// |  0  | a[0]   | b[0]   |
/// | …   |  …     |  …     |
/// | 15  | a[15]  | b[15]  |
///
/// This mirrors the Nand2tetris convention, where `And16`, `Or16`, `Not16` and
/// `Mux16` are each a bundle of 16 of the single-bit gates defined above.
///
/// The number of bits in one word.
pub const WORD_SIZE: usize = 16;

/// The canonical 16-bit word: the value that flows through every 16-bit gate and,
/// above this layer, through the ALU, registers, and memory.
///
/// It is backed by a `u16`, so arithmetic, sign, and equality come for free and the
/// width is fixed by the type. Bit `0` is the *least*-significant bit. This is the
/// representation the single-bit gates above [Bit] feed into.
///
/// It renders as its 16 bits most-significant first, grouped in bytes of four and
/// joined by `_` — e.g. `0xffff` prints as `"1111_1111_1111_1111"`. Parsing
/// ([`std::str::FromStr`]) is the exact inverse, which is why `"1010_1010_1010_1010"`
/// is the number `0xaa` and its bit `0` is `0`: the string and the number agree, with
/// no per-element bit order to keep track of.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Word(pub u16);

impl Word {
    /// All bits clear.
    pub const ZERO: Word = Word(0x0000);
    /// All bits set: the two's-complement value `-1`.
    pub const ONES: Word = Word(0xffff);

    /// The value of bit `i`, where `i = 0` is the least-significant bit.
    pub fn bit(self, i: usize) -> Bit {
        (self.0 >> i) & 1 == 1
    }

    /// Whether the most-significant bit is set, i.e. the value is negative under
    /// two's-complement interpretation.
    pub fn is_negative(self) -> Bit {
        self.bit(WORD_SIZE - 1)
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // MSB first, four bits per group, groups joined by '_'.
        let mut first_group = true;
        for group in 0..WORD_SIZE / 4 {
            if !first_group {
                write!(f, "_")?;
            }
            first_group = false;
            // Group `group` (0 = most significant) holds bits `[hi - 3 ..= hi]`;
            // print them most-significant first.
            let hi = WORD_SIZE - 4 * group - 1;
            for i in ((hi - 3)..=hi).rev() {
                write!(f, "{}", self.bit(i) as u8)?;
            }
        }
        Ok(())
    }
}

/// Error returned when a string is not a valid [`Word`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseWordError {
    /// A character other than `0`, `1`, or the `_` separator was found.
    InvalidChar { got: char },
    /// The string did not contain exactly 16 bits.
    WrongBitCount { got: usize },
}

impl std::fmt::Display for ParseWordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseWordError::InvalidChar { got } => write!(f, "invalid bit character {got:?}"),
            ParseWordError::WrongBitCount { got } => {
                write!(f, "expected {WORD_SIZE} bits, found {got}")
            }
        }
    }
}

impl std::error::Error for ParseWordError {}

impl std::str::FromStr for Word {
    type Err = ParseWordError;

    fn from_str(s: &str) -> Result<Word, ParseWordError> {
        let mut value = 0u16;
        let mut bits = 0usize;
        for c in s.chars() {
            match c {
                '0' | '1' => {
                    value = (value << 1) | u16::from(c == '1');
                    bits += 1;
                }
                '_' => {} // byte separator
                c => return Err(ParseWordError::InvalidChar { got: c }),
            }
        }
        if bits != WORD_SIZE {
            return Err(ParseWordError::WrongBitCount { got: bits });
        }
        Ok(Word(value))
    }
}

/// ### Not16
/// The 16-bit Not gate inverts every bit of its input, bit by bit.
///
/// It is 16 independent [Not](not) gates wired in parallel: `not16(a)[i] = not(a[i])`,
/// i.e. the one's complement. Compiled to a native bitwise NOT.
pub fn not16(a: Word) -> Word {
    Word(!a.0)
}

/// ### And16
/// Bit `i` of the result is the And of bit `i` of its two operands: 16 independent
/// [And](and) gates in parallel, `and16(a, b)[i] = and(a[i], b[i])`. Compiled to a
/// native bitwise And.
pub fn and16(a: Word, b: Word) -> Word {
    Word(a.0 & b.0)
}

/// ### Or16
/// Bit `i` of the result is the Or of bit `i` of its two operands: 16 independent
/// [Or](or) gates in parallel, `or16(a, b)[i] = or(a[i], b[i])`. Compiled to a native
/// bitwise Or.
pub fn or16(a: Word, b: Word) -> Word {
    Word(a.0 | b.0)
}

/// ### Mux16
/// The 16-bit Multiplexer selects, as a whole word, between two 16-bit data words
/// using a single selection bit. When `sel` is 0 the entire word `a` passes through,
/// and when `sel` is 1 the entire word `b` passes through:
///
/// | sel | Mux16(a, b, sel) |
/// |-----|------------------|
/// |  0  |        a         |
/// |  1  |        b         |
///
/// It is 16 independent [Mux](mux) gates wired in parallel, all sharing the same
/// `sel`: `mux16(a, b, sel)[i] = mux(a[i], b[i], sel)`.
pub fn mux16(a: Word, b: Word, sel: Bit) -> Word {
    Word(if sel { b.0 } else { a.0 })
}


/// ## Adders and the ALU
///
/// The full adder is the last single-bit building block; the 16-bit adder chains 16 of
/// them as a ripple-carry chain, and the ALU composes the 16-bit gates into the
/// arithmetic core of a Nand2tetris processor.
///
/// Note the seam in representation: the full adder still works on `Bit`, but from the
/// adder upward everything is a [`Word`]. The adder's body is built bit by bit because
/// the carry chain is the structure worth showing; the ALU above it works on words and
/// treats the adder as a black box.

/// ### Half Adder
/// Sums two single bits and produces a carry out.
///
/// | a | b | sum | carry |
/// |---|---|-----|-------|
/// | 0 | 0 |  0  |   0   |
/// | 0 | 1 |  1  |   0   |
/// | 1 | 0 |  1  |   0   |
/// | 1 | 1 |  0  |   1   |
pub fn half_adder(a: Bit, b: Bit) -> (Bit, Bit) {
    (xor(a, b), and(a, b))
}

/// ### Full Adder
/// Sums two single bits plus a carry in, producing a sum and a carry out. It is two
/// [half_adder](half_adder)s in series: the first sums the two data bits, the second
/// folds in the carry.
///
/// | a | b | cin | sum | cout |
/// |---|---|-----|-----|------|
/// | 0 | 0 |  0  |  0  |  0   |
/// | 0 | 0 |  1  |  1  |  0   |
/// | 0 | 1 |  0  |  1  |  0   |
/// | 0 | 1 |  1  |  0  |  1   |
/// | 1 | 0 |  0  |  1  |  0   |
/// | 1 | 0 |  1  |  0  |  1   |
/// | 1 | 1 |  0  |  0  |  1   |
/// | 1 | 1 |  1  |  1  |  1   |
pub fn full_adder(a: Bit, b: Bit, cin: Bit) -> (Bit, Bit) {
    let (sum, carry) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum, cin);
    (sum, or(carry, carry2))
}

/// ### Add16
/// Adds two 16-bit words plus an optional carry in, in two's complement.
///
/// It is 16 [full_adder](full_adder)s wired as a *ripple-carry* chain: the carry out of
/// stage `i` is the carry in of stage `i + 1`, bit `0` first. Because the carry chain is
/// the structure worth showing, this one is built bit by bit rather than compiled to a
/// native add. A carry in of `true` makes this `a + b + 1`, which is how two's-complement
/// negation is obtained (`not16(a) + 1`).
pub fn add16(a: Word, b: Word, carry_in: Bit) -> Word {
    let mut carry = carry_in;
    let mut out = 0u16;
    for i in 0..WORD_SIZE {
        let (sum, cout) = full_adder(a.bit(i), b.bit(i), carry);
        out |= (u16::from(sum) << i);
        carry = cout;
    }
    Word(out)
}

/// The 3-bit control field of the [ALU](alu).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AluControl {
    /// Select the arithmetic operand: `a` when `false`, `b` when `true`.
    pub in_out: Bit,
    /// Zero out the second operand (mask it to `0`).
    pub iz: Bit,
    /// Negate the first operand (one's complement).
    pub nz: Bit,
}

/// The output of the [ALU](alu): the 16-bit result and its status flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AluResult {
    /// The 16-bit result.
    pub out: Word,
    /// Set when `out` is negative (two's-complement MSB set).
    pub n: Bit,
    /// Set when `out` is zero.
    pub z: Bit,
    /// Set when `out` is positive (neither zero nor negative).
    pub p: Bit,
}

/// ### ALU
/// The arithmetic logic unit: it combines two 16-bit inputs `a` and `b` under the
/// 3-bit [AluControl](AluControl) field and produces a 16-bit output with three flags.
///
/// It follows the Nand2tetris wiring:
/// 1. `x = mux16(a, b, in_out)` — pick the first operand.
/// 2. `x = nz ? not16(x) : x` — optionally negate it.
/// 3. `y = iz ? not16(b) : b` — take the second operand, negated when `iz` is set.
/// 4. `arith = add16(x, y, iz)` — add, carrying `iz` so `y` becomes `b`, `~b`, or `-b`.
/// 5. `out = mux16(arith, iz ? -1 : 0, in_out)` — the arithmetic, or the constant `0`/`-1`.
///
/// The flags are word-level reductions that would be painful over a bit array but are
/// trivial here. (The instruction's jump bits `jm`/`je`/`jg` are consumed by the CPU's
/// program-counter logic in a later chapter, not by the ALU.)
pub fn alu(a: Word, b: Word, c: AluControl) -> AluResult {
    let x = mux16(a, b, c.in_out);
    let x = if c.nz { not16(x) } else { x };
    let y = if c.iz { not16(b) } else { b };
    let arith = add16(x, y, c.iz);
    let constant = if c.iz { Word::ONES } else { Word::ZERO };
    let out = mux16(arith, constant, c.in_out);

    let z = out == Word::ZERO;
    let n = out.is_negative();
    let p = !z && !n;
    AluResult { out, n, z, p }
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

    // A few named 16-bit test words, written as 0b literals. Bit 0 (the LSB) is the
    // rightmost bit, so `0x5555` is the alternating pattern starting at bit 0.
    const ZEROS: Word = Word(0x0000);
    const ONES: Word = Word(0xffff);
    // Only the low 8 bits set.
    const LOW: Word = Word(0x00ff);
    // Alternating, and its complement (shifted by one bit).
    const ALT: Word = Word(0x5555);
    const ALT_X: Word = Word(0xaaaa);

    #[test]
    fn test_not16_table() {
        // Not16 inverts every bit, so each word maps to its complement.
        for (input, expected) in [
            (ZEROS, ONES),
            (ONES, ZEROS),
            (ALT, ALT_X),
            (ALT_X, ALT),
        ] {
            assert_eq!(not16(input), expected);
        }
    }

    #[test]
    fn test_and16_table() {
        // and16 sets a bit only where both operands have it set.
        for (a, b, expected) in [
            (ZEROS, ONES, ZEROS),
            (ONES, ZEROS, ZEROS),
            (ONES, ONES, ONES),
            (LOW, ONES, LOW), // AND with all-1s is the identity
            (ALT, ALT, ALT),
            (ALT, ALT_X, ZEROS), // the two patterns never agree
        ] {
            assert_eq!(and16(a, b), expected);
        }
    }

    #[test]
    fn test_or16_table() {
        // or16 sets a bit wherever either operand has it set.
        for (a, b, expected) in [
            (ZEROS, ZEROS, ZEROS),
            (ONES, ZEROS, ONES),
            (ZEROS, ONES, ONES),
            (LOW, ZEROS, LOW), // OR with all-0s is the identity
            (ALT, ALT, ALT),
            (ALT, ALT_X, ONES), // the two patterns complement each other
        ] {
            assert_eq!(or16(a, b), expected);
        }
    }

    #[test]
    fn test_mux16_table() {
        // sel = 0 passes the whole word `a` through; sel = 1 passes `b`.
        for (a, b) in [(LOW, ALT), (ALT_X, ONES), (ZEROS, ONES)] {
            assert_eq!(mux16(a, b, false), a);
            assert_eq!(mux16(a, b, true), b);
        }
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
        // (a, b, cin) -> (sum, cout) over all 8 inputs.
        for (a, b, cin, sum, cout) in [
            (false, false, false, false, false),
            (false, false, true, true, false),
            (false, true, false, true, false),
            (false, true, true, false, true),
            (true, false, false, true, false),
            (true, false, true, false, true),
            (true, true, false, false, true),
            (true, true, true, true, true),
        ] {
            assert_eq!(full_adder(a, b, cin), (sum, cout));
        }
    }

    #[test]
    fn test_add16() {
        assert_eq!(add16(Word(0), Word(0), false), Word(0));
        assert_eq!(add16(Word(1), Word(1), false), Word(2));
        assert_eq!(add16(Word(0x1234), Word(0x0012), false), Word(0x1246));
        assert_eq!(add16(Word(0xffff), Word(1), false), Word(0)); // overflow wraps
        assert_eq!(add16(Word(0xffff), Word(0), true), Word(0)); // plus carry in
    }

    #[test]
    fn test_alu_arithmetic() {
        let a = Word(0x0005);
        let b = Word(0x0003);
        // A + B
        assert_eq!(alu(a, b, AluControl::default()).out, Word(0x0008));
        // A - B
        assert_eq!(
            alu(a, b, AluControl { iz: true, ..Default::default() }).out,
            Word(0x0002)
        );
        // constant 0
        assert_eq!(
            alu(a, b, AluControl { in_out: true, ..Default::default() }).out,
            Word::ZERO
        );
        // constant -1
        assert_eq!(
            alu(a, b, AluControl { in_out: true, iz: true, ..Default::default() }).out,
            Word::ONES
        );
    }

    #[test]
    fn test_alu_flags() {
        // 5 - 3 = 2 > 0: positive.
        let r = alu(Word(5), Word(3), AluControl { iz: true, ..Default::default() });
        assert_eq!(r.out, Word(2));
        assert_eq!((r.n, r.z, r.p), (false, false, true));
        // The constant 0: zero flag.
        let r = alu(Word(0), Word(0), AluControl { in_out: true, ..Default::default() });
        assert_eq!(r.out, Word::ZERO);
        assert_eq!((r.n, r.z, r.p), (false, true, false));
        // 3 - 5 = -2: negative.
        let r = alu(Word(3), Word(5), AluControl { iz: true, ..Default::default() });
        assert_eq!(r.out, Word(0xfffe));
        assert_eq!((r.n, r.z, r.p), (true, false, false));
    }

    #[test]
    fn test_word_display_and_parse() {
        use std::str::FromStr;
        assert_eq!(format!("{}", ZEROS), "0000_0000_0000_0000");
        assert_eq!(format!("{}", ONES), "1111_1111_1111_1111");
        assert_eq!(format!("{}", Word(0x5a3c)), "0101_1010_0011_1100");
        // Parsing is the inverse of Display.
        assert_eq!(Word::from_str("0101_1010_0011_1100").unwrap(), Word(0x5a3c));
        // The separator is optional.
        assert_eq!(Word::from_str("0101101000111100").unwrap(), Word(0x5a3c));
        // Errors: wrong bit count and a non-bit character.
        assert!(Word::from_str("101").is_err());
        assert!(Word::from_str("1010_1010_1010_1012").is_err());
    }
}
