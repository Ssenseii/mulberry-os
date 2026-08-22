# Assembly Language

By this point, you can create a bootable ISO.
next up, we need to understand more assembly language to understand how to setup more sectors and load more programs especially the kernel.

[Assembly Language](https://pacman128.github.io/static/pcasm-book.pdf)

Dec: 234 = 2 * 10² + 3 * 10^1 + 4 * 10^0
Bin: 11001 = 1 * 2^4 + 1 * 2^3 + 0 * 2^2 + 0 * 2^1 + 1 * 2^0
           = 16 + 8 + 1
           = 25 

**Memory**

basic unit: byte

_byte: 8 bits, can hold a single char, or a number from 0 to 255, or -128 to 127_
_bits: can hold 1 or 0_

32mb : 32 million bytes.

address:    0   1   2   ...
Memory:     2A  45  B8  ...

Diff between ascii and unicode:
ascii uses one byte
unicode uses more than one byte sometimes

example: 
- ASCII maps byte 41(16) to capital A.
- UTF-16 maps it to 0041(16)

**CPU**

This is what performs the instructions.
CPU has registers.

_you must always keep only currently used data in registers._

each cpu has it's own machine language.
each computer has an internal clock, each clock pulses at a specific speed.

a 1.5 GHz computer, means = 1.5 GHz frequency of the clock.
it doesn't track minutes and seconds, just pulses.

_IBM 80*86 machines_

- 8088, 8086: earliest pc cpus, 16 bit registers, and they only support 1MB of memory, operate in real mode. segments can't be larger than 64KB.
- 80286     : 16-bit protected mode added. supports 16MB of memory, protect access between programs.
- 80386     : registers now hold 32 bits, two new 16 bit registers, 32 bit protected mode, 4GB of memory access, and segments can't be larger than 4GB.   
- Pentium   : speed up
- Pent. MMX : Multi-Media Extensions for the graphical operations
...

*16-bit registers*


- AX
- BX 
- CX
- DX

all four can be decomposed into 8-bit registers, AH and AL.

- SI
- DI

often used as pointers, can't be decomposed, can be used generally.

- BP
- SP

Base pointer and Stack Pointer, for the machine language stack.

- CS: Code Segment 
- DS: Data Segment
- SS: Stack Segment
- SD: Extra Segment

denote what memory is used for different parts of the program.

- IP: Instruction poiner

used with CS to keep track of the addr of the next instruction

- FLAGS

stores the results of a previous instruction, especially conditions

_Real Mode:_

2MB only.
from 00000 to FFFFF which can result 20 bits...
how do we store the address that's 20 bit in 16 bit registers.
we use the two reg.
the first register is called the selector,
the second register is called the offset,

the physical address referenced by the selector:offset is **16 * selector + offset.**

**multiplying by 16 in hex is easy, just add 0 to the right of the number**

047C:0048 => apply formula => 047C0 + 0048 = 04808.


## Assembly Language Basics

_mnemonic operand (s)_
add eax, ebx

operands can either be:
- register: contents of the CPU
- memory: data in memory
- immediate: contents of the instruction itself
- implied: increment for example adds one without ever writing it.

MOV moves data from source to desitnation

```assembly
mov dest, src
mov eax, 3          ; store 3 into the eax register
```

ADD SUB, self explained
INC, DEC same thing

```assembly
add eax, 4          ; eax = eax + 4 
sub al, ah          ; al = al - ah

inc ecx             ; ecx++
dec ecx             ; ecx--
```







