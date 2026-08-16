# Bootloader

[Rust Docs](https://docs.rs/hadris-iso/latest/hadris_iso/)

While looking for a way to actually start development instead of only reading history, 
I learned that the first thing I should do is create an ISO file 
I needed to write a boot loader, so I can have something to work on
I am aware this is too premature, but I want to atleast work on something

_A pure Rust ISO 9660 filesystem and disk-image library_

the thing about this is we won't actually be writing our own ISO 9660 handler, this is basically using an existing ISO creator...
are we okay with this?

## pros:

- Faster to get up and running with the bootloader
- It's working as per the example

## Cons

- We don't really touch on ISO files creating and reading on bare metal 
- A little bit too much Abstraction than I'm comfortable with
- I want to understand more about CPU, Memory and such


If we take some time to think about this:
we'll be writing the ISO file creator and reader... but not the bootloader. 

So I guess we're going to either:
A. write a ISO file creator and reader
B. use this one...

my gut tells me to write an ISO file creator and reader. however shitty it may be.

now how do we find documentation for that?

---

[This might be a little bit cheating](https://wiki.osdev.org/Rolling_Your_Own_Bootloader)
it's a wiki
it doesn't really give you a step by step guide
_The next step consist..._ AHHHHH, I can't use step by step guides
this honestly feels like trying to create the starter of an engine, when you don't know what goes into it.
but I have to start somewhere, as currently, I feel like I'm starting with the history of the universe, when really I don't need much
I know at some point I'm going to have to study the ins and outs of CPU and Memory, but I don't know why, so I need to figure that out first.

---

[Theory of a Bootloader](https://wiki.osdev.org/Bootloader)
a wiki
more theory than how to.
better ig...
---

## What does a bootloader do?

- Bring the kernel into memory
- Provide the kernel with the info it needs
- Switch to an environment the kernel will like (I guess the ISO 9660)
- Transfer all the control to the kernel

_that's more like it!_

From my reading of a lot of general wiki pages and github docs, it seems what we did earlier with hadris IO's example where we booted up the ISO was what we need.
when a PC sends a power on signal, we basically use the first 512 bytes/ first sector for the hard-disk to run the bootloader which will try to load the kernel which is the bigger part of the system.

Now, Normally this is done using assembly, but we don't know assembly and we want to code this project in Rust...

[QEMU documentation](https://www.qemu.org/docs/master/system/introduction.html)

this is the virtual machine we're going to use...
I guess we need to define what kind of machine we're going to use.

_so far, my understanding is that I need to create a small virtual machine with certain specifications, that takes my ISO file, and runs the bootloader first, that runs the kernel._

after reading the [QEMU commands manual](https://www.qemu.org/docs/master/system/invocation.html#hxtool-0) I guess we can specify any type of machine we want with any specs.
we can also create an ISO file using QEMU...



what's next?

**we need to write the bootloader.**

there's too many tutorials on how to?
especially in Assembly?
but I'm not allowed to copy-paste from them

[kolibrios](https://wiki.kolibrios.org/wiki/For_developers/en) _all in russian_

from reading more and looking a bit more... this seems way more complicated than it should, I might be skimming a bit too hard...

...

and after reading everything, especially [GRUB bootloader documentation](https://www.gnu.org/software/grub/index.html). I've come to realize we're not going to need to write a bootloader.
because QEMU already has one, and that's what it does when wwe start, goes by drives and such...

to write an actual bootloader, would be useless because we won't be using it.

Do I need to write a bootloader? not necessarily.
Do I feel satisfied by all that I read today? more than you think.

**TODO: Write a bootloader in ASM.**

The goal of this project is to write an operating system in Rust

I guess for now we'll use hadris, but we'll get back to this once we have much more knowledge on the internal of computer systems.

_hold on_

```Rust
 // Create a simple boot image (normally this would be a real bootloader)
    // This is just a placeholder that does an infinite loop (jmp $)
    let mut boot_image = vec![0u8; 2048];
    boot_image[0] = 0xEB; // jmp
    boot_image[1] = 0xFE; // -2 (infinite loop)

    // Create some additional files for the ISO
    let readme_content = b"This is a bootable ISO created with hadris-iso!\n";
```

I guess we're going to need to write one after all.
fuck it, we're looking for more.

[systemd-boot](https://www.freedesktop.org/software/systemd/man/latest/systemd-boot.html)
[UEFI Design Overview](https://uefi.org/specs/UEFI/2.10/01_Introduction.html#uefi-design-overview)

Holy mother ball;
actually it's a little too complicated, 
I'll go with BIOS instead of UEFI

## BIOS Design Overview

[ROM BIOS](https://www.reenigne.org/crtc/PC-XT.pdf)
[How BIOS Works](https://flint.cs.yale.edu/feng/cos/resources/BIOS/)

Every computer with a motherboard includes a special chip referred to as the BIOS or ROM BIOS (Read Only Memory Basic Input/Output System). The BIOS includes instructions on how to load basic computer hardware

the "Basic Input Output System" **resides in ROM**on the system board.
provides device level control of the major I/O devices n the system.
services such as: "Time of Day" or "Memory Size Determination" are provided by the BIOS

the BIOS interface insulates the user from the hardware.

BIOS has 4 main functions:

    - POST: check if hardware is functioning properly
    - Bootstrap Loader: Process of locating the operating system, then pass control to it
    - BIOS: Software/Drivers which interface between the OS and the hardware
    - CMOS: config program: computer passwords, time, and date.

...
from reddit:

_Ok, being serious, this falls in the territory of "technically anything is possible." Yes, you can probably create your own BIOS but it's going to be an enormous and extremely complex project._

I guess screw it for now we'll write a bootloader later when we need it
for now we have one.