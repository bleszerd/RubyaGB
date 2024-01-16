#![allow(warnings)]

mod cpu;

fn main() {
    let a: u8 = 200;
    let b: u8 = 100;

    let result: u8 = a.wrapping_add(b);

    println!("{}", result);
}
