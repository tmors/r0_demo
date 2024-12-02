use std::time::Instant;
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::Serialize;
use zk_methods::ZKLOGIN_ELF;

fn main() {
    println!("this is prover zklogin");
    let jwt: &str = "eyJhbGciOiJSUzI1NiJ9.eyJleHAiOjE5MzI4NDkyNjgsImlhdCI6MTczMjg0OTI2OCwic3ViIjoic3ViIiwiYXVkIjoiYXVkIiwiaXNzIjoiaXNzIiwiZW1haWwiOiJlbWFpbCIsIm5hbWUiOiJuYW1lIn0.TxOEkjetS1rbMzFKYDtL1Xlh0vjVYgh09rTlAj1jZpXUyTLSXShVSVpLgi487i0Ku9aMAgttfNd4zsSg7EAZzVMuEwrWxWXjhhgocLRYzrC2A7lTzvPWcprYQAx1ac1z865YmQfwOzvUEmcm1vStP0LvaK_x24ytOwUjLDtt8nuVhg3djQuC5LkkEXIMoiLj0XnX5sYgwJALKQ21gRpmTekowLi5KnXOcgXG4xDRhBicTFdUxUwR-I72FT83VPkZb8Lm2JgF2B25H8A-wWGOK_rM2QNHAHTgZC7jBTSR8zEGM1AtTRlg_zPcVCjU0xDKBPyOhJuoVaoLveCpIrGEEAjUD7QHIu8jSzyguF_1_XI-zkuw_g4LaPM4lPeTK0h21IYZRBxg4_KIdo8OuqmSCV4brU4Sqj6sgwxY-1eaX0HlEwWV5DnjjVZH9BkbN2m3yMPH9dBHZX-BJskFfRMuQA9RjNMxCDK16K8v457Vo2ul48hujTprgiy20gzB9V4K";

    let pubkey: &str = r#"
    {
      "alg": "RS256",
      "e": "AQAB",
      "key_ops": [
        "verify"
      ],
      "kty": "RSA",
      "n": "zcQwXx3EevOSkfH0VSWqtfmWTL4c2oIzW6u83qKO1W7XjLgTqpryL5vNCaxbVTkpU-GZctit0n6kj570tfny_sy6pb2q9wlvFBmDVyD-nL5oNjP5s3qEfvy15Bl9vMGFf3zycqMaVg_7VRVwK5d8QzpnVC0AGT10QdHnyGCadfPJqazTuVRp1f3ecK7bg7596sgVb8d9Wpaz2XPykQPfphsEb40vcp1tPN95-eRCgA24PwfUaKYHQQFMEQY_atJWbffyJ91zsBRy8fEQdfuQVZIRVQgO7FTsmLmQAHxR1dl2jP8B6zonWmtqWoMHoZfa-kmTPB4wNHa8EaLvtQ1060qYFmQWWumfNFnG7HNq2gTHt1cN1HCwstRGIaU_ZHubM_FKH_gLfJPKNW0KWML9mQQzf4AVov0Yfvk89WxY8ilSRx6KodJuIKKqwVh_58PJPLmBqszEfkTjtyxPwP8X8xRXfSz-vTU6vESCk3O6TRknoJkC2BJZ_ONQ0U5dxLcx",
      "use": "sig",
      "kid": "6ab0e8e4bc121fc287e35d3e5e0efb8a"
    }
    "#;

    // let pubkey : &str = "";

    let env = ExecutorEnv::builder()
        .write(&jwt).unwrap()
        .write(&pubkey).unwrap()
        .build()
        .unwrap();

    // 获取 prover 并生成证明
    let prover = default_prover();
    let receipt = prover.prove(env, ZKLOGIN_ELF).unwrap();
    // 获取计算结果
    let serialized = bincode::serialize(&receipt.receipt).unwrap();

    // Writing the serialized contect to receipt.bin file
    let _saved_file = match std::fs::write("./receipt_zklogin.bin", serialized) {
        Ok(()) => println!("Receipt saved and serialized as receipt.bin"),
        Err(_) => println!("Something went wrong !!"),
    };
}
