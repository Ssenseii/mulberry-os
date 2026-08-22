# Loading a Kernel

[Writing a Bootloader from Scratch](https://www.cs.cmu.edu/~410-s07/p4/p4-boot.pdf)

we're going to have to do a two-stage bootloader.
and we're going to have to enter protected mode.

_now I understand why we jump to the end, it starts from right to left..._

physical memory map:

|--- 0xFFFFFFF
|   
|   User Memory (protected free memory)   
|   
|--- 0x1000000
|--- 0xffffff
|   
|   Kernel Memory (protected free memory)   
|   
|--- 0x100000

- BIOS
- Memory Mapped IO
- Video BIOS
- Video Memory
- Extended BIOS Data
- Bootloader Memory
- BIOS data 
- Interrupt Vector table

_In the 5th title it does say step by step, but I can't do this with just wikipedia_


**Boot0**: 
- Disable Interrupts
- Canonicalize %CS:%EIP
- Load Segment Registers
- Set the Stack Pointer
- Enable Interrupts
- Reset the FD controller
- read boot1 sectors from the floppy disk
- jump to boot1 code

**boot1:**
- set the stack pointer
- query the bios for the size of lower memory
- query the bios for the size of upper memory
- read kernel sectors from the floppy into lower memory
- enable the A20 gate
- disable interrupts
- load the global descriptor table
- switch to protected mode
- invoke the multi-boot loader
- begin execution of kernel

**Multiboot Loader**
The Multiboot loader must accomplish the following tasks:
1. Locate the Multiboot header in the preloaded kernel image.
2. Verify the Multiboot header & flags.
3. Load the kernel image into high memory.
4. Write the Multiboot information structure.
5. Return success or failure to boot1


...

I'm not here to learn asm...
I guess I'm going to stop unless i want to learn asm.