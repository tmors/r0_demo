// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use risc0_zkvm::{default_prover, ExecutorEnv};
use zk_methods::MULTIPLY_ID;
use risc0_zkvm::Receipt;

fn main() {
    println!("this is verify multiply");

    // Let's impor the receipt that was generated by prove
    let receipt_path ="./receipt_multiply.bin".to_string();
    let receipt_file = std::fs::read(receipt_path).unwrap();

    // As we has serialized the receipt we need to desrialize it
    let receipt = bincode::deserialize::<Receipt>(&receipt_file).unwrap();

	// Let's verify if the receipt that was generated was not created tampered with
    let _verification = match receipt.verify(MULTIPLY_ID){
        Ok(()) => println!("Proof is Valid"),
        Err(_) => println!("Something went wrong !!"),
    };
}
