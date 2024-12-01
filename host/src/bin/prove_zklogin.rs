use risc0_zkvm::{default_prover, ExecutorEnv};
use zk_methods::ZKLOGIN_ELF;

fn main() {
    println!("this is prover zklogin");
    let a: String = String::from("this is jwt");
    let b: String = String::from("jwt");

    let env = ExecutorEnv::builder()
        .write(&a).unwrap()
        .write(&b).unwrap()
        .build()
        .unwrap();

    // 获取 prover 并生成证明
    let prover = default_prover();
    let receipt = prover.prove(env, ZKLOGIN_ELF).unwrap();
    // 获取计算结果
    let _output: u32 = receipt.receipt.journal.decode().unwrap();
    let serialized = bincode::serialize(&receipt.receipt).unwrap();

    // Writing the serialized contect to receipt.bin file
    let _saved_file = match std::fs::write("./receipt_zklogin.bin", serialized) {
        Ok(()) => println!("Receipt saved and serialized as receipt.bin"),
        Err(_) => println!("Something went wrong !!"),

    };
    // 验证证明
    // receipt.receipt.verify(MULTIPLY_ID).unwrap();
}
