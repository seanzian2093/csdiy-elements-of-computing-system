/// # Chapter 1: Boolean Logic
/// 
/// A single bit: the smallest signal in any digital circuit.
///
/// ## Basic Signals
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

#[cfg(test)]
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
}
