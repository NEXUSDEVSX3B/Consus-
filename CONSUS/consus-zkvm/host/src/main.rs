use sp1_core::SP1Prover;
use guest::main;

fn main() {
    let mut prover = SP1Prover::new();
    prover.write(&100u32);
    prover.write(&42u32);

    let (proof, output) = prover.prove(main);
    let result: u32 = output[0];
    println!("Proved result: {}", result);
    proof.verify(main);
}
