// only-x86_64

#![feature(asm, never_type)]
#[allow(dead_code)]

fn main() {
    unsafe {
        // Type checks ignore never type
        // But type annotations are needed
        // See also type-check-2.rs

        let u: ! = unreachable!();
        asm!("{}", in(reg) u); //~ ERROR: type annotations needed
    }
}
