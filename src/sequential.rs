//! # Chapter 3 - Sequential Logic
//!
//! All Boolean and arithmetic chips that we built in chapter 1 and 2 were combinational.
//! Combinational chips compute functions that depend only on combinations of their inputs.
//! They cannot maintain state.
//!
//! Computers must be able to not only compute values but also store and recall values. They
//! must be qeuipped with memory elements that can preserve data over time. These memeory
//! elements are built from sequential chips.
//!

use crate::gate::{Bit, Bits, WORD_SIZE, inc16, mux};

/// ## Clock
/// The hardware implementation of a master clock is usually based on a oscillator that
/// alternates continously between two phases labeled 0-1, low-high, or tick-tock, etc..
/// The elapased time bwteen the beginning of a **tick** and the end of a **tock** is
/// called a **cycle**. The current clock phase is represented by a binary signal.
pub struct Clock {
    cycles: u64,
}

impl Clock {
    // `new()` without argument, i.e. initial value(s), will not trigger a clippy warning.
    // but a `new()` with argument(s) will trigger a warning if there is no `default()`.
    // so we could add an attribute to allow `new()` without `default()`.
    // though define a `default()` is idomatic.
    // #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { cycles: 0 }
    }

    pub fn tick(&mut self) {
        self.cycles += 1;
    }

    pub fn cycles(&self) -> u64 {
        self.cycles
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

/// ## D Flip-Flop
/// The most elementary sequential element in a computer is a device called a **flip-flop**.
/// In this implementation, we will use a **D flip-flop** (DFF) which has a single data input
/// and a single output. In addition, the **DFF** has a clock input that continuously changes
/// according to the master clock's signal.
pub struct Dff {
    q: Bit,
}

impl Dff {
    pub fn new(initial: Bit) -> Self {
        Self { q: initial }
    }

    pub fn output(&self) -> Bit {
        self.q
    }

    pub fn tick(&mut self, input: Bit) {
        self.q = input;
    }
}

/// ## Single-Bit Register
/// A single-bit register, called **Bit** or **binary cell**, is designed to store a single bit
/// of information(0 or 1). The chip interface consists of an input pin that carries a data bit
/// a load pin that enables the cell for writes, and an output pin that emits the current state
/// of the cell.
pub struct Register {
    dff: Dff,
}

impl Register {
    pub fn new(initial: Bit) -> Self {
        Self {
            dff: Dff::new(initial),
        }
    }

    pub fn output(&self) -> Bit {
        self.dff.output()
    }

    pub fn read(&self) -> Bit {
        self.output()
    }

    pub fn tick(&mut self, input: Bit, load: Bit) {
        let next = mux(self.output(), input, load);
        self.dff.tick(next);
    }
}

/// ## 16-Bit Register
/// A 16-bit register stores a 16-bit input word. It consists of 16 single-bit
/// registers, one for each bit position.
pub struct Register16 {
    registers: [Register; WORD_SIZE],
}

impl Register16 {
    pub fn new(initial: Bits) -> Self {
        Self {
            registers: std::array::from_fn(|index| Register::new(initial[index])),
        }
    }

    pub fn output(&self) -> Bits {
        std::array::from_fn(|index| self.registers[index].output())
    }

    pub fn read(&self) -> Bits {
        self.output()
    }

    pub fn tick(&mut self, input: Bits, load: Bit) {
        for (index, register) in self.registers.iter_mut().enumerate() {
            register.tick(input[index], load);
        }
    }
}

/// ## Memory
/// A direct-access memory (RAM) is an array of n w-bit registers of which n is size and w is width.
/// For this implementation, we will build a hierarchy of such memory devices, all 16-bit wide, with sizes
/// of 8, 64, 512, 4k, and 16k. All have precisely same API which can be generated as:
/// RAM8, inputs: `in[16]`, `address[3]`, `load`
/// RAM64, inputs: `in[16]`, `address[6]`, `load`
/// RAM512, inputs: `in[16]`, `address[9]`, `load`
/// RAM4k, inputs: `in[16]`, `address[12]`, `load`
/// RAM16k, inputs: `in[16]`, `address[14]`, `load`
///
/// A three-bit address, with bit 0 as the least-significant bit.
pub type Address3 = [Bit; 3];
/// A six-bit address, with bit 0 as the least-significant bit.
pub type Address6 = [Bit; 6];

/// ### RAM8
/// RAM8 is a memory unit of 8 16-bit registers, also called 8-word memory.
///
pub struct RAM8 {
    registers: [Register16; 8],
}

impl RAM8 {
    // new RAM8 instance start with all `false`. It’s an explicit implementation choice, not guaranteed.
    // RAM8::new() initializes each word to all false bits (zero), giving the simulator a predictable starting state.
    // Real RAM may have unspecified contents at power-on unless the hardware provides initialization.
    pub fn new() -> Self {
        Self {
            registers: std::array::from_fn(|_| Register16::new([false; WORD_SIZE])),
        }
    }

    pub fn output(&self, address: Address3) -> Bits {
        self.registers[Self::address_index(address)].output()
    }

    pub fn read(&self, address: Address3) -> Bits {
        self.output(address)
    }

    pub fn tick(&mut self, input: Bits, address: Address3, load: Bit) {
        // Only one of the eight registers is selected for loading, based on the three-bit address.
        let selected = Self::address_index(address);
        for (index, register) in self.registers.iter_mut().enumerate() {
            // Load and save the `input` only for the selected register when `load` is true
            register.tick(input, load && index == selected);
        }
    }

    // Interpret the three bits as a binary number and returns its numeric value, which is used as a decimal array index.
    fn address_index(address: Address3) -> usize {
        usize::from(address[0]) + 2 * usize::from(address[1]) + 4 * usize::from(address[2])
    }
}

impl Default for RAM8 {
    fn default() -> Self {
        Self::new()
    }
}

/// ### RAM64
/// RAM64 is built from 8 RAM8 chips. Its address is 6 bits wide, i.e., k=6
/// The low 3 address bits select a register, called a word, from the 8 registers within a RAM8 chip
/// and the high 3 bits select a RAM8 chip, called a bank, from those 8 RAM8 chips.
pub struct RAM64 {
    banks: [RAM8; 8],
}

impl RAM64 {
    pub fn new() -> Self {
        Self {
            banks: std::array::from_fn(|_| RAM8::new()),
        }
    }

    pub fn output(&self, address: Address6) -> Bits {
        let (word_address, bank_address) = Self::split_address(address);
        self.banks[RAM8::address_index(bank_address)].output(word_address)
    }

    pub fn read(&self, address: Address6) -> Bits {
        self.output(address)
    }

    pub fn tick(&mut self, input: Bits, address: Address6, load: Bit) {
        let (word_address, bank_address) = Self::split_address(address);
        let selected_bank = RAM8::address_index(bank_address);

        for (index, bank) in self.banks.iter_mut().enumerate() {
            bank.tick(input, word_address, load && index == selected_bank);
        }
    }

    fn split_address(address: Address6) -> (Address3, Address3) {
        (
            [address[0], address[1], address[2]],
            [address[3], address[4], address[5]],
        )
    }
}

impl Default for RAM64 {
    fn default() -> Self {
        Self::new()
    }
}

/// ### Counter
/// The interface of a counter is similarto that of a register, except that it has two additional control
/// bits labeled **reset** and **inc**. WHen `inc=1`, the counter increments its statein every clock cycle,
/// emitting the value `out(t)=out(t-1)+1`. If we want to reset the counter to 0, we set `reset=1`. If we
/// want to initialize it to some ther counting base **d**, we put **d** in the **in** input and set `load=1`.
pub struct Counter {
    register: Register16,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            register: Register16::new([false; WORD_SIZE]),
        }
    }

    pub fn output(&self) -> Bits {
        self.register.output()
    }

    pub fn read(&self) -> Bits {
        self.output()
    }

    pub fn tick(&mut self, input: Bits, load: Bit, inc: Bit, reset: Bit) {
        let current = self.output();
        let next = if reset {
            [false; WORD_SIZE]
        } else if load {
            input
        } else if inc {
            inc16(current)
        } else {
            current
        };

        self.register.tick(next, true);
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::parse_bits;

    #[test]
    fn register_starts_with_initial_value() {
        assert!(!Register::new(false).output());
        assert!(Register::new(true).output());
    }

    #[test]
    fn register_not_loads_on_tick() {
        let mut register = Register::new(false);
        register.tick(true, false);
        assert!(!register.output());

        register = Register::new(true);
        register.tick(false, false);
        assert!(register.output());
    }

    #[test]
    fn register_loads_value_on_tick() {
        let mut register = Register::new(false);
        register.tick(true, true);
        assert!(register.output());

        register.tick(false, true);
        assert!(!register.output());
    }

    #[test]
    fn register16_starts_with_initial_value() {
        let initial = parse_bits("0101_0101_0101_0101").expect("valid test bit string");
        let register = Register16::new(initial);

        assert_eq!(register.output(), initial);
    }

    #[test]
    fn register16_not_loads_on_tick() {
        let zeros = parse_bits("0000_0000_0000_0000").expect("valid test bit string");
        let ones = parse_bits("1111_1111_1111_1111").expect("valid test bit string");
        let mut register = Register16::new(zeros);

        register.tick(ones, false);
        assert_eq!(register.output(), zeros);
    }

    #[test]
    fn register16_loads_input_on_tick() {
        let zeros = parse_bits("0000_0000_0000_0000").expect("valid test bit string");
        let input = parse_bits("1010_1010_1010_1010").expect("valid test bit string");
        let mut register = Register16::new(zeros);

        register.tick(input, true);
        assert_eq!(register.output(), input);
    }

    #[test]
    fn ram8_starts_with_zero_in_every_address() {
        let ram = RAM8::new();

        for address in 0..8 {
            assert_eq!(ram.read(test_address(address)), [false; WORD_SIZE]);
        }
    }

    #[test]
    fn ram8_loads_and_reads_only_the_selected_address() {
        let mut ram = RAM8::new();
        let first = parse_bits("1010_1010_1010_1010").expect("valid test bit string");
        let second = parse_bits("0101_0101_0101_0101").expect("valid test bit string");

        ram.tick(first, test_address(2), true);
        ram.tick(second, test_address(5), true);

        assert_eq!(ram.read(test_address(2)), first);
        assert_eq!(ram.read(test_address(5)), second);
        assert_eq!(ram.read(test_address(0)), [false; WORD_SIZE]);
    }

    #[test]
    fn ram8_preserves_selected_value_when_not_loads() {
        let mut ram = RAM8::new();
        let stored = parse_bits("1010_1010_1010_1010").expect("valid test bit string");
        let not_loaded = parse_bits("0101_0101_0101_0101").expect("valid test bit string");
        let address = test_address(3);

        // Load the `stored` value into the RAM at the specified address
        ram.tick(stored, address, true);
        // Next cycle, with `load` set to `false`, the RAM does not load the `not_loaded` value, so the previously `stored` value is still there.
        ram.tick(not_loaded, address, false);

        // When read, `stored` is still the output.
        assert_eq!(ram.read(address), stored);
    }

    #[test]
    fn ram64_loads_and_reads_only_the_selected_address() {
        let mut ram = RAM64::new();
        let first = parse_bits("1010_1010_1010_1010").expect("valid test bit string");
        let second = parse_bits("0101_0101_0101_0101").expect("valid test bit string");

        for (address, input) in [(7, first), (8, second), (45, first), (63, second)] {
            ram.tick(input, test_address(address), true);
        }

        assert_eq!(ram.read(test_address(7)), first);
        assert_eq!(ram.read(test_address(8)), second);
        assert_eq!(ram.read(test_address(45)), first);
        assert_eq!(ram.read(test_address(63)), second);
        assert_eq!(ram.read(test_address(0)), [false; WORD_SIZE]);
    }

    #[test]
    fn ram64_preserves_selected_value_when_not_loads() {
        let mut ram = RAM64::new();
        let stored = parse_bits("1010_1010_1010_1010").expect("valid test bit string");
        let not_loaded = parse_bits("0101_0101_0101_0101").expect("valid test bit string");
        let address = test_address(37);

        ram.tick(stored, address, true);
        ram.tick(not_loaded, address, false);

        assert_eq!(ram.read(address), stored);
    }

    #[test]
    fn counter_starts_at_zero_and_holds_when_no_control_is_set() {
        let mut counter = Counter::new();
        let zeros = parse_bits("0000_0000_0000_0000").expect("valid test bit string");
        let input = parse_bits("1010_1010_1010_1010").expect("valid test bit string");

        assert_eq!(counter.output(), zeros);
        counter.tick(input, false, false, false);
        assert_eq!(counter.output(), zeros);
    }

    #[test]
    fn counter_loads_and_increments() {
        let mut counter = Counter::new();
        let input = parse_bits("0000_0000_0000_0101").expect("valid test bit string");
        let six = parse_bits("0000_0000_0000_0110").expect("valid test bit string");

        counter.tick(input, true, false, false);
        assert_eq!(counter.output(), input);

        counter.tick([false; WORD_SIZE], false, true, false);
        assert_eq!(counter.output(), six);
    }

    #[test]
    fn counter_reset_has_priority_over_load_and_increment() {
        let mut counter = Counter::new();
        let input = parse_bits("0000_0000_0000_0101").expect("valid test bit string");

        counter.tick(input, true, true, true);
        assert_eq!(counter.output(), [false; WORD_SIZE]);
    }

    #[test]
    fn counter_load_has_priority_over_increment() {
        let mut counter = Counter::new();
        let input = parse_bits("0000_0000_0000_0101").expect("valid test bit string");

        counter.tick(input, true, true, false);
        assert_eq!(counter.output(), input);
    }

    #[test]
    fn counter_increment_wraps_at_sixteen_bits() {
        let mut counter = Counter::new();
        let maximum = parse_bits("1111_1111_1111_1111").expect("valid test bit string");

        counter.tick(maximum, true, false, false);
        counter.tick([false; WORD_SIZE], false, true, false);
        assert_eq!(counter.output(), [false; WORD_SIZE]);
    }

    #[test]
    fn clock_counts_ticks() {
        let mut clock = Clock::new();
        assert_eq!(clock.cycles(), 0);
        clock.tick();
        clock.tick();
        assert_eq!(clock.cycles(), 2);
    }

    // Converts an integer into address bits ordered least-significant first.
    fn test_address<const ADDRESS_BITS: usize>(index: usize) -> [Bit; ADDRESS_BITS] {
        let mut remaining = index;
        std::array::from_fn(|_| {
            let bit = remaining % 2 != 0;
            remaining /= 2;
            bit
        })
    }
}
