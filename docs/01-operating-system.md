# Operating Systems

it's a _system software_, manages _computer hardware_, to provide _commmon services_ to computer programs

OS acts as an intermediary for Hardware Functions:
    - Input and Output
    - Memory Allocation

an OS is an intermediary between the Program and the Hardware Basically.

[User] <-> [Program] <-> [**Operating System**] <-> [hardware]  

Operating Systems are found on anything that contains a computer:
    - Cellular Phones
    - Video Game Consoles
    - Web Servers
    - SuperComputers

Examples:
    - Android
    - Windows
    - IOS
    - MacOS
    - Linux

Too many Linux Distributions

Operating Systems really are versatile, especially linux variants, depending on the task at hand.

_"An operating system is difficult to define"_ Shit.

## Main Purposes of an Operating System:

1. Allocate Resources between different applications (CPU and Memory) while isolating them, while enabing comms between them.
2. Abstract the details of hardware access
3. Provide Common Services: Interface or accessing network and disk devices _"this functionality makes up the great majority of code for most operating systems"_

## Types of Operating Systems

MultiProcessors: Multiple CPUs have one memory
Multicomputer: Cluster of Computers, Each CPU has it's own Memory, developed because large multiprocessors are difficult to engineer.
Distributed Systems: Like the above but can be anywhere with their own OS and file system, combined with a middleware.
Embedded: can be 10kb, don't need much as they're just loaded into IoT or without internet access, no need for user installed software...
Real-Time: I'm not getting into this now, seems like a deep topic, out of scope
HyperVisor: Runs a Virtual machine, which emulates a hardware. you really can make the argument that minecraft can act as a hypervisor.
Library: I wish I could summarize this, but from initial read, it's basically base programs in the shape of libraries.

# Components:

## Kernel:

1. **Kernel** : 
    - Protection Between Different Applications and Users.
    - Isolate Errors to One Program.
    - Security and Protecting Private Data
    - Ensure non-moopoly over resources
    - user mode: legal instructions
    - kernel mode: unrestricted
    - manage memoy for other processes
    - i/o control access

2. **Program Execution**:
    - Executing a program is the creation of a *process* by the *kernel*
    - It Assigns memory space, and other resources (?) and establishes a priority for the process.
    - It loads the binary program into the memory
    - Initiates the execution of the application program
    - the Program Interacts with the user and the hardware.

3. **Interrupts**:
    - fault, abort, execption, signal, trap.
    - change the control flow from the currently running program to an interrupt handler (ISR: Interrupt Service Routine)
    - Common Interrupt Functions:
        + transfer control to an interrupt service routine.
        + save the state of the currently running process.
        + restore the state after the interrupt is serviced.

    _Read More On Software Interrupts and Hardware Interrupts_

4. **Signal**:
    - System Call: how a program requests a service from the operating system.
    - kill(pid, signum) syscall will send a _signal_ to another process
    - Signal Categories:
        + when a process finishes normally.
        + when a process has an error exception.
        + when a process runs out of a system resource.
        + when a process executes an illegal instruction.
        + when a process sets an alarm event.
        + when a process is aborted from the keyboard.
        + when a process has a tracing alert for debugging.

5. **Input/Output** :
    - I'm not going to get into Device Drivers now.

6. **Direct Memory Access** : 
    - HDDs, SSDs, and Magnetic Tape Drives transfer data so fast that interrupting would be inoptimal.
    - Data is then transfered between the device and the memory independantly from the CPU using _channels_ or _direct memory access controllers_, 
    - Interrupts are delivered hen the data is transferred.

    - Example: Block I/O Write Operation
        (PCB is the process control block: Data Structure that Stores all The info on a process)
        + set content of the CPU register into the PCB
        + create an entry to the device_status table (one field is the mem address of the PCB)
        + place all the characters to be sent to the device into a memory buffer (just say RAM)
        + set the mem address to the memory buffer to a predetermined device register. ('basically nonesense to me from here on forward')
        + set the buffer size (int) to another predetermined register

        *_while looking up references for the register keyword, I discovered a book called Structure Computer Organisation_*
        *_To those who know, yeah, I am happy_*
        *_To those who don't, Digging for Copper, I struck Mythrill_*
        *_I'll leave it for when I need it_*
        + 

        + Execute Machine Instruction to begin the writing
        + Perform a context switch to the next process in the ready queue?

7. **Memory Management**:

    - each program must have independant access to memory
    - Cooperative memory management: _This system of memory management is almost never seen anymore,_
    - Memory Protection's Various Methods    
        + Memory Segementation: divide memory into segments, so the reference to a memory location has the segment and an offset which is the mem location. 
        + Paging: Fixed-length contiguous block of virtual memory
    - Attempts to access register-protected addresses will trigger an itnerrupt, which will cause the CPU to enter SuperVisor Mode, Kernel Takes the Charge
        => this is called a Seg-V or Segmentation Violation or Seg-Fault.
        => the kernel generally resorts to terminating the offending program, and reports the error.

8. **Virtual Memory**  I won't look into now

## File System

    - permanent storage is much cheaper per byte.
    - longer to acces, read or write
    - file systems are an abstraction of accessing this sort of storage
    - readable filennames, metadata, increase performance, prevent multi-threads from accessing the same section, include checksomes to identify corruption.
    - made up of either directories or files
    - Absolute Paths are made up from the root up til the requested sub-directory
    - Relative Paths are made up from the location of a file from a directory

    - We use System Calls to Create, Edit, Show, Close or Delete files.
    - We also use cache to reduce latency (don't ask me where the cache is)

    - File writing protocols are designed with atomic operations, so they won't leave storage partially, or in an inconsistent state in the event of a crash during writing.
    - Data corruption is addressed by redundant storage: RAID and Checksums
    - Background processes are often used to detect and recover from data corruption.

## Networking

    - Modern OS include a networking stack such as TCP/IP protocol stack

## Security

---to be added---

## Concurrency

---to be added---

## User Interface

Two Types: **Comand Line Interface** or **Graphical User Interface** (WIMP)


---



