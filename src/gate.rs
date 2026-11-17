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
/// A 16-bit word is simply a vector of 16 single bits. Every 16-bit gate below is
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

/// The number of bits in one word.
pub const WORD_SIZE: usize = 16;

/// A 16-bit word: a run of 16 single bits, indexed `0..15`.
///
/// Bit `0` is the least-significant bit.
pub type Bits = [Bit; WORD_SIZE];

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
/// |-----|------------------|
/// |  0  |        a         |
/// |  1  |        b         |
///
/// It is 16 independent [Mux](mux) gates wired in parallel, all sharing the same
/// `sel`: `mux16(a, b, sel)[i] = mux(a[i], b[i], sel)`.
pub fn mux16(a: Bits, b: Bits, sel: Bit) -> Bits {
    std::array::from_fn(|i| mux(a[i], b[i], sel))
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

    // Build a 16-bit word from a `u16`, bit 0 = bit 0.
    fn bits_from(value: u16) -> Bits {
        std::array::from_fn(|i| (value >> i) & 1 == 1)
    }

    // Convert a 16-bit word back to a `u16` for easy comparison.
    fn to_u16(bits: Bits) -> u16 {
        bits.iter().enumerate().fold(0u16, |acc, (i, &b)| acc | (b as u16) << i)
    }

    #[test]
    fn test_not16_table() {
        // Inverting every bit is the same as `0xFFFF ^ x`, i.e. one's complement.
        for value in [0x0000u16, 0x0001, 0x000F, 0x8000, 0x5555, 0xABCD, 0xFFFF] {
            assert_eq!(to_u16(not16(bits_from(value))), value ^ 0xFFFF);
        }
    }

    #[test]
    fn test_and16_table() {
        for value in [0x0000u16, 0x0001, 0x000F, 0x8000, 0x5555, 0xABCD, 0xFFFF] {
            for other in [0x0000u16, 0x0001, 0x0F0F, 0x8000, 0x5555, 0xABCD, 0xFFFF] {
                assert_eq!(to_u16(and16(bits_from(value), bits_from(other))), value & other);
            }
        }
    }

    #[test]
    fn test_or16_table() {
        for value in [0x0000u16, 0x0001, 0x000F, 0x8000, 0x5555, 0xABCD, 0xFFFF] {
            for other in [0x0000u16, 0x0001, 0x0F0F, 0x8000, 0x5555, 0xABCD, 0xFFFF] {
                assert_eq!(to_u16(or16(bits_from(value), bits_from(other))), value | other);
            }
        }
    }

    #[test]
    fn test_mux16_table() {
        // sel = 0 selects the whole word `a`; sel = 1 selects the whole word `b`.
        for value in [0x0000u16, 0x0001, 0x000F, 0x8000, 0x5555, 0xABCD, 0xFFFF] {
            for other in [0x0000u16, 0x0001, 0x0F0F, 0x8000, 0x5555, 0xABCD, 0xFFFF] {
                assert_eq!(to_u16(mux16(bits_from(value), bits_from(other), false)), value);
                assert_eq!(to_u16(mux16(bits_from(value), bits_from(other), true)), other);
            }
        }
    }
}
