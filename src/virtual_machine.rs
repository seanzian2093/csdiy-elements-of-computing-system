//! # Chapter 7&8 - Virtual Machine
//!
//! Building a **compiler** for a typical object-based high-level language is approached
//! in two stages, each spanning two chapters:
//! - High-level programs will first be translated into intermediate code(chapter 10&11)
//! - intermediate code will be translated into target machine language (chapter 7&8)
//!
//! This two-tier translation model is adopted by Java and C#.
//!
//! Intermediate code is designed to run on a **Virtual Machine**, an abstract computer
//! that is realized on other computer platform. This chapter presents a typical VM architecture
//! modeled after **JVM**(Java Virtual Machine) paradigm. This VM architecture is achieved
//! - Specify the VM abstraction
//! - Implement it over Hack platform, which entails
//!   - writing a program, **VM translator**, that translate VM language(intermediate code) into
//!     Hack assemly code and then machine language
//!
//! VM presented in this chapter is based on stack, which is a fundamental and elegant data
//! structure that comes to play in many computer systems and algorithms.
//!
//! ## Background
//!
//! ### VM Paradigm
//!
//! Before a high-level program can run on a target computer, it must be translated into the
//! computer's machine language. This translation is called **compilation** which is rather
//! complex process. So a compiler is dependent on its source and destination language. One
//!  way to decouple this dependency is to break the overall compilation process into two
//! separate stages.
//! - high-level program is parsed and its commands are translated into intermediate language
//! - intermedia language is translanted further into machine language of the target hardware.
//!
//! The interface between above two stages is sufficiently important to merit itself a stand-
//! alone language of an asbstract machine. Specifically, we can formulate a **virtual machine**
//! whose instruction are the intermediate steps into which high-level language are decomposed.
//! Sometimes the term **compiler** refers to the program that performs first step.
//!
//! ### Stack Machine Model
//!
//! VM language implementation needs to answer a key question: **where will the operands and the
//! results of the VOM operation reside?**. The clearest solution is to put them on **stack**
//! data structure, i.e., a **stack machine** model.
//!
//! In a **stack machine** model,
//! - arithmetic commands pop their operands from the top of the stack and push their results
//!   back onto the top of the stack.
//! - other commands transfer data items from the stack's top to designated memory locations,
//!   and vice versa.
//! - These simple stack operations can be used to implement the evaluation of any arithmetic
//!   or logical expression.
//! - Any program, written in any programming language, can be translated into an equivalent
//!   stack machine program, e.g., one such stack machine is used in **JVM**
//!
//! A stack is an abstrct data structure(not the memory) that supports two basic operations:
//! - **push** adds an element to the top of the stack, and
//! - **pop** retrieves and removes top element, i.e., **last-in-first-out**(LIFO) storage model.
//!
//! Simplest approach to implement stack data structure is to keep
//! - an array **stack**, and
//! - a **stack pointer** variable, e.g., `sp` that points to the available location above the
//!   topmost element.
//!
//! Then `push x` commands is then implemented by
//! - storing `x` at the array entry pointed by `sp`, i.e., `stack[sp]=x`,and
//! - incrementing `sp`, i.e., `sp=sp+1`
//!
//! The `pop` operation is implemented by
//! - decrementing `sp`, i.e., `sp=sp-1`, and
//! - return the value stored in the top position, i.e., `return stack[sp]`
//!
//! ### Stack Arithmetic
//!
//! Stack-based arithmetic is a simple matter: operands are popped from the stack and required
//! operation is performed on them, and result is pushed back onto the stack. Any arithmetic and
//! Boolean expression can be systematically converted into and evaluated by, a sequence of simple
//! operations on a stack. Thus we can write a compiler that translate high-level arithmetic and
//! Boolean expressions into sequences of stack command.
//!
//! ## VM Specification - Part I
//!
//! ### General
//!
//! - stack-based
//! - function-based
//! - 16-bit data type
//! - 4 types of commands
//!   - Arithmetic commands
//!   - Memory access commands
//!   - Program flow commands
//!   - Function calling commands
//!
//! ### Program and Command Structure
//!
//! A VM **program** is a collection of one or more **files** with a **.vm** extension, each consisting
//! of one or more **functions**. A correspondence between object-oriented language is as below
//! - **program** -> **program**
//! - **file** -> **class**
//! - **function** -> **method**
//!
//! Within a **.vm** file, each VM command appears in a separate line, and in one of the following formats
//! - **command**, e.g., **add**
//! - **command arg**, e.g., **goto loop**
//! - **command arg1 arg2**, e.g., **push local 3**
//!
//! The arguments are separated from each other and from the **command** part by an arbitrary number of
//! spaces. "//" comments can appear at the end of any line and are ignored. Blank lines are permitted and
//! ignored.
//!
//! ### Arithmetic and Logical Commands
//!
//! The VM language features nine stack-oriented arithmetic and logical commands.
//! | Command | Return value           | Comment                              |
//! |:-------:|:----------------------:|:------------------------------------:|
//! | add     | x + y                  | Integer addtion (2's complement)     |
//! | sub     | x - y                  | Integer substraction (2's complement)|
//! | neg     |  - y                   | Arithmetic negation (2's complement) |
//! | eq      | true if x=y else false | Equality                             |
//! | gt      | true if x>y else false | Greater than                         |
//! | lt      | true if x<y else false | Less than                            |
//! | and     | x And y                | Bit-wise                             |
//! | or      | x Or  y                | Bit-wise                             |
//! | not     |  Not y                 | Bit-wise                             |
//!
//! ### Memeory Access Commands
//!
//! The VM manipulates 8 separate **virtual memory segments**. All the memory segments are accessed by the same two commands:
//! - `push segment index`: push the value of `segment[index]` onto the stack
//! - `pop segment index`: pop the top stack value and store it in `segment[index]`
//!
//! where `segment` is one of below 8 segment names, and `index` is non-negative integer. E.g
//! - `push argument 2; pop local 1` performs below tasks in that order
//!   - push the function's 3rd argument(each segment's index starts at 0) to top of stack
//!   - pop that newly added topmost entry of stack and store it in `local[1]`
//!
//! In addition to the 8 memory segments, which are explicitly managed by VM push and pop commands, the VM implementation manages
//! two implicit data structure called **stack** and **heap**.
//!
//! <table>
//! <thead>
//! <tr><th>Segment</th><th>Purpose</th><th>Comment</th></tr>
//! </thead>
//! <tbody>
//! <tr>
//!   <td><code>arguments</code></td>
//!   <td>Stores the function's arguments.</td>
//!   <td>Allocated dynamically by the VM implementation when the function is entered.</td>
//! </tr>
//! <tr>
//!   <td>local</td>
//!   <td>Stores the function's local variables.</td>
//!   <td>Allocated dynamically by the VM implementation and initialized to 0's when the function is entered.</td>
//! </tr>
//! <tr>
//!   <td>static</td>
//!   <td>Stores static variables shared by all functions in the same <b>.vm</b> file.</td>
//!   <td>Allocated dynamically by the VM implementation for each <b>.vm</b> file; shared by all functions in the <b>.vm</b> file.</td>
//! </tr>
//! <tr>
//!   <td>constant</td>
//!   <td>Pseudo-segment that holds all the constants in the range 0...32767</td>
//!   <td>Emulated by the VM implementation. Seen by all functions in the program.</td>
//! </tr>
//! <tr>
//!   <td>this, that</td>
//!   <td>General-purpose segments. Can be made to correspond to different areas in the heap.
//!       Serve various programming needs
//!   </td>
//!   <td>Any VM function can use these segments to manipulate selected areas on the heap.</td>
//! </tr>
//! <tr>
//!   <td>pointer</td>
//!   <td>A two-entry segment that holds the base addresses of the <code>this</code> or <code>that</code> segments.</td>
//!   <td>Any VM function can set <code>pointer</code> 0(or 1) to some address. This has the effect of aligning
//!       the <code>this</code> (or <code>that</code>) segment to the heap area beginning at that address.
//!   </td>
//! </tr>
//! <tr>
//!   <td>temp</td>
//!   <td>Fixed eight-entry segment that holds temporary variables for general use.</td>
//!   <td>May be used by any VM function for any purpose. Shared by all functions in the program.</td>
//! </tr>
//! </tbody>
//! </table>
//!
//! ### Program Flow and Function Calling Commands
//!
//! The VM features 6 additional commands(`functionName` is a symbol and `nLocals` and `nArgs` are non-negative integers):
//! - <code>label <b>symbol</b></code>: Label declaration
//! - <code>goto <b>symbol</b></code>: Unconditional branching
//! - <code>if-goto <b>symbol</b></code>: Conditional branching
//! - <code>function <b>functionName nLocals</b></code>: Function declaration, specifying the number of function's local variables
//! - <code>call <b>functionName nArgs</b></code>: Function invocation, specifying the number of function's local arguments
//! - <code>return</code>: Transfer control back to the calling function
//!
//! ## Implementation - Part I
//!
//! ### VM to Hack Translation
//!
//! A VM program consists of one or more **.vm files*, each
//!   - each **file** contains one or more VM **functions**(order does not matter)
//!     - each **function** contains a sequence of VM **commands**.
//!       he VM translator takes a collection of **.vm files** and produces a single Hack assembly language, i.e., **.asm** file.
//!
//! ### RAM Usage
//!
//! Data memory of the Hack computer consists of 32k 16-bit words. First 16k serves as general-purpose RAM. Next 16k contain
//! memory maps of I/O devices and below mappings:
//!
//! | RAM address | Usage                                                |
//! | -----------:|:----------------------------------------------------:|
//! | 0-15        | 16 virtual registers, usage described below          |
//! | 16-255      | Static variables(of all VM functions in VM program)  |
//! | 256-2047    | Stack                                                |
//! | 2048-16483  | Heap (to store objects and arrays)                   |
//! | 16384-24575 | Memory mapped I/O                                    |
//!
//! 16 virtual registers are as below table:
//!
//! | Register     | Name | Usage                                                                   |
//! |:-------------|:----:|:------------------------------------------------------------------------|
//! | RAM\[0\]     | SP   | Stack pointer, points to next topmost location in stack                 |
//! | RAM\[1\]     | LCL  | Points to base of current VM function's `local` segment                 |
//! | RAM\[2\]     | ARG  | Points to base of current VM function's `argument` segment              |
//! | RAM\[3\]     | THIS | Points to base of current VM function's `this` segment(within the heap) |
//! | RAM\[4\]     | THAT | Points to base of current VM function's `that` segment(within the heap) |
//! | RAM\[5-12\]  |      | Holds the contents of the `temp` segment                                |
//! | RAM\[13-15\] |      | Can be used by the VM implementation as general-purpose registers       |
//!
//! ### Program Structure
//!
//! - The **Parser** Module
//!
//! **Parser** parses a single **.vm** file, removes all white spaces and comments.
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
//!   <td>Constructor</td>
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
//!     only f <code>hasMoreCommands()</code> is true. Initially there is no current command.
//!   </td>
//! </tr>
//! <tr>
//!   <td>commandType</td>
//!   <td>NA</td>
//!   <td>
//!     C_ARITHMETIC,<br>
//!     C_PUSH,<br>
//!     C_POP,<br>
//!     C_LABEL,<br>
//!     C_GOTO,<br>
//!     C_IF,<br>
//!     C_FUNCTION,<br>
//!     C_RETURN,<br>
//!     C_CALL,<br>
//!   </td>
//!   <td>
//!     Returns the type of the current VM command: <code>C_ARITHMETIC</code> is returned for all arithmetic commands.
//!   </td>
//! </tr>
//! <tr>
//!   <td>arg1</td>
//!   <td>NA</td>
//!   <td>string</td>
//!   <td>
//!     Returns the first argument of current command. In case of <code>C_ARITHMETIC</code>, the command itself is returned.
//!     Should not be called if the current command is <code>C_RETURN</code>
//!   </td>
//! </tr>
//! <tr>
//!   <td>arg2</td>
//!   <td>NA</td>
//!   <td>int</td>
//!   <td>
//!     Returns the second argument of current command. Should be called only if the current command is <code>C_PUSH</code>,
//!     <code>C_POP</code>, <code>C_FUNCTION</code>, <code>C_CALL</code>.
//!   </td>
//! </tr>
//! </tbody>
//! </table>
//!
//! ### The CodeWriter Module
//!
//! **CodeWriter** translates VM commands into Hack assembly code.
//!
//! | Routine         | Arguments                           | Returns | Function                                                                           |
//! |:----------------|:-----------------------------------:|:-------:|:-----------------------------------------------------------------------------------|
//! | Constructor     | Output file/stream                  | NA      | Opens the output file/stream and gets ready to write into it                       |
//! | setFileName     | fileName(string)                    | NA      | Informs the code writer that translation of a new VM file is started               |
//! | writeArithmetic | command(string)                     | NA      | Writes assembly code that is translation of the given arithmetic command           |
//! | writePushPop    | command, segment, index             | NA      | Writes assembly code that is translation of the given command, `C_PUSH` or `C_POP` |
//! | close           |       NA                            | NA      | Closes the output file                                                             |
//! | writeInit       |       NA                            | NA      | Writes boostrap code and put it at beginning of output file                        |
//! | writeLabel      | Label(string)                       | NA      | Writes assembly code that effects the `label` command                              |
//! | writeGoto       | Label(string)                       | NA      | Writes assembly code that effects the `goto` command                               |
//! | writeIf         | Label(string)                       | NA      | Writes assembly code that effects the `if-goto` command                            |
//! | writeCall       | functionName(string), numArgs(int)  | NA      | Writes assembly code that effects the `call` command                               |
//! | writeReturn     | NA                                  | NA      | Writes assembly code that effects the `return` command                             |
//! | writeFunction   | functionName(string), numLocals(int)| NA      | Writes assembly code that effects the `goto` command                               |
//!
//! ### Main Program
//!
//! **Main program** should construct a **Parser** to parse the VM input file and a **CodeWriter** to
//! generate code into output file. If the main program's argument is a directory name rather than a
//! file name, it should process all **.vm** files in this directory. In doing so, separate **Parser**
//! be used to parse each input file and a single **CodeWriter** for handling the output.
//!
//! ## VM Specification - Part I
//!
//! ### Subroutine Calling
//!
//! In programming language, high-level, programmer-defined operations are called **subroutines**,
//! **functions**, **procedures**, **functions** or **methods** in object-oriented language. From now we
//! call it **subroutines**.
//!
//! We use the term **frame**, conceptually, to refer to the subroutine's local variables, the arguments
//! on which it operates, its **working stack** and other memory segments that support its operation.
//!
//! From now on, when we say **stack**, we mean **global stack**, i.e., the memory area containing the
//! frames of the current subroutines, and all subroutines waiting for it to return.
//!
//! Those two **stack** are closely related since **working stack** of current subroutine is located at
//! the very tip of the **global stack**.
//!
//! ### Program Flow Commands
//!
//! - <code>label <em>label</em></code>
//!   - labels current location in the function's code. Only labeled locations can be jumpped to.
//!   - Scope is the function in which the label is defined.
//!   - <em>label</em> is an arbitrary string composed of any sequence of letters, digits, underscore,
//!     dot, and colon that does not start with a digit.
//! - <code>goto <em>label</em></code>
//!   - effects an **unconditional goto** operation
//!     - causing execution to continue from the location marked by the `label`.
//!     - Jump destination must be located in the same function.
//! - <code>if-goto <em>label</em></code>
//!   - effects a **conditional goto** operation.
//!     - The stack's topmost value is popped.
//!     - if the value is not zero, execution continues from location marked by `label`.
//!     - otherwise ocntinue from next command in the program.
//!     - Jump destination must be located in the same function.
//!
//! ### Function Calling Commands
//!
//! A function has a symbolic name that is used globally to call it. The name is an arbitrary string
//! composed of any sequence of letters, digits, underscore, dot, and colon that does not start with digit.
//!
//! - <code>function <em>f n</em></code> starts code of function `f` that has `n` local variables.
//! - <code>call <em>f m</em></code> calls function `f`, stating that `m` arguments have already been pushed
//!   onto stack by called.
//! - `return` returns to the calling function.
//!
//! ### Function Calling Protocol - Part I
//!
//! The events of calling a function and returning from a function can be viewed from two different perspectives
//!
//! - the calling function view
//!   - Before calling the function, caller must push as many arguments as necessary onto stack;
//!   - Next, caller invokes the function using `call` command;
//!   - After the called function returns,arguments that caller pushed before the call have disappeared from
//!     the stack, and a `return value`(that always exists) appears at the top of the stack;
//!   - After the called function returns, caller'r memory segments `argument`, `local`, `static`, `this`,
//!     `that`, and `pointer` are same as before the call, and `temp` segment is undefined.
//!
//! - the called function view
//!   - when called function starts executing,
//!     - its `argument` segment has been initialized with actual values passed by caller and
//!     - its `local` variables segment has been allocated and initialized as zeros,
//!     - the `static` segment that the called function sees has been set to the `static` segment of the VM file
//!       to which it belongs.
//!     - its `working stack` that it sees is empty.
//!     - the `this`, `that`, `pointer`, and `temp` are undefined upon entry.
//!   - before returning, the called function must push a value onto the stack.
//!
//! ### Initialization
//!
//! VM starts or (reset) with an argument-less VM function called `Sys.init`, which then calls main function in
//! user's program. Thus compilers that generate VM code must ensure that each translated program has one such
//! `Sys.init` function.
//!
//! VM stack should be mapped on the location RAM\[256\] onward.
//!
//! ## Implementation - Part II
//!
//! ### Function Calling Protocol - Part II
//!
//! The function calling protocol and the global stack structure implied by it can be implemented and the Hack
//! platform by effecting, in Hack assembly, the pseudo-code below
//!
//!
//! <table>
//! <thead>
//! <tr><th>VM Coomand</th><th>Generated Code</th></tr>
//! </thead>
//! <tbody>
//! <tr>
//!   <td>
//!     <code><b>call f n</b></code><br>
//!     (calling a function <code>f</code> after <code>n</code> <br>arguments have been pushed onto the stack)
//!   </td>
//!   <td>
//!     <code>push return-address</code> // (using the label declared below)<br>
//!     <code>push LCL</code> // save <code>LCL</code> of the calling function<br>
//!     <code>push ARG</code> // save <code>ARG</code> of the calling function<br>
//!     <code>push THIS</code> // save <code>THIS</code> of the calling function<br>
//!     <code>push THAT</code> // save <code>THAT</code> of the calling function<br>
//!     <code>ARG = SP-n-5</code> // Reposition <code>ARG</code>(<code>n</code>is number of args)<br>
//!     <code>LCL= SP</code> // Reposition <code>LCL</code><br>
//!     <code>goto f</code> // Transfer control<br>
//!     <code>(return-address)</code> // Declare a label for the return-address
//!   </td>
//! </tr>
//! <tr>
//!   <td>
//!     <code><b>function f k</b></code><br>
//!     (declararing a function <code>f</code> that has <code>k</code> <br>local variables)
//!   </td>
//!   <td>
//!     <code>(f)</code> // Declare a label for the function entry<br>
//!     repeat <code>k</code> times: // <code>k</code> = number of local variables<br>
//!     <code>push 0</code> // Initialize all of them to 0<br>
//!   </td>
//! </tr>
//! <tr>
//!   <td>
//!     <code><b>return</b></code><br>
//!     (from a function)
//!   </td>
//!   <td>
//!     <code>FRAME = LCL</code> // <code>FRAME</code> is a temporary variable<br>
//!     <code>RET = *(FRAME-5)</code> // Put the return-address in a temp variable<br>
//!     <code>*ARG = pop()</code> // Reposition the return value for the caller<br>
//!     <code>SP = ARG+1</code> // Restore <code>SP</code> of the caller<br>
//!     <code>THAT = *(FRAM-1)</code> // Restore <code>THAT</code> of the caller<br>
//!     <code>THIS = *(FRAM-2)</code> // Restore <code>THIS</code> of the caller<br>
//!     <code>ARG = *(FRAM-3)</code> // Restore <code>ARG</code>of the caller<br>
//!     <code>LCL = *(FRAM-4)</code> // Restore <code>LCL</code>of the caller<br>
//!     <code>goto RET</code> // Goto return-address (in caller's code)<br>
//!   </td>
//! </tr>
//! </tbody>
//! </table>
//!
//! ### Assembly Language Symbol
//!
//! The implementation of **program flow** and **function calling** commands requires VM
//! implementation to create and use special symbols at assembly level. A complete list
//! of the symbols are as below
//!
//! <table>
//! <thead>
//! <tr><th>Symbol</th><th>Usage</th></tr>
//! </thead>
//! <tbody>
//! <tr>
//!   <td> <code>SP, LCL, ARG, THIS, THAT</code> </td>
//!   <td>
//!     These predefined symbols point, respectively, to the stack top and the base address of <br>
//!     the virtual segment of <code>local, arguments, this, that</code>
//!   </td>
//! </tr>
//! <tr>
//!   <td><code>R13-R15</code></td>
//!   <td> These predefined symbols can be used for any purposes </td>
//! </tr>
//! <tr>
//!   <td> <code>Xxx.j</code> </td>
//!   <td>
//!     Each static variable <code>j</code> in a VM file <code>Xxx.vm</code> is translated into <code>Xxx.j</code>.
//!   </td>
//! </tr>
//! <tr>
//!   <td> <code>functionName$label</code> </td>
//!   <td>
//!     Each <code>label b</code> command in a VM function <code>f</code> should generate a global unique symbol
//!     "<code>f$b</code>" where "<code>f</code>" is the function name and "<code>b</code>" is the label symbol.
//!     When translating <code>goto b</code> and <code>if-goto b</code>, full label specification "<code>f$b</code>"
//!     must be used, instead of "<code>b</code>".
//!   </td>
//! </tr>
//! <tr>
//!   <td> <code>(FunctionName)</code> </td>
//!   <td>
//!     Each VM function <code>f</code> should generate a symbol "<code>f</code>" that refers to its entry point in
//!     instruction memory of target computer.
//!   </td>
//! </tr>
//! <tr>
//!   <td> <code>return-address</code> </td>
//!   <td>
//!     Each VM function call should generate and insert into the translated code stream a unique symbol that serves
//!     as a return address, namely the memory location, in the target platform's memory, of the command following
//!     the function call.
//!   </td>
//! </tr>
//! </tbody>
//! </table>
//!
//! ### Bootstrap Code
//!
//! Hack computer is designed that when it resets, it fetch and execute the word located in ROM\[0\]. Thus the code
//! segment that starts at ROM\[0\], called **bootstrap code** is the first thing that gets executed when computer
//! "boots up". To effect the initialization described earlier, Hack's bootstrap code shoud effect following operation
//! in machine language:
//! - `SP=256        // Initialize the stack pointer at 0x0100`
//! - `call Sys.init // Starting executing(the translated code of) Sys.init`
//!
//! `Sys.init` is then to call the main function of the main program and then enter an infinite loop. This action cause
//! the translated VM program to start running.
