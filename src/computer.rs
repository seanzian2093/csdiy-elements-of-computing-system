//! # Chapter 5 - Computer
//!
//! In contrast to Turing Machines, which are abstract mathematical models of computation,
//! the von Neumann machine is a practical architecture and the conceptual blueprint of almost
//! all computer platform today. It is based on a **central processing unit** (CPU), interacting with
//! a **memory device**, receiving data from some **input** device, and sending data to some **output**
//! device. Ath the heart of this architecture lies the stored program concept: The computer's memory
//! stores not only the data that the computer manipulates, but also the very instructions that tell
//! the computer what to do.
//!
//! ## Memory
//! The memory of a von Neumann machine holds two types of information: the data items and programming
//! instructions. Both are represented as binary numbers, and both are stored in same generic random-
//! access structure: a continuous array of cells of some fixed width, also called **words** or **location**,
//! each having a unique **adress**. Hence, an individual word, representing either data item or instruction, is
//! specified by supplying its address.
//!
//! ### Data Memory
//! Once an individual word of data memory is selected by specifying its address, it can be **read** or **written**
//! to. In former case, we retrieve the word's value. In the latter case, we store a new value into the selected
//! location, erasing the old value.
//!
//! ### Instruction Memory
//! High-level command are translated into a series of binary **word**, representing the machine language instructions.
//! These instructions are stored in the **instruction memory**. In each step of the computer's operation, the CPU
//! **fetches**(i.e., reads) a word from the instruction memory, decodes it, executes the decoded instruction, and
//! figures out which instruction to execute next.
//!
//! ## CPU
//! The CPU executes tasks using three main hardware elements: an **Arithmetic Logic Unit** (ALU), a **Control Unit**,
//! and a set of **registers**.
//!
//! ### ALU
//! The ALU performs low-level arithmetic and logical operations, featured by the computer.
//!
//! ### Registers
//! THe CPU is designed to carry out simple calculations **quickly**. So it is locally equipped with a small set of high-speed
//! **registers**, each capable of holding a single word.
//! - **Data registers** are used to store temporary result.
//! - **Adress registers** are used to store an memory address for CPU's later use.
//! - **Program counter register** keeps the address of next instruction to be fetched from the instruction memory.
//!
//! ### Control Unit
//! The **Control Unit** docodes instructions and figures out which instruction to fetch and exeute next.
//!
//! ## Input and Output
//! All devices like screnns, keyboards, printers, etc can be abstracted as **memoery-mapped I/O** by allocating each device
//! an exlcusive area in memory, becoming its **memory map**.
//!
//! ### Input devices
//! The memory map is made to continuously **reflect** the physical state of the device.
//!
//! ### Output devices
//! The memory map is made to continuously **drive** the physical state of the device.
//!
//! ## Hack Specification
//! The Hack platform is a 16-bit von Neumann machine, consisting of a CPU, two separate memory modules serving as instruction
//! memory(ROM) and data memory(RAM), and two memory-mapped I/O devices: a screen and a keyboard.
//!
//! ### CPU
//! Hack's CPU consists of following components:
//! - ALU
//! - **data register**(D), 32k 16-bit register
//! - **address register**(A), 32k 16-bit register
//! - **progrma counter**(PC), 16-bit counter
//!   - PC is connected to the address input of the ROM. So ROM chip output always is `ROM[PC]`, i.e., the content of instruction
//! memory location whose address is pointed at by PC. This content is called **current instruction**.
//!
//! ### Machine Language
//!
//! - **address instruction** has format **0vvv_vvvv_vvvv_vvvv** (does not include **_**, here is just for readability), v being
//! 0 or 1. It causes computer to load the 15-bit constant into the A-register.
//! - **compute instruction** has format **111_a_ccccccc_ddd_jjj**.
//!   - The **a-** and **c-** bits instruct the ALU which function to compute.
//!   - The **d-** bits instruct where to store the ALU output.
//!   - The **j-** bits specify an optional jump condition.

use crate::gate::{Bit, Bits, WORD_SIZE, alu};
use crate::sequential::{Counter, Register16};

/// The output pins produced by the CPU after a clock tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpuOutput {
    pub out_m: Bits,
    pub write_m: Bit,
    pub address_m: Bits,
    pub pc: Bits,
}

impl CpuOutput {
    pub fn out_m(&self) -> Bits {
        self.out_m
    }

    pub fn write_m(&self) -> Bit {
        self.write_m
    }

    pub fn address_m(&self) -> Bits {
        self.address_m
    }

    pub fn pc(&self) -> Bits {
        self.pc
    }
}

/// The Hack central processing unit.
///
/// The CPU fetches its current instruction through `instruction`, reads data from
/// `in_m`, and produces the value to write, the destination address, and the
/// address of the next instruction.
pub struct CPU {
    a_register: Register16,
    d_register: Register16,
    pc: Counter,
}

impl CPU {
    pub fn new() -> Self {
        let zero = [false; WORD_SIZE];
        Self {
            a_register: Register16::new(zero),
            d_register: Register16::new(zero),
            pc: Counter::new(),
        }
    }

    pub fn read_a(&self) -> Bits {
        self.a_register.read()
    }

    pub fn read_d(&self) -> Bits {
        self.d_register.read()
    }

    pub fn read_pc(&self) -> Bits {
        self.pc.read()
    }

    /// Executes one instruction and advances the program counter.
    pub fn tick(&mut self, in_m: Bits, instruction: Bits, reset: Bit) -> CpuOutput {
        // address instruction, in binary number format, starts with 0, i.e. Bits[15]
        // compute instruction, in binary number format, starts with 1, i.e. Bits[15]
        let is_compute_instruction = instruction[15];

        let load_a = if is_compute_instruction {
            // compute instruction has format "111acccccccdddjjj"
            // instruction[5] — destination A
            // instruction[4] — destination D
            // instruction[3] — destination M, M = Memory[A]
            instruction[5]
        } else {
            // For an address instruction, is_compute_instruction is false, so load_a is always true
            true
        };

        let a_input = if is_compute_instruction {
            let (out, _, _) = alu(
                self.d_register.read(),
                if instruction[12] {
                    in_m
                } else {
                    self.a_register.read()
                },
                instruction[11],
                instruction[10],
                instruction[9],
                instruction[8],
                instruction[7],
                instruction[6],
            );
            out
        } else {
            instruction
        };

        let (out_m, zr, ng) = if is_compute_instruction {
            alu(
                self.d_register.read(),
                if instruction[12] {
                    in_m
                } else {
                    self.a_register.read()
                },
                instruction[11],
                instruction[10],
                instruction[9],
                instruction[8],
                instruction[7],
                instruction[6],
            )
        } else {
            (self.a_register.read(), false, false)
        };

        self.a_register.tick(a_input, load_a);

        let write_d = is_compute_instruction && instruction[4];
        self.d_register.tick(out_m, write_d);

        let write_m = is_compute_instruction && instruction[3];

        let jump = is_compute_instruction
            && ((instruction[2] && ng) || (instruction[1] && zr) || (instruction[0] && !ng && !zr));

        let pc_input = self.a_register.read();
        self.pc
            .tick(pc_input, jump && !reset, !jump && !reset, reset);

        CpuOutput {
            out_m,
            write_m,
            address_m: self.a_register.read(),
            pc: self.pc.read(),
        }
    }

    pub fn step(&mut self, in_m: Bits, instruction: Bits, reset: Bit) -> CpuOutput {
        self.tick(in_m, instruction, reset)
    }

    pub fn a_output(&self) -> Bits {
        self.a_register.read()
    }

    pub fn d_output(&self) -> Bits {
        self.d_register.read()
    }

    pub fn pc_output(&self) -> Bits {
        self.pc.read()
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

/// Lowercase alias for conventional Rust naming.
pub type Cpu = CPU;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::parse_bits;

    fn word(value: &str) -> Bits {
        parse_bits(value).expect("valid test bit string")
    }

    #[test]
    fn cpu_loads_an_address_instruction() {
        let mut cpu = CPU::new();
        let instruction = word("0000_0000_0000_0101");

        let output = cpu.tick([false; WORD_SIZE], instruction, false);

        assert_eq!(cpu.read_a(), instruction);
        assert_eq!(output.address_m, instruction);
        assert_eq!(cpu.read_pc(), word("0000_0000_0000_0001"));
    }

    #[test]
    fn cpu_writes_d_and_memory_from_a_compute_instruction() {
        let mut cpu = CPU::new();
        cpu.tick([false; WORD_SIZE], word("0000_0000_0000_0011"), false);

        let instruction = word("1110_1100_0011_1000");
        let output = cpu.tick(word("0000_0000_0000_0101"), instruction, false);

        assert_eq!(cpu.read_a(), word("0000_0000_0000_0011"));
        assert_eq!(cpu.read_d(), word("0000_0000_0000_0011"));
        assert!(output.write_m);
    }

    #[test]
    fn cpu_jumps_when_the_condition_is_true() {
        let mut cpu = CPU::new();
        cpu.tick([false; WORD_SIZE], word("0000_0000_0000_0101"), false);

        let instruction = word("1110_1100_0000_0001");
        let output = cpu.tick([false; WORD_SIZE], instruction, false);

        assert_eq!(output.pc, word("0000_0000_0000_0101"));
    }

    #[test]
    fn cpu_reset_has_priority_over_jump_and_increment() {
        let mut cpu = CPU::new();
        cpu.tick([false; WORD_SIZE], word("0000_0000_0000_0101"), false);

        let instruction = word("1110_1111_1001_1111");
        let output = cpu.tick([false; WORD_SIZE], instruction, true);

        assert_eq!(output.pc, [false; WORD_SIZE]);
    }
}
