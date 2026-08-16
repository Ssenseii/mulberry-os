# Problems Breakdown

- How to build an operating system?
    - How to build a bootable ISO file using Rust.
        - What is an ISO: [optical_disk_image](https://en.wikipedia.org/wiki/Optical_disc_image)
        - How to build an ISO in rust? [hadris_iso](https://docs.rs/hadris-iso/latest/hadris_iso/)
        - How to create a small virtual machine to run our own OS? [QEMU](https://www.qemu.org/docs/master/system/introduction.html)

so far, my understanding is that I need to create a small virtual machine with certain specifications, that takes my ISO file, and runs the bootloader first, that runs the kernel.

- How to write a boot system?
    - How does a bootloader operate?
        - How does UEFI operate?
            - How does BIOS operate?