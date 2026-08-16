# How to go about this now?

# 

Obviously we don't know anything about writing operating systems. 
but we do have some understanding of how they work now based on the document before this one
If we're going to write our own OS, We'll probably need some inspiration.

the goal is to write a working operating system from scratch, not based on another. Yet, I'm very tempted to build a UNIX based Operating System.

---

I guess the goal of this file is to see how each OS manufacturer built theirs, to find out how to build mine.

Oh, a few small notes, since there's like hundereds, 
I'm going to keep it constrained to Personal Computer Popular Operating System which are written in C or C++

--- 

I looked in the github repo of Serenity OS... 
[File System of Serenity OS](https://github.com/SerenityOS/serenity/tree/master/Kernel/FileSystem)
We're not going to use this codebase as reference, I just wanted to see what the structure is like.

---

## Sources:

[Unix-like operating system based on the BSD](https://github.com/NetBSD/src)
[Unix-like operating system based on the BSD](https://github.com/openbsd/src)
[Unix-like operating system based on the BSD](https://github.com/freebsd/freebsd-src)
[Linux kernel](https://github.com/torvalds/linux)
[BeOS inspired OS](https://github.com/haiku/haiku)
[The XNU kernel source code for use in MacOS and iOS](https://github.com/apple/darwin-xnu)

_thank you for these resources: [exajobs](https://github.com/exajobs/os-collection#popular-operating-system)_
_It's a shame I couldn't use the rest of the collection for now, I'll get back to it someday_



---

# Unix Operating Systems

[unix](https://en.wikipedia.org/wiki/Unix)

We'll dive directly into the components, to have a global understanding of the structure so we have an idea of what we look at in github:

## V7 Structure:

1. Kernel: Source code in /usr/sys
    + conf: confiuration of the machine parts, including boot code.
    + dev: device drivers
    + sys: operating system kernel: memory management, process scheduling and sys calls.
    + h: header files

2. Development Environment: 
    + ed: text editor
    + cc: C language Compiler
    + as: machine-language assembler
    + ld: linker (?)
    + lib: primary library for functionalities, like meth and stdio
    + make: build manager
    + include: header files for software development
    + ...

3. Commands:
    + sh: the shell interpreter, primary user interface.
    + utilities: cp, ls, grep, find (System and User)
    + Document Formatting
    + Graphics: X11 (X Window System), wayland, quartz (MacOS), OpenGL, Vulkan

4. Documentation:
    + man: manual pages
    + doc: longer docs, for like the C language or such


_Doesn't really have a standard structure here, it does show a list of books but we'll start digging now in the repos_
_I just realized my github repos are for unix-like which are not exactly Unix_


