# Memory Storage in the Simulator

## Chip behavior and Rust storage

Whether Rust stores a chip's data on the stack or heap does not change the chip's simulated behavior. Address decoding, reads, writes, and the RAM hierarchy determine what the chip does; the storage location is an implementation detail of the simulator.

An array is stored inline wherever its containing value is stored. For example, a small `[Bit; 8]` local variable is usually on the stack, while an array inside a heap-allocated `Vec` element is on the heap. A `Vec<T>` itself is small; its elements are stored in a heap allocation.

## Why the simulator uses heap-backed storage

Large memories contain many nested values. With one byte per `bool`, their data alone is approximately:

- RAM16k: 256 KiB
- Screen: 128 KiB
- ROM32k: 512 KiB

Constructing these memories can also require temporary space. Keeping all of the large arrays inline can exceed the limited stack available to a test thread and cause a stack overflow.

The simulator therefore uses `Vec` for the large backing stores in `RAM16k`, `Screen`, and `ROM32k`. Each is initialized to its required size and its storage is private, so callers cannot change the modeled chip capacity. This avoids large stack allocations without changing the chips' behavior or the RAM bank hierarchy.
