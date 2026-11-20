//! # Chapter 4 - Machine Language
//!
//! A machine language is an agreed-upon formalism, designed to code low-level programs as a series of
//! machine instructions, for direct execution in and total control of the computer's harward platform.
//! This is the fine line where hardware and software meet,i.e., the point where the abstrct thoughts
//! of the programmer, as manifested in symbolic instructions, are turned into physical operations performed
//! in silicon.
//!
//! ## Machine
//!
//! It is sufficient to focus on three main abstraction: a **processor**, a **memory**, and a set of **registers**.
//! In this sense, a machine language can be viewed as an agreed-upon formalism, designed to manipulate a **memory**
//! using a **processor** and a set of **registers**.
//!
//!
//! ### Memory
//!
//! From the programmer's standpoint, all memories have the same structure: a continuous array of cells of some
//! fixed width, also called **words**(representing either a data item or an instruction) or **locations**, each
//! having a unique **address**. A **word** is specified by supplying its address, i.e., `Memory[address]`, `RAM[address]`
//! or `M[address]`.
//!
//! ### Processor
//!
//! The processor, also called CPU, typically include arithmetic and logic operation, memory access operation, and control(
//! also called **branching**) operation. The operands of these operations are binary values that come from registers
//! and selected memory locations. Likewise, the results of operations, i.e., CPU's output, can be stored in register or in
//! selected memory locations.
//!
//! ## Languages
//!
//! A machine language program is a series of coded instruction, e.g., a typical instruction in a 16-bit computer may be
//! `1010_0011_0001_1001`(**_** is added for better readability only). To figure out what this instruction mean, we need
//! the rule sets, i.e., the instruction set of underlying hardware platform. E.g., the instruction could mean the operation
//! **set R3 to R1 + R9**, here **Rn** means a register or the content of it.
//! Machine language normally are also specified by symbolic mnenonics, e.g., previous instruction can be specified by **ADD
//! R3,R1,R9**. The symbolic nation is called **assembly language**, or simply **assembly** and the program that translates
//! from assembly to binary is called **assembler**.
//!
//! ## Commands
//!
//! ### Arithmetic and Logic Operations
//!
//! Every computer is required to perform basic airithmetic operation and Boolean operations.
//!
//! ### Memory Access
//!
//! Memory access commands fall into two categories:
//! - Arithmetic and logic operations are performed on registers and selected memory locations.
//! - Explicit **load** and **store** commands to move data between registers and memory.
//!
//! Above memory access commands may use several types of **addressing modes**. Below 3 modes are almost always supported:
//! - **Direct addressing**, to load value of a memory location to a register
//!   - `LOAD R1,67 // R1<-Memory[67]` or assuming that `bar` refers to memory address 67
//!   - `LOAD R1,bar // R1<-Memory[67]`
//! - **Immediate addressing**, to load a constant to a registser
//!   - `LOAD R1,67 // R1<-67`
//! - **Indirect addressing**, to handle **pointers**.
//!   - Address of the required memory location is not hard-coded into the instruction, instead, the instruction specifies
//! a memory location that holds the required address. E.g.,
//!   - Translation of `x=foo[j]` in C(vector `foo` is a pointer that points at the begining of underlying memory area.)
//!   - `ADD R1,foo,j // R1<-foo+j`
//!   - `LOAD R2,R1 // R2<-Memory[R1]`
//!   - `STR R2,x // x<-R2`
//!
//! ### Flow of Control
//!
//! Branching serves several purposes including **repetition**, **conditional execution**, **subroutine calling**.
//!
//!
//! ## Hack Machine Language Specification
//! The Hack computer is a 16-bit von Neumann machine, consisting of a CPU, two separate memory modules serving as instruction
//! memory(ROM) and data memory(RAM), and two memory-mapped I/O devices: a screen and a keyboard.
//!
//! ### Memory Address Space
//!
//! - **instruction memory**, 16-bit register, 15-bit address space thus 32k size, i.e., 32k 16-bit words
//! - **data memory**, 16-bit register, 15-bit address space thus 32k size, i.e., 32k 16-bit words
//!
//! ### Registers
//! - **data register**(D), 16-bit register, used to store data values
//! - **address register**(A), 16-bit register, used to store data values, address in data memory, or address in instruction memory
//!
//! Since Hack instructions are 16-bit wide, and address are specified using 15 bits, it is impossible to pack both operation
//! code and address in one instruction. Thus Hack machine language mandates that memory access instructions operate on an implicit
//! memory location labeled **M**.
//! - E.g., `D=M+1`, **M** always refers to the memory word whose address is the current value of A register.
//! - E.g., `D=Memory[516]-1`, we need to use one instruction to set A register to 516, and a subsequent instruction ot specify
//! `D=M-1`
//! - E.g., to effect the operation `goto 35`, we use one instruction to set A to 35, and a second instruction to code the `goto`
//! command, without specifying an address.
//!
//! ### A-instruction
//! The A-instruction, `@value`, is used to set the A register to a 15-bit value
//! - `value` is either a non-negative decimal number or a symbol referring to such number
//! - Binaryformat(_ is added for readability only): "0vvv_vvvv_vvvv_vvvv", v =0 or 1
//! - e.g. `@5`, in binary format is "0000_0000_0000_0101", causes the computer to store the binary representation of 5 in A register.
//!
//! The A-instruction is used for 3 purposes:
//! - enter a constant to computer program
//! - set stage for subsequent C-instruction that manipulates a data memory location,i.e.,
//!   - first store a target data memory address/location in A-register
//!   - C-instruction manipulates the content of the address that stored in A-register
//! - set stage for subsequent C-instruction that specifies a jump,i.e.,
//!   - first store address/location of jump desitnation in A-register
//!   - C-instruction jump to the address that stored in A-register
//!
//! ### C-instruction
//!
//! A C-instruction code, , is a specification that answers 3 questions:
//! - what to compute
//! - where to store the computed value
//! - what to do next
//!
//! Mnemonics form, e.g, `dest=comp;jump`,
//! - Either the `dest` or `jump` field maybe empty
//!   - if `dest` is empty, the `=` is omitted;
//!   - if `jump` is empty, the `;` is omitted;
//!
//! Bianry form, e.g, "111_acccccc_ddd_jjj"
//! - leftmost bit is C-instruction code, which is 1
//! - next 2 bits are not used
//! - the "acccccc", i.e., `comp` field, instructs the ALU what to compute
//! - the "ddd", i.e., `dest` field, instructs where to store the computed value, i.e., ALU output
//! - the "jjj", i.e., `jump` field, specifies a jump condition, i.e., which command to fetch and execute next.
//!
//! ### Computation Specification
//!
//! Hack ALU is designed to compute a fixed set of functions on the D, A and M (i.e., `Memory[A]`) registers.
//!
//! | when a=0 | c1 | c2 | c3 | c4 | c5 | c6 | when a=1 |
//! |:--------:|:--:|:--:|:--:|:--:|:--:|:--:|:--------:|
//! |    0     | 1  | 0  | 1  | 0  | 1  |  0 |    0     |
//! |    1     | 1  | 1  | 1  | 1  | 1  |  1 |    1     |
//! |   -1     | 1  | 1  | 1  | 0  | 1  |  0 |   -1     |
//! |    D     | 0  | 0  | 1  | 1  | 0  |  0 |    D     |
//! |    A     | 1  | 1  | 0  | 0  | 0  |  0 |    M     |
//! |   !D     | 0  | 0  | 1  | 1  | 0  |  1 |   !D     |
//! |   !A     | 1  | 1  | 0  | 0  | 0  |  1 |   !M     |
//! |   -D     | 0  | 0  | 1  | 1  | 1  |  1 |   -D     |
//! |   -A     | 1  | 1  | 0  | 0  | 1  |  1 |   -M     |
//! |   D+1    | 0  | 1  | 1  | 1  | 1  |  1 |   D+1    |
//! |   A+1    | 1  | 1  | 0  | 1  | 1  |  1 |   M+1    |
//! |   D-1    | 0  | 0  | 1  | 1  | 1  |  0 |   D-1    |
//! |   A-1    | 1  | 1  | 0  | 0  | 1  |  0 |   M-1    |
//! |   D+A    | 0  | 0  | 0  | 0  | 1  |  0 |   D+M    |
//! |   D-A    | 0  | 1  | 0  | 0  | 1  |  1 |   D-M    |
//! |   A-D    | 0  | 0  | 0  | 1  | 1  |  1 |   M-D    |
//! |   D&A    | 0  | 0  | 0  | 0  | 0  |  0 |   D&M    |
//! |   D\|A   | 0  | 1  | 0  | 1  | 0  |  1 |   D\|M   |
//!
//! The ALU is designed to take D as first input is D. For second input, it is A when `a=0` and M when `a=1`.
//! The `c` bits control the ALU in this order:
//! - `c1` (`zx`) sets the first input to zero when 1, do nothing when 0.
//! - `c2` (`nx`) bitwise-negates the first input when 1, do nothing when 0.
//! - `c3` (`zy`) sets the second input to zero when 1, do nothing when 0.
//! - `c4` (`ny`) bitwise-negates the second input when 1, do nothing when 0.
//! - `c5` (`f`) selects addition when 1 and bitwise AND when 0.
//! - `c6` (`no`) bitwise-negates the output when 1, do nothing when 0.
//!
//! The ALU applies these controls in order: zero and optionally negate each input, perform the
//! selected operation, then optionally negate the output. `!` in the table denotes bitwise
//! negation; arithmetic results are represented using 16-bit words.
//!
//! The following examples follow the table order. Each `comp` pattern is written as `a` followed
//! by `c1` through `c6`. Let Y denote the selected second input: A when `a=0`, or M when `a=1`.
//! `!` is bitwise negation, `!0` is -1 in a 16-bit word, and arithmetic uses 16-bit words.
//!
//! Bit-wise negates(!) operations used in below examples-
//! - `!0 = -1`
//!
//!   `0 = 0000_0000_0000_0000`
//!
//!  `!0 = 1111_1111_1111_1111` is `-1` using 2's complement arithmetic
//!
//! - `!(-2) = 1`
//!
//!    `-2  = 1111_1111_1111_1110`
//!
//!  `!(-2) = 0000_0000_0000_0001`
//!
//! - `D & -1`
//!
//!     `D  = 0000_0000_0000_0101`
//!
//!     `-1 = 1111_1111_1111_1111`
//!
//! `D & -1 = 0000_0000_0000_0101`
//!
//! - `!(D-1) = -D`
//!   - in 16-bit two’s-complement arithmetic, bitwise NOT follows the identity `!x = -x - 1`.
//!   - Set `x = D - 1`, then `!(D-1) = -(D-1) - 1 = -D`
//!   - e.g.,
//!     `D-1    = 0000_0000_0000_0100`   // 4
//!
//!     `!(D-1) = 1111_1111_1111_1011`   // -5
//!
//! 1. **Compute 0.** The c-bits patterns are `101010` (`a=0`) and `101010` (`a=1`).
//!    - `a` selects A or M as the second input.
//!    - `c1=1` sets the first input to 0;
//!    - `c2=0` leaves it at 0.
//!    - `c3=1` sets the second input to 0;
//!    - `c4=0` leaves it at 0.
//!    - `c5=1` adds the inputs: `0 + 0 = 0`.
//!    - `c6=0` leaves the output unchanged, so the result is 0.
//!
//! 2. **Compute 1.** The c-bits patterns are `111111` (`a=0`) and `111111` (`a=1`).
//!    - `a` selects A or M, but the controls below replace that value.
//!    - `c1=1` sets the first input to 0;
//!    - `c2=1` bit-wise negates it, giving `!0 = -1`.
//!    - `c3=1` sets the second input to 0;
//!    - `c4=1` negates it, giving `!0 = -1`.
//!    - `c5=1` adds the inputs: `-1 + -1 = -2`.
//!    - `c6=1` negates the output: `!(-2) = 1`.
//!
//! 3. **Compute -1.** The c-bits patterns are `111010` (`a=0`) and `111010` (`a=1`).
//!    - `a` selects A or M, but `c3=1` will set the second input to 0.
//!    - `c1=1` sets the first input to 0;
//!    - `c2=1` negates it, giving `!0 = -1`.
//!    - `c3=1` sets the second input to 0;
//!    - `c4=0` leaves it at 0.
//!    - `c5=1` adds the inputs: `-1 + 0 = -1`.
//!    - `c6=0` leaves the output unchanged, so the result is -1.
//!
//! 4. **Compute D.** The c-bits patterns are `001100` (`a=0`) and `001100` (`a=1`).
//!    - `a` selects A or M, but `c3=1` sets the second input to 0.
//!    - `c1=0` and `c2=0` leave the first input as D.
//!    - `c3=1` sets the second input to 0;
//!    - `c4=1` negates it, giving `!0 = -1`, so second input is now `-1`.
//!    - `c5=0` takes the bitwise AND: `D & -1 = D`.
//!    - `c6=0` leaves the output unchanged, so the result is D.
//!
//! 5. **Compute A or M.** The c-bits patterns are `110000` (`a=0`) and `110000` (`a=1`).
//!    - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!    - `c1=1` sets the first input to 0;
//!    - `c2=1` negates it, giving `!0 = -1`.
//!    - `c3=0` and `c4=0` leave the second input as Y.
//!    - `c5=0` takes the bitwise AND: `-1 & Y = Y`.
//!    - `c6=0` leaves the output unchanged, so the result is A or M.
//!
//! 6. **Compute !D.** The c-bits patterns are `001101` (`a=0`) and `001101` (`a=1`).
//!    - `a` selects A or M, but `c3=1` sets the second input to 0.
//!    - `c1=0` and `c2=0` leave the first input as D.
//!    - `c3=1` sets the second input to 0;
//!    - `c4=1` negates it, giving `!0 = -1`.
//!    - `c5=0` takes the bitwise AND: `D & -1 = D`.
//!    - `c6=1` negates the output, giving !D.
//!
//! 7. **Compute !A or !M.** The c-bits patterns are `110001` (`a=0`) and `110001` (`a=1`).
//!    - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!    - `c1=1` sets the first input to 0;
//!    - `c2=1` negates it, giving `!0 = -1`.
//!    - `c3=0` and `c4=0` leave the second input as Y.
//!    - `c5=0` takes the bitwise AND: `-1 & Y = Y`.
//!    - `c6=1` negates the output, giving !Y.
//!
//! 8. **Compute -D.** The c-bits patterns are `001111` (`a=0`) and `001111` (`a=1`).
//!    - `a` selects A or M, but `c3=1` sets the second input to 0.
//!    - `c1=0` and `c2=0` leave the first input as D.
//!    - `c3=1` sets the second input to 0;
//!    - `c4=1` negates it, giving `!0 = -1`.
//!    - `c5=1` adds the inputs: `D + -1 = D-1`.
//!    - `c6=1` negates the output: `!(D-1) = -D`.
//!
//! 9. **Compute -A or -M.** The c-bits patterns are `110011` (`a=0`) and `110011` (`a=1`).
//!    - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!    - `c1=1` sets the first input to 0; `c2=1` negates it, giving `!0 = -1`.
//!    - `c3=0` and `c4=0` leave the second input as Y.
//!    - `c5=1` adds the inputs: `-1 + Y = Y-1`.
//!    - `c6=1` negates the output: `!(Y-1) = -Y`.
//!
//! 10. **Compute D+1.** The c-bits patterns are `011111` (`a=0`) and `011111` (`a=1`).
//!     - `a` selects A or M, but `c3=1` sets the second input to 0.
//!     - `c1=0` and `c2=1` leave the first input as D, then negate it to !D.
//!     - `c3=1` sets the second input to 0; `c4=1` negates it, giving `!0 = -1`.
//!     - `c5=1` adds the inputs: `!D + -1`.
//!     - `c6=1` negates the output: `!(!D + -1) = D+1`.
//!       - using identity `!x = -x -1`
//!       - `!D + -1 = (-D - 1) - 1 = -D - 2`
//!       - `!(!D + -1) = !(-D - 2) = -(-D - 2) - 1 = D + 1`
//!
//! 11. **Compute A+1 or M+1.** The c-bits patterns are `110111` (`a=0`) and `110111` (`a=1`).
//!     - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!     - `c1=1` sets the first input to 0; `c2=1` negates it, giving `!0 = -1`.
//!     - `c3=0` leaves the second input as Y; `c4=1` negates it to !Y.
//!     - `c5=1` adds the inputs: `-1 + !Y`.
//!     - `c6=1` negates the output: `!(-1 + !Y) = Y+1`.
//!
//! 12. **Compute D-1.** The c-bits patterns are `001110` (`a=0`) and `001110` (`a=1`).
//!     - `a` selects A or M, but `c3=1` sets the second input to 0.
//!     - `c1=0` and `c2=0` leave the first input as D.
//!     - `c3=1` sets the second input to 0; `c4=1` negates it, giving `!0 = -1`.
//!     - `c5=1` adds the inputs: `D + -1 = D-1`.
//!     - `c6=0` leaves the output unchanged, so the result is D-1.
//!
//! 13. **Compute A-1 or M-1.** The c-bits patterns are `110010` (`a=0`) and `110010` (`a=1`).
//!     - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!     - `c1=1` sets the first input to 0; `c2=1` negates it, giving `!0 = -1`.
//!     - `c3=0` and `c4=0` leave the second input as Y.
//!     - `c5=1` adds the inputs: `-1 + Y = Y-1`.
//!     - `c6=0` leaves the output unchanged, so the result is Y-1.
//!
//! 14. **Compute D+A or D+M.** The c-bits patterns are `000010` (`a=0`) and `000010` (`a=1`).
//!     - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!     - `c1=0` and `c2=0` leave the first input as D.
//!     - `c3=0` and `c4=0` leave the second input as Y.
//!     - `c5=1` adds the inputs: `D + Y`.
//!     - `c6=0` leaves the output unchanged, so the result is D+A or D+M.
//!
//! 15. **Compute D-A or D-M.** The c-bits patterns are `010011` (`a=0`) and `010011` (`a=1`).
//!     - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!     - `c1=0` leaves the first input as D; `c2=1` negates it to !D.
//!     - `c3=0` and `c4=0` leave the second input as Y.
//!     - `c5=1` adds the inputs: `!D + Y`.
//!     - `c6=1` negates the output: `!(!D + Y) = D-Y`.
//!       - using identity `!x = -x - 1`
//!       - `!(!D + Y) = -(!D + Y) - 1 = -!D - 1 - Y = !!D - Y = D - Y`
//!
//! 16. **Compute A-D or M-D.** The c-bits patterns are `000111` (`a=0`) and `000111` (`a=1`).
//!     - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!     - `c1=0` and `c2=0` leave the first input as D.
//!     - `c3=0` leaves the second input as Y; `c4=1` negates it to !Y.
//!     - `c5=1` adds the inputs: `D + !Y`.
//!     - `c6=1` negates the output: `!(D + !Y) = Y-D`.
//!
//! 17. **Compute D&A or D&M.** The c-bits patterns are `000000` (`a=0`) and `000000` (`a=1`).
//!     - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!     - `c1=0` and `c2=0` leave the first input as D.
//!     - `c3=0` and `c4=0` leave the second input as Y.
//!     - `c5=0` takes the bitwise AND: `D & Y`.
//!     - `c6=0` leaves the output unchanged, so the result is D&A or D&M.
//!
//! 18. **Compute D|A or D|M.** The c-bits patterns are `010101` (`a=0`) and `010101` (`a=1`).
//!     - `a=0` selects A; `a=1` selects M. Call the selected value Y.
//!     - `c1=0` leaves the first input as D; `c2=1` negates it to !D.
//!     - `c3=0` leaves the second input as Y; `c4=1` negates it to !Y.
//!     - `c5=0` takes the bitwise AND: `!D & !Y`.
//!     - `c6=1` negates the output: `!(!D & !Y) = D|Y`.
//!       - De Morgan's law applied bit by bit, i.e.,
//!
//! | D | Y | !D | !Y | !D & !Y | !(!D & !Y) | D \| Y |
//! |:-:|:-:|:--:|:--:|:-------:|:----------:|:-----: |
//! | 0 | 0 |  1 |  1 |    1    |      0     |   0    |
//! | 0 | 1 |  1 |  0 |    0    |      1     |   1    |
//! | 1 | 0 |  0 |  1 |    0    |      1     |   1    |
//! | 1 | 1 |  0 |  0 |    0    |      1     |   1    |
//!
//! ### Destination Specification
//!
//! The value computed by the `comp` part of C-instruction can be stored in several destinations, specified by `dest`:
//! - first bit code whether to store the computed value in A register
//! - second bit code whether to store the computed value in D register
//! - third bit codes whether to store the computed value in M, i.e., in `Memory[A]`
