#![no_main]

// use std::error::Error;
// #![no_std]
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

risc0_zkvm::guest::entry!(main);

#[derive(Debug, Serialize, Deserialize)]
struct GoogleClaims {
    // 必须字段
    iss: String,         // 发行者 (应该是 "https://accounts.google.com")
    sub: String,         // Subject (用户ID)
    aud: String,         // 目标受众 (你的 Google Client ID)
    iat: u64,           // 发行时间
    exp: u64,           // 过期时间
}
//

fn main() {
    // 私密输入 a (要搜索的主字符串)
    println!("this is guest zklogin");
    let jwt: String = env::read();
    // 公共输入 b (要查找的子字符串)
    let iss: String = env::read();
    let sub: String = env::read();
    let aud: String = env::read();
    let exp: u64 = env::read();
    let iat: u64 = env::read();

    // 反序列化 JWT
    let claims = match serde_json::from_str::<GoogleClaims>(&jwt) {
        Ok(claims) => claims,
        Err(e) => {
            println!("Deserialization error: {}", e);
            panic!("Failed to deserialize JWT");
        }
    };

    if claims.iss != iss {
        panic!("iss mismatch");
    }

    if claims.sub != sub {
        panic!("sub mismatch");
    }

    if claims.aud != aud {
        panic!("aud mismatch");
    }

    if claims.exp != exp {
        panic!("exp mismatch");
    }

    if claims.iat != iat {
        panic!("iat mismatch");
    }

    env::commit(&(iss, iat, exp));
}
