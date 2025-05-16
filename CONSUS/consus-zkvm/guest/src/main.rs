#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let a: u32 = sp1_zkvm::io::read();
    let b: u32 = sp1_zkvm::io::read();
    let result = a + b;
    sp1_zkvm::io::write(&result);
}
