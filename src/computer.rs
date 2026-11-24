//! # Chapter 5 - Computer
//!
//! In contrast to Turing Machines, which are abstract mathematical models of computation,
//! the von Neumann machine is a practical architecture and the conceptual blueprint of almost
//! all computer platform today. It is based on a **central processing unit** (CPU), interacting with
//! a **memory device**, receiving data from some **input** device, and sending data to some **output**
//! device. At the heart of this architecture lies the stored program concept: The computer's memory
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
//! ### Control Unit/Logic
//!
//! The **Control Unit** docodes instructions and figures out which instruction to fetch and exeute next.
//!
//! - Instruction Decoding
//!   - this is achieve by A-instruction and C-instruction specification designed in machine language
//! - Instruction Execution
//!   - this is achieved by C-instruction definition, i.e., `comp` bits: `acccccc`, `dest` bits: `ddd`, and jump bits `jjj`
//! - Next Instruction Fetching
//!   - this is achieved by emiting CPU's `pc` output to PC chip and CPU's `pc` output is calculated(by ALU output and j bits)
//!     so that - if jump(t) then PC(t) = A(t-1) else PC(t-1) + 1
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
//!
//! The Hack platform is a 16-bit von Neumann machine, consisting of a CPU, two separate memory modules serving as instruction
//! memory(ROM) and data memory(RAM), and two memory-mapped I/O devices: a screen and a keyboard.
//!
//! ### CPU
//! Hack's CPU consists of following components:
//! - ALU
//! - **data register**(D), 16-bit register
//! - **address register**(A), 16-bit register
//! - **progrma counter**(PC), 16-bit counter
//!   - PC is connected to the address input of the ROM. So ROM chip output always is `ROM[PC]`, i.e., the content of instruction
//!     memory location whose address is pointed at by PC. This content is called **current instruction**.
//!
//! ### Machine Language
//!
//! See details in previous chapter.
//!
//! ### Screen
//!
//! The Hack computer interacts with a black-white screen organized as 256 rows of 512 pixels per row. This interaction with physical
//! screen is via a memory map which is implemented by a chip **Screen**, i.e., any bits written to the memory
//! map is reflected as a pixel on the physical screen, i.e., 1=black, 0=white.
//!
//! The interaction with physical screen is to be implenented yet, i.e., via `minifb` library.
//!
//! ### Keyboard
//!
//! Similarly, Hack computer interacts with a standard keyboard via memory map, implemented by a chip **keyboard**.
//!
//! ### Overall Memory
//!
//! The overall address space of Hack, i.e., its entire data memory, is provided by a chip **Memory**, which includes
//! the RAM(16k for regular data storage) and screen(8k) and keyboard memory maps.
//!
//! - Access to any address >24576(0x6000) is invalid
//! - Access to any address in the range of 16384 to 24575(0x4000 - 0x5FFF) is accessing the screen memory map.
//! - Access to address =24576(0x6000) is accessing keyboard memory map.
//!
//! ### Computer
//! The topmost chip is Computer that operates all the devices including CPU, data memory, instruction memory(ROM),
//! a screen and a keyboard. To execute a program, the program's code must be preloaded into the ROM.
//!
//! - When `reset` is 0, the program stored in the ROM executes.
//! - When `reset` is 1, execution restarts. Thus to start a program's execute, `reset` must be set to 1 and then 0.
//!
//!
//!

use crate::gate::{Bit, Bits, WORD_SIZE, alu};
use crate::sequential::{Address14, Counter, RAM16k, Register16};
use minifb::{Window, WindowOptions};

pub use crate::sequential::{Address15, ROM32K, ROM32k, Rom32k};

/// A 13-bit address selects one of 8,192 16-bit screen words, representing 131072(512*256) pixels.
pub type Address13 = [Bit; 13];

/// The Hack screen is 512 pixels wide and 256 pixels high.
pub const SCREEN_WIDTH: usize = 512;
pub const SCREEN_HEIGHT: usize = 256;
pub const SCREEN_WORDS: usize = SCREEN_WIDTH * SCREEN_HEIGHT / WORD_SIZE;
pub const RAM_WORDS: usize = 16_384;
pub const SCREEN_BASE_ADDRESS: usize = RAM_WORDS;
pub const KEYBOARD_ADDRESS: usize = SCREEN_BASE_ADDRESS + SCREEN_WORDS;

/// Memory-mapped screen.
///
/// Each 16-bit word represents 16 consecutive pixels. Words are arranged from
/// the top-left of the screen, with 32 words in each row.
// TODO: Add a minifb renderer once Screen is connected to the computer memory map.
pub struct Screen {
    words: Vec<Bits>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            words: vec![[false; WORD_SIZE]; SCREEN_WORDS],
        }
    }

    pub fn output(&self, address: Address13) -> Bits {
        self.words[address_index(address)]
    }

    pub fn read(&self, address: Address13) -> Bits {
        self.output(address)
    }

    pub fn tick(&mut self, input: Bits, address: Address13, load: Bit) {
        if load {
            self.words[address_index(address)] = input;
        }
    }

    pub fn clear(&mut self) {
        self.words.fill([false; WORD_SIZE]);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, black: Bit) {
        assert!(x < SCREEN_WIDTH, "screen x-coordinate is out of range");
        assert!(y < SCREEN_HEIGHT, "screen y-coordinate is out of range");

        let word_index = y * (SCREEN_WIDTH / WORD_SIZE) + x / WORD_SIZE;
        let pixel_index = x % WORD_SIZE;

        let mut word = self.words[word_index];
        word[pixel_index] = black;
        self.words[word_index] = word;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Bit {
        assert!(x < SCREEN_WIDTH, "screen x-coordinate is out of range");
        assert!(y < SCREEN_HEIGHT, "screen y-coordinate is out of range");

        let word_index = y * (SCREEN_WIDTH / WORD_SIZE) + x / WORD_SIZE;
        let pixel_index = x % WORD_SIZE;
        self.words[word_index][pixel_index]
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

/// Displays the Hack screen in a host window.
pub struct ScreenWindow {
    window: Window,
    pixels: Vec<u32>,
}

impl ScreenWindow {
    pub fn new() -> Result<Self, minifb::Error> {
        let window = Window::new(
            "Hack Screen",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions::default(),
        )?;

        Ok(Self {
            window,
            pixels: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
        })
    }

    // Copies the virtual Hack screen into the window’s pixel buffer, then displays it
    pub fn update(&mut self, screen: &Screen) -> Result<(), minifb::Error> {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.pixels[y * SCREEN_WIDTH + x] = if screen.get_pixel(x, y) {
                    // if Hack screen's pixel is 1, set window's pixel to black
                    // 0x000000 is hexadecimal RGB color notation of black
                    0x000000
                } else {
                    // else white
                    0x00ff_ffff
                };
            }
        }

        self.window
            .update_with_buffer(&self.pixels, SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

/// Memory-mapped keyboard input.
///
/// The output is the Hack key code for the currently pressed key, or zero when
/// no key is pressed.
// TODO: Connect host keyboard events to Hack key codes when the computer is integrated.
pub struct Keyboard {
    key_code: Bits,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key_code: [false; WORD_SIZE],
        }
    }

    pub fn output(&self) -> Bits {
        self.key_code
    }

    pub fn read(&self) -> Bits {
        self.output()
    }

    pub fn press_key(&mut self, key_code: Bits) {
        self.key_code = key_code;
    }

    pub fn release_key(&mut self) {
        self.key_code = [false; WORD_SIZE];
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

/// Hack's overall data memory, including RAM, screen memory, and keyboard input.
pub struct Memory {
    ram: RAM16k,
    screen: Screen,
    keyboard: Keyboard,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: RAM16k::new(),
            screen: Screen::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn output(&self, address: Address15) -> Bits {
        self.read(address)
    }

    pub fn read(&self, address: Address15) -> Bits {
        Self::assert_valid_address(address);

        if !address[14] {
            self.ram.read(Self::ram_address(address))
        } else if !address[13] {
            self.screen.read(Self::screen_address(address))
        } else {
            self.keyboard.read()
        }
    }

    pub fn tick(&mut self, input: Bits, address: Address15, load: Bit) {
        Self::assert_valid_address(address);

        if !address[14] {
            self.ram.tick(input, Self::ram_address(address), load);
        } else if !address[13] {
            self.screen.tick(input, Self::screen_address(address), load);
        }
    }

    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn keyboard_mut(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }

    fn assert_valid_address(address: Address15) {
        let mut index = 0;
        let mut place = 1;

        for bit in address.iter() {
            index += usize::from(*bit) * place;
            place *= 2;
        }

        assert!(
            index <= KEYBOARD_ADDRESS,
            "Hack memory address is out of range"
        );
    }

    fn ram_address(address: Address15) -> Address14 {
        std::array::from_fn(|index| address[index])
    }

    fn screen_address(address: Address15) -> Address13 {
        std::array::from_fn(|index| address[index])
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

fn address_index(address: Address13) -> usize {
    let mut index = 0;
    let mut place = 1;

    for bit in address.iter() {
        index += usize::from(*bit) * place;
        place *= 2;
    }

    index
}

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
        let a_before_tick = self.a_register.read();

        // address instruction, in binary number format, starts with 0, i.e. Bits[15]
        // compute instruction, in binary number format, starts with 1, i.e. Bits[15]
        let is_compute_instruction = instruction[15];

        // Read the first d-bit of c-insturction to determine if store ALU output to A-register
        let load_a = if is_compute_instruction {
            // compute instruction has format "111acccccccdddjjj"
            // instruction[5] — destination is A-register
            // instruction[4] — destination is D-register
            // instruction[3] — destination is M, where M = Memory[A]
            instruction[5]
        } else {
            // For an address instruction, is_compute_instruction is false, so load_a is always true
            true
        };

        // if instruction is a c-instruction, causes ALU to calculate and get the computed value
        let (alu_output, zr, ng) = if is_compute_instruction {
            alu(
                // ALU's first input is alway D-register
                self.d_register.read(),
                // C-instruction has format of `111_acccccc_ddd_jjj`(ignore the _)
                // if the `a` bit is 1, second input is M, i.e., Memory[A]
                if instruction[12] {
                    in_m
                } else {
                    // if the `a` bit is 0, second input is A-register
                    a_before_tick
                },
                // the c-bts are other inputs to ALU
                instruction[11],
                instruction[10],
                instruction[9],
                instruction[8],
                instruction[7],
                instruction[6],
            )
        } else {
            // if A-instruction, assign an arbitrary to `alu_output` since it will not be used
            ([false; WORD_SIZE], false, false)
        };

        // the previous `let (alu_output...) = if is_compute_instruction {...` statement ensures that
        // if `instruction` is C-instruction, the computed value is to be stored
        // else A-instruction, the A-instruction iteself is to be stored
        let a_input = if is_compute_instruction {
            alu_output
        } else {
            // If instruction is an address instruction, the value to be store in A-register is in the instruction itself.
            instruction
        };

        // `load_a` is 1 when below two situation:
        // when the insturction is A-instruction, i.e., setting A-register to `0vvv_vvvv_vvvv_vvvv`, `a_input` is exactly the instruction itself
        // when the instruction is c-instruction and first d-bit is 1, `a_input` is ALU output
        self.a_register.tick(a_input, load_a);

        // Determine if it is c-instruction and destination is D-register, i.e, 2nd d-bit is 1
        let load_d = is_compute_instruction && instruction[4];
        // If so, store ALU output to D-register
        self.d_register.tick(alu_output, load_d);

        // Determine if it is c-instruction and destnation is M, i.e., Memory[A], i.e., 3rd d-bit is 1
        let load_m = is_compute_instruction && instruction[3];

        // Determine the `jump` field:
        // j-bit(s) are set 1 and
        // ALU output(ng, zr) meet the condition indicated by j-bit(s)

        // use `||` to include multiple situations, i.e., <=, >=, <>
        let jump = is_compute_instruction
            && ((instruction[2] && ng) || (instruction[1] && zr) || (instruction[0] && !ng && !zr));

        // A little bit
        // Only when `jump` condition is met and `reset` is 0, load A-register content of previous cycle to PC.
        // PC's output is pc part of CPU's output
        // Next Instruction Fetching:
        // when no jump and no reset, PC increases by 1, i.e., PC(t) = PC(t-1) + 1
        // when to effect a `goto n` command, machine language specifiction requires that first set A-register to `n` by A-instruction
        // Then issue a jump directive, coded by the j-bits of a subsequent C-instruction. Loading `a_before_tick`
        // to PC achieve the jump command, i.e., If jump(t) then PC(t) = A(t-1)
        self.pc
            .tick(a_before_tick, jump && !reset, !jump && !reset, reset);

        CpuOutput {
            out_m: alu_output,
            write_m: load_m,
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

/// The Hack computer, combining the CPU, instruction memory, and overall data memory.
pub struct Computer {
    cpu: CPU,
    rom: ROM32k,
    memory: Memory,
}

impl Computer {
    /// Creates a computer with the supplied machine-language program loaded into ROM.
    pub fn new(program: &[Bits]) -> Self {
        Self {
            cpu: CPU::new(),
            rom: ROM32k::from_words(program),
            memory: Memory::new(),
        }
    }

    /// Executes one computer cycle.
    pub fn tick(&mut self, reset: Bit) -> CpuOutput {
        let current_pc = self.cpu.read_pc();
        let a_before_tick = self.cpu.read_a();
        let instruction = self.rom.read(low_15_bits(current_pc));
        let in_m = if instruction[15] && instruction[12] {
            self.memory.read(low_15_bits(a_before_tick))
        } else {
            [false; WORD_SIZE]
        };

        let output = self.cpu.tick(in_m, instruction, reset);

        if output.write_m {
            // A-register may also be a destination, so write to its value before this CPU tick.
            self.memory
                .tick(output.out_m, low_15_bits(a_before_tick), true);
        }

        output
    }

    pub fn cpu(&self) -> &CPU {
        &self.cpu
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }
}

fn low_15_bits(word: Bits) -> Address15 {
    std::array::from_fn(|index| word[index])
}

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
    fn cpu_jump_uses_a_register_value_from_before_the_tick() {
        let mut cpu = CPU::new();
        let ten = word("0000_0000_0000_1010");
        let five = word("0000_0000_0000_0101");

        cpu.tick([false; WORD_SIZE], ten, false);
        cpu.tick([false; WORD_SIZE], word("1110_1100_0001_0000"), false);
        cpu.tick([false; WORD_SIZE], five, false);

        let output = cpu.tick([false; WORD_SIZE], word("1110_0011_0010_0111"), false);

        assert_eq!(cpu.read_a(), ten);
        assert_eq!(output.pc, five);
    }

    #[test]
    fn cpu_reset_has_priority_over_jump_and_increment() {
        let mut cpu = CPU::new();
        cpu.tick([false; WORD_SIZE], word("0000_0000_0000_0101"), false);

        let instruction = word("1110_1111_1001_1111");
        let output = cpu.tick([false; WORD_SIZE], instruction, true);

        assert_eq!(output.pc, [false; WORD_SIZE]);
    }

    #[test]
    fn screen_starts_with_all_pixels_white() {
        let screen = Screen::new();
        let address = test_address(8191);

        assert_eq!(screen.read(address), [false; WORD_SIZE]);
        assert!(!screen.get_pixel(SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1));
    }

    #[test]
    fn screen_loads_and_reads_the_selected_word() {
        let mut screen = Screen::new();
        let stored = word("1010_1010_1010_1010");
        let address = test_address(37);

        screen.tick(stored, address, true);

        assert_eq!(screen.read(address), stored);
        assert_eq!(screen.read(test_address(36)), [false; WORD_SIZE]);
    }

    #[test]
    fn screen_maps_pixels_to_screen_words() {
        let mut screen = Screen::new();

        screen.set_pixel(17, 3, true);

        assert!(screen.get_pixel(17, 3));
        assert_eq!(screen.read(test_address(3 * 32 + 1))[17 % WORD_SIZE], true);
    }

    #[test]
    fn keyboard_reports_pressed_key_and_returns_zero_after_release() {
        let mut keyboard = Keyboard::new();
        let key_code = word("0000_0000_0100_0001");

        assert_eq!(keyboard.read(), [false; WORD_SIZE]);

        keyboard.press_key(key_code);
        assert_eq!(keyboard.read(), key_code);

        keyboard.release_key();
        assert_eq!(keyboard.read(), [false; WORD_SIZE]);
    }

    #[test]
    fn memory_routes_addresses_to_ram_and_screen() {
        let mut memory = Memory::new();
        let values = [
            (0, word("0000_0000_0000_0001")),
            (511, word("0000_0000_0000_0010")),
            (512, word("0000_0000_0000_0011")),
            (4095, word("0000_0000_0000_0100")),
            (4096, word("0000_0000_0000_0101")),
            (RAM_WORDS - 1, word("0000_0000_0000_0110")),
            (SCREEN_BASE_ADDRESS, word("0000_0000_0000_0111")),
            (
                SCREEN_BASE_ADDRESS + SCREEN_WORDS - 1,
                word("0000_0000_0000_1000"),
            ),
        ];

        for (address, value) in values {
            memory.tick(value, test_memory_address(address), true);
        }

        for (address, value) in values {
            assert_eq!(memory.read(test_memory_address(address)), value);
        }
    }

    #[test]
    fn memory_reads_keyboard_and_ignores_writes_to_it() {
        let mut memory = Memory::new();
        let key_code = word("0000_0000_0100_0001");

        memory.keyboard_mut().press_key(key_code);
        assert_eq!(memory.read(test_memory_address(KEYBOARD_ADDRESS)), key_code);

        memory.tick(
            [true; WORD_SIZE],
            test_memory_address(KEYBOARD_ADDRESS),
            true,
        );
        assert_eq!(memory.read(test_memory_address(KEYBOARD_ADDRESS)), key_code);
    }

    #[test]
    #[should_panic(expected = "Hack memory address is out of range")]
    fn memory_rejects_addresses_after_keyboard() {
        let memory = Memory::new();

        memory.read(test_memory_address(KEYBOARD_ADDRESS + 1));
    }

    #[test]
    fn computer_executes_program_and_writes_memory() {
        let program = [
            word("0000_0000_0000_0101"),
            word("1110_1100_0001_0000"),
            word("0000_0000_0000_1010"),
            word("1110_0011_0010_1000"),
        ];
        let mut computer = Computer::new(&program);

        for _ in 0..program.len() {
            computer.tick(false);
        }
        println!(
            "RAM[10]: {:?}",
            computer.memory().read(test_memory_address(10))
        );

        println!("A: {:?}", computer.cpu().read_a());
        println!("D: {:?}", computer.cpu().read_d());
        println!("PC: {:?}", computer.cpu().read_pc());

        assert_eq!(
            computer.memory().read(test_memory_address(10)),
            word("0000_0000_0000_0101")
        );
        assert_eq!(computer.cpu().read_a(), word("0000_0000_0000_0101"));
    }

    fn test_address(index: usize) -> Address13 {
        let mut remaining = index;
        std::array::from_fn(|_| {
            let bit = remaining % 2 == 1;
            remaining /= 2;
            bit
        })
    }

    fn test_memory_address(index: usize) -> Address15 {
        let mut remaining = index;
        std::array::from_fn(|_| {
            let bit = remaining % 2 == 1;
            remaining /= 2;
            bit
        })
    }
}
