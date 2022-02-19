use std::mem::{align_of, size_of};

pub fn memory_size_and_alignment() {
    println!("--------------------------------------------");
    println!(
        "     size of (char, u8, i32): {} bytes",
        size_of::<(char, u8, i32)>()
    );
    println!(
        "alignment of (char, u8, i32): {} bytes",
        align_of::<(char, u8, i32)>()
    );
    println!("~~~~~~~~~~~~~~~~~~~~~~");
    println!(
        "     size of (char, u8, i8): {} bytes",
        size_of::<(char, u8, i8)>()
    );
    println!(
        "alignment of (char, u8, i8): {} bytes",
        align_of::<(char, u8, i8)>()
    );
    println!("~~~~~~~~~~~~~~~~~~~~~~");
    println!(
        "     size of (char, u8, i16): {} bytes",
        size_of::<(char, u8, i16)>()
    );
    println!(
        "alignment of (char, u8, i16): {} bytes",
        align_of::<(char, u8, i16)>()
    );
    println!("~~~~~~~~~~~~~~~~~~~~~~");
    println!(
        "     size of (char, u8, i64): {} bytes",
        size_of::<(char, u8, i64)>()
    );
    println!(
        "alignment of (char, u8, i64): {} bytes",
        align_of::<(char, u8, i64)>()
    );
    println!("~~~~~~~~~~~~~~~~~~~~~~");
    println!(
        "     size of (char, u8, i128): {} bytes",
        size_of::<(char, u8, i128)>()
    );
    println!(
        "alignment of (char, u8, i128): {} bytes",
        align_of::<(char, u8, i128)>()
    );
    println!("--------------------------------------------");

    println!("~~~~~~~~~~~~~~~~~~~~~~");
    println!("     size of [u8; 3]: {} bytes", size_of::<[u8; 3]>());
    println!("alignment of [u8; 3]: {} bytes", align_of::<[u8; 3]>());
    println!("--------------------------------------------");

    println!("~~~~~~~~~~~~~~~~~~~~~~");
    println!("     size of char: {} bytes", size_of::<char>());
    println!("alignment of char: {} bytes", align_of::<char>());
    println!("--------------------------------------------");
}
