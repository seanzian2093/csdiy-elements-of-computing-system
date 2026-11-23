//! # Chapter 5 - Assembler
//!
//! ## Assembler
//!
//! Assembler is essentially a text-processing program, designed to provide translation services.
//!
//! ### Machine Language Specification
//! A **machine language specification**, i.e., a contract between
//! - full documentation of assembly syntax
//! - the respective binary codes.
//!
//! ### Assemlber's Tasks
//! Assembler is a program that follows above contract and perform following task(not necessarily in that order)
//! - parse the symbolic command into its underlying fields
//! - for each field, generate the corresponding bits in the machine language
//! - replace all symbolic references(if any) with numeric address of memory location(most chanllenging)
//! - assembly the binary codes into a complete machine instruction
//!
//! ### Symbol
//!
//! In general, symbols are introduced into assembly programs from two sources: **variables** and **labels**
//!
//! - The programmer can use symbolic variable names, and the ranslator will automatically assign them to memory
//!   addresses.
//! - The programmer can mark various locations in the program with symbols, i.e. `loop`. Other commands in the programm can
//!   then `goto loop` unconditionally, or conditionally.
//!
//! ## Hack Assembly-to-Binary Translation Specification
//!
//! ### Syntax Conventions and File Formats
//!
//! - assembly code is stored in a textfile with `.asm` extension
//!   - each text line representing an **instruction** or a **symbol declaration**
//!   - **instruction**: A-instruction or C-instruction
//!   - **(Symbol)**: this pseudo-command binds the **Symbol** to memory location into which the next command in the programm
//!     will be stored.
//!
//! - binary code is stored in a textfile with `.hack` extension
//!   - each line is a sequence of 16 `0` and `1`
//!   - when loaded to computer's instruction memory, the binary code represented by the file's `nth` line is stored in
//!     address `n` of the instruction memory(the count of both program lines and memory addresses start at 0)
//!
//! - constants and symbols
//!   - **constants** must be non-negative and written in decimal notation.
//!   - **symbol** can be any sequence of letters, digits, underscore(_), dot(.), dollar sign($) and colon(:) that does
//!     not start with a digit.
//!
//! - comments
//!   - text that begins with two slashes(//) and ends at the end of the line is considered a comment and is ignored.
//!
//! - white space
//!   - space characters and empty lines are ignored.
//!
//! - case conventions
//!   - all the assembly mnemonics must be written in uppercase.
//!   - the rest(user-defined labels and variable names) is case sensitive.
//!   - convention is to use uppercase for labels and lowercase for variable names.
//!
//! ### Instructions
//!
//! - A-instruction
//!   Refer to machine language
//!
//! - C-instruction
//!   Refer to machine language
//!
//! - Symbols
//!
//!   - predefined symbols
//!
//! | Label| RAM Address | Hexa |
//! |:-----|------------:|-----:|
//! | R0   | 0           | 0x0000 |
//! | R1   | 1           | 0x0001 |
//! | R2   | 2           | 0x0002 |
//! | R3   | 3           | 0x0003 |
//! | R4   | 4           | 0x0004 |
//! | R5   | 5           | 0x0005 |
//! | R6   | 6           | 0x0006 |
//! | R7   | 7           | 0x0007 |
//! | R8   | 8           | 0x0008 |
//! | R9   | 9           | 0x0009 |
//! | R10  | 10          | 0x000A |
//! | R11  | 11          | 0x000B |
//! | R12  | 12          | 0x000C |
//! | R13  | 13          | 0x000D |
//! | R14  | 14          | 0x000E |
//! | R15  | 15          | 0x000F |
//! | SP   | 0           | 0x0000 |
//! | LCL  | 1           | 0x0001 |
//! | ARG  | 2           | 0x0002 |
//! | THIS | 3           | 0x0003 |
//! | THAT | 4           | 0x0004 |
//! | SCREEN | 16384     | 0x4000 |
//! | KBD  | 24576       | 0x6000 |
//!
//!   - label symbol: defined once, used anywhere in the program, even before definition line.
//!
//!   - variable symbol: any symbol that is treated as a variable if it is
//!     - not predefined or
//!     - not defined else where using `(Xxx)` command.
//!     - variables are mapped to consecutive memory locations as they are first encountered, starting at RAM address 16 (0x0010)
//!
//! ## Implementaion
//!
//! Hack assemlber implementation is proposed based on 4 modules: **Parser**, **Code**, **SymbolTable**, and **main program**.
//!
//! ### Parser Module API
//!
//! <table>
//! <thead>
//! <tr>
//!   <th>Routine</th>
//!   <th>Arguments</th>
//!   <th>Returns</th>
//!   <th>Function</th>
//! </tr>
//! </thead>
//! <tbody>
//! <tr>
//!   <td>Constructor/initializer</td>
//!   <td>Input file/ stream</td>
//!   <td>NA</td>
//!   <td>
//!     Opens the input file/stream and gets ready to parse it
//!   </td>
//! </tr>
//! <tr>
//!   <td>hasMoreCommands</td>
//!   <td>NA</td>
//!   <td>Boolean</td>
//!   <td>Are there more commands in the input?</td>
//! </tr>
//! <tr>
//!   <td>advance</td>
//!   <td>NA</td>
//!   <td>NA</td>
//!   <td>
//!     Reads the next command from the input and makes it the current command. Should be called
//!     only when hasMoreCommands() is true. Initially there is no current command.
//!   </td>
//! </tr>
//! <tr>
//!   <td>commandType</td>
//!   <td>NA</td>
//!   <td>
//!     A_COMMAND,
//!     C_COMMAND,
//!     L_COMMAND,
//!   </td>
//!   <td>
//!     Returns the type of the current command:
//!     <ul>
//!       <li><strong>A_COMMAND</strong> for <code>@Xxx</code> where <strong>Xxx</strong> is either a symbol or a decimal number</li>
//!       <li><strong>C_COMMAND</strong> for <code>dest=comp;jump</code></li>
//!       <li><strong>L_COMMAND</strong> for <code>Xxx</code> where <strong>Xxx</strong> is a symbol</li>
//!     </ul>
//!   </td>
//! </tr>
//! <tr>
//!   <td>symbol</td>
//!   <td>NA</td>
//!   <td>string</td>
//!   <td>
//!     Returns the symbol or decimal <strong>Xxx</strong> of the current command <code>@Xxx</code> or <code>(Xxx)</code>. Should
//!     be called only when <code>commandType</code> is <strong>A_COMMAND</strong> or <strong>L_COMMAND</strong>.
//!   </td>
//! </tr>
//! <tr>
//!   <td>dest</td>
//!   <td>NA</td>
//!   <td>string</td>
//!   <td>
//!     Returns the <code>dest</code> mnemonic in the current C-command(8 possibilities). SHould be called only when <code>commandType</code>
//!     is <strong>C_COMMAND</strong>.
//!   </td>
//! </tr>
//! <tr>
//!   <td>comp</td>
//!   <td>NA</td>
//!   <td>string</td>
//!   <td>
//!     Returns the <code>comp</code> mnemonic in the current C-command(28 possibilities). SHould be called only when <code>commandType</code>
//!     is <strong>C_COMMAND</strong>.
//!   </td>
//! </tr>
//! <tr>
//!   <td>jump</td>
//!   <td>NA</td>
//!   <td>string</td>
//!   <td>
//!     Returns the <code>jump</code> mnemonic in the current C-command(8 possibilities). SHould be called only when <code>commandType</code>
//!     is <strong>C_COMMAND</strong>.
//!   </td>
//! </tr>
//! </tbody>
//! </table>
//!
//!
//! ### Code Module API
//!
//! Translate Hack assembly language mnemonics into binary codes.
//!
//! | Routine | Arguments       | Returns | Function                                         |
//! |:-------:|:---------------:|:-------:|:------------------------------------------------:|
//! | dest    | mnemonic string | 3 bits  | Returns the binary code of the **dest** mnemonic |
//! | comp    | mnemonic string | 3 bits  | Returns the binary code of the **comp** mnemonic |
//! | jump    | mnemonic string | 3 bits  | Returns the binary code of the **jump** mnemonic |
//!
//! ### SymbolTable Module
//!
//! Keep a correspondece between symbolic labels and numeric addresses.
//!
//! | Routine      | Arguments                    | Returns | Function                                            |
//! |:------------:|:----------------------------:|:-------:|:---------------------------------------------------:|
//! | Constructor  | NA                           | NA      | Creates a new empty symbol table.                   |
//! | addEntry     | symbol(string), address(int) | NA      | Adds teh pair **(symbol, address)** to the table.   |
//! | contains     | symbol(string)               | Boolean | Does the symbol table contain the given **symbol**? |
//! | GetAddress   | symbol(string)               | int     | Returns the address associated with the **symbol**. |
//!
//! - Initialization: initialize the symbol table with all the predefined symbols and their pre-allocated RAM addresses.
//! - First pass: go through the entire assembly program, line by line, and build the symbol table without generating
//!   any code.
//! - Second pass: go through again the entire assembly program, line by line.
//!
//! ### Why a label is assigned the next instruction's address
//!
//! A label does not generate a machine instruction; it names the ROM address where the next
//! instruction will be stored. For example:
//!
//! ```text
//! @LOOP
//! 0;JMP
//! (LOOP)
//! D=M
//! ```
//!
//! The first two instructions occupy addresses 0 and 1. `(LOOP)` emits no instruction, so it
//! is assigned address 2, where `D=M` is stored. A jump to `LOOP` therefore starts at `D=M`.
//!
use std::collections::HashMap;
use std::fmt;

/// An assembly error associated with its source line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblerError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for AssemblerError {}

/// Maps Hack symbols to their numeric addresses.
#[derive(Debug, Clone)]
pub struct SymbolTable {
    entries: HashMap<String, u16>,
}

impl SymbolTable {
    /// Creates a symbol table initialized with Hack's predefined symbols.
    pub fn new() -> Self {
        let mut table = Self {
            entries: HashMap::new(),
        };

        // Add R1 to R15
        for register in 0..=15 {
            table.add_entry(&format!("R{register}"), register);
        }

        // Add Othe predefined symbols
        table.add_entry("SP", 0);
        table.add_entry("LCL", 1);
        table.add_entry("ARG", 2);
        table.add_entry("THIS", 3);
        table.add_entry("THAT", 4);
        table.add_entry("SCREEN", 16384);
        table.add_entry("KBD", 24576);
        table
    }

    /// Adds or replaces a symbol address.
    pub fn add_entry(&mut self, symbol: &str, address: u16) {
        self.entries.insert(symbol.to_owned(), address);
    }

    /// Returns whether the symbol table contains `symbol`.
    pub fn contains(&self, symbol: &str) -> bool {
        self.entries.contains_key(symbol)
    }

    /// Returns the address associated with `symbol`, if present.
    pub fn get_address(&self, symbol: &str) -> Option<u16> {
        self.entries.get(symbol).copied()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum Command {
    Address(String),
    Compute(String),
}

#[derive(Debug)]
struct LocatedCommand {
    line: usize,
    command: Command,
}

/// Translates Hack assembly source into newline-separated 16-bit machine code.
///
/// Blank lines and `//` comments are ignored. Labels are resolved in a first
/// pass, and previously unseen symbols are assigned RAM addresses starting at 16.
pub fn assemble(source: &str) -> Result<String, AssemblerError> {
    let mut commands = Vec::new();
    let mut symbols = SymbolTable::new();
    let mut instruction_address = 0u16;

    // First pass through
    for (line_index, source_line) in source.lines().enumerate() {
        let line_number = line_index + 1;

        // Removes the in-line comment from the line by splitting at //
        // `.next()` keeps only the first part before the first occurrence.
        // e.g., "D=M // set D" becomes "D=M".
        // `unwrap_or("")`` provides an empty-string fallback, though split always yields at least one part,
        // so that fallback is effectively unnecessary.
        let without_comment = source_line.split("//").next().unwrap_or("");

        // Remove ASCII whitespace, e.g., `D = M + 1`` becomes `D=M+1``, which is easier for the assembler to parse
        let line: String = without_comment
            .chars()
            .filter(|character| !character.is_ascii_whitespace())
            .collect();

        // Ignore empty line
        if line.is_empty() {
            continue;
        }

        // Label declaration statement,e.g., `(Xxx)`
        if line.starts_with('(') || line.ends_with(')') {
            let Some(label) = line
                .strip_prefix('(')
                .and_then(|line| line.strip_suffix(')'))
            else {
                return Err(error(line_number, "malformed label declaration"));
            };

            // Check if valid ASCII characters are used, first must not be digit
            if !is_valid_symbol(label) {
                return Err(error(line_number, "invalid label symbol"));
            }

            // For each iteration of loop, if it is a label declaration statement
            // check instruction_address is with in 32767, largest 15-bit address.

            if instruction_address > 32767 {
                return Err(error(
                    line_number,
                    "label address exceeds the 15-bit address space",
                ));
            }

            // If re-declarating a symbol, throw an error
            if symbols.contains(label) {
                return Err(error(
                    line_number,
                    &format!("symbol '{label}' is already defined"),
                ));
            }

            // Add pair of label and address to the symbol table
            symbols.add_entry(label, instruction_address);

            // After recording a new label, skip the rest of current iteration, otherwise the label text would
            // fall through to the instruction-parsing code and be treated as an instruction, incorrectly increasing
            continue;
        }

        // If current line is an A-instruction, i.e., `@xxxx`
        let command = if let Some(address) = line.strip_prefix('@') {
            // there must be a decimal after @
            if address.is_empty() {
                return Err(error(line_number, "missing A-instruction value"));
            }
            // Only use 15-bits, i.e., `@` is discarded once used
            Command::Address(address.to_owned())
        // if current line is C-instruction
        } else {
            Command::Compute(line)
        };

        // A program can exceed address 32767 without defining a label there. Labels are checked in their own branch
        // instructions also need to fit in the 15-bit address space, so this check is to reject an out-of-range instruction
        if instruction_address > 32767 {
            return Err(error(
                line_number,
                "program exceeds the 15-bit instruction address space",
            ));
        }

        // Add entry to located commands if the current line is not label statement
        commands.push(LocatedCommand {
            line: line_number,
            command,
        });

        // Increase instruction address only if it is not a label statement
        instruction_address += 1;
    }

    let mut output = String::new();
    // 15-bit instruction address space start with 16, i.e., predefine symbols R0-R5 use 0 to 15
    let mut next_variable_address = 16u16;
    // Second pass through
    for located in commands {
        let instruction = match located.command {
            // For A-instruction, `@` is discarded. e.g., `@70000` -> `70000` or `@value` -> `value`,
            Command::Address(value) => {
                // A-instruction could be all digits
                let address = if value.bytes().all(|byte| byte.is_ascii_digit()) {
                    // Must within u16 range, i.e., 70000 is too large for u16
                    value.parse::<u16>().map_err(|_| {
                        error(located.line, "numeric A-instruction value is out of range")
                    })?
                } else {
                    // A-insturction could also be a symbol so check if the symbol is legit
                    if !is_valid_symbol(&value) {
                        return Err(error(located.line, "invalid A-instruction symbol"));
                    }
                    // If symbol is legit, try to find the corresponding address from symbol table
                    if let Some(address) = symbols.get_address(&value) {
                        // If found and assign it to `value` at begging of this match branch
                        address
                    } else {
                        // If not found, try to write it to symbol table
                        // to write, check if address space is used up or not, i.e., 15-bits space
                        if next_variable_address > 32767 {
                            return Err(error(
                                located.line,
                                "no RAM addresses remain for variables",
                            ));
                        }

                        // If address space is not full, write the new symol and address pair to symbol table
                        let address = next_variable_address;
                        symbols.add_entry(&value, address);

                        // increase the next address
                        next_variable_address += 1;
                        address
                    }
                };
                if address > 32767 {
                    return Err(error(located.line, "A-instruction value exceeds 32767"));
                }

                // Represent the address in string
                format!("{address:016b}")
            }

            // For C-instruction, convert the mnemonics to binary numbers
            Command::Compute(command) => encode_compute(&command).ok_or_else(|| {
                error(located.line, &format!("invalid C-instruction '{command}'"))
            })?,
        };

        // Add binary instruction to output
        output.push_str(&instruction);
        output.push('\n');
    }

    Ok(output)
}

fn error(line: usize, message: &str) -> AssemblerError {
    AssemblerError {
        line,
        message: message.to_owned(),
    }
}

fn is_valid_symbol(symbol: &str) -> bool {
    let mut characters = symbol.chars();
    // Empty string is not valid
    let Some(first) = characters.next() else {
        return false;
    };

    // First character must not be decimal
    (first.is_ascii_alphabetic() || matches!(first, '_' | '.' | '$' | ':'))
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '.' | '$' | ':')
        })
}

fn encode_compute(command: &str) -> Option<String> {
    // A C-instruction must not contain more than 1 "=" or ";"
    if command.matches('=').count() > 1 || command.matches(';').count() > 1 {
        return None;
    }

    // C-instruction like `dest=comp;jump`. First split by "="
    let (destination, remainder) = match command.split_once('=') {
        Some((destination, remainder)) if !destination.is_empty() => (destination, remainder),
        Some(_) => return None,
        // if it has no `dest` field, i.e., `comp;jump`, put `command` into `remainder` for further decoding.
        None => ("", command),
    };

    // Split the `comp;jump` part by ";"
    let (computation, jump) = match remainder.split_once(';') {
        Some((computation, jump)) if !jump.is_empty() => (computation, jump),
        Some(_) => return None,
        // A C-instruction could also contain only a computation, i.e., `D`, or `A`
        None => (remainder, ""),
    };

    // Make sure `computation` is not empty
    if computation.is_empty() {
        return None;
    }

    // Encode `dest`, `comp`, and `jump` into binary bits
    let destination_bits = encode_destination(destination)?;
    let computation_bits = encode_computation(computation)?;
    let jump_bits = encode_jump(jump)?;

    // Change to string format
    Some(format!(
        "111{computation_bits:07b}{destination_bits:03b}{jump_bits:03b}"
    ))
}

fn encode_destination(destination: &str) -> Option<u8> {
    let mut writes_a = false;
    let mut writes_d = false;
    let mut writes_m = false;
    for character in destination.chars() {
        // `writes_flag` is a reference to `writes_a`, `writes_d` or `writes_m`
        let writes_flag = match character {
            'A' => &mut writes_a,
            'D' => &mut writes_d,
            'M' => &mut writes_m,
            // For characters that are not A, D or M, it is a invalid `dest` field
            _ => return None,
        };

        // if `writes_flag` alread set to `true`, then it is a duplication, e.g., `AAD`
        // Return None, because this is not a valid `dest` fields
        if *writes_flag {
            return None;
        }

        // Encode the dest bit to true
        *writes_flag = true;
    }
    // Convert to u8 decimal
    Some(u8::from(writes_a) * 4 + u8::from(writes_d) * 2 + u8::from(writes_m))
}

fn encode_computation(computation: &str) -> Option<u8> {
    Some(match computation {
        "0" => 0b0101010,
        "1" => 0b0111111,
        "-1" => 0b0111010,
        "D" => 0b0001100,
        "A" => 0b0110000,
        "!D" => 0b0001101,
        "!A" => 0b0110001,
        "-D" => 0b0001111,
        "-A" => 0b0110011,
        "D+1" => 0b0011111,
        "A+1" => 0b0110111,
        "D-1" => 0b0001110,
        "A-1" => 0b0110010,
        "D+A" | "A+D" => 0b0000010,
        "D-A" => 0b0010011,
        "A-D" => 0b0000111,
        "D&A" | "A&D" => 0b0000000,
        "D|A" | "A|D" => 0b0010101,
        "M" => 0b1110000,
        "!M" => 0b1110001,
        "-M" => 0b1110011,
        "M+1" => 0b1110111,
        "M-1" => 0b1110010,
        "D+M" | "M+D" => 0b1000010,
        "D-M" => 0b1010011,
        "M-D" => 0b1000111,
        "D&M" | "M&D" => 0b1000000,
        "D|M" | "M|D" => 0b1010101,
        _ => return None,
    })
}

fn encode_jump(jump: &str) -> Option<u8> {
    Some(match jump {
        "" => 0,
        "JGT" => 1,
        "JEQ" => 2,
        "JGE" => 3,
        "JLT" => 4,
        "JNE" => 5,
        "JLE" => 6,
        "JMP" => 7,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::{SymbolTable, assemble};

    #[test]
    fn assembles_a_instructions_and_c_instructions() {
        assert_eq!(
            assemble("@2\nD=A\n@3\nD=D+A\n").unwrap(),
            "0000000000000010\n1110110000010000\n0000000000000011\n1110000010010000\n"
        );
    }

    #[test]
    fn resolves_forward_labels_and_assigns_variables_after_predefined_symbols() {
        assert_eq!(
            assemble("@LOOP\nD=M\n(LOOP)\n@counter\nM=D\n@counter\n").unwrap(),
            "0000000000000010\n1111110000010000\n0000000000010000\n1110001100001000\n0000000000010000\n"
        );
    }

    #[test]
    fn ignores_comments_and_whitespace() {
        assert_eq!(
            assemble("  // comment\n D = M + 1 ; JGT // trailing comment\n").unwrap(),
            "1111110111010001\n"
        );
    }

    #[test]
    fn reports_invalid_instructions_and_out_of_range_addresses() {
        assert_eq!(assemble("D=Q").unwrap_err().line, 1);
        assert_eq!(assemble("@32768").unwrap_err().line, 1);
    }

    #[test]
    fn symbol_table_starts_with_predefined_entries() {
        let symbols = SymbolTable::new();
        assert_eq!(symbols.get_address("SP"), Some(0));
        assert_eq!(symbols.get_address("R15"), Some(15));
        assert_eq!(symbols.get_address("SCREEN"), Some(16384));
    }
}
