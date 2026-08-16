/*
    1. BootLoad
*/

mod boot;


fn main() {
    println!("Mulberry OS");
    println!("For the Enchanted");

    boot::create_bootable_iso();
    boot::read_bootable_iso();
}
