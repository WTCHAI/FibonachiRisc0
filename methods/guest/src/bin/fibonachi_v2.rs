use std::io::Read;

use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use serde_json::{Result as JsonResult, Value};

use alloy_primitives::U256;
use alloy_sol_types::SolValue;

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    times: U256,
    x: U256,
    y: U256,
}

fn main() {
    let mut payload_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut payload_bytes).unwrap();

    let filtered_bytes: Vec<u8> = payload_bytes
    .iter()
    .cloned()
    .filter(|&b| b != 0)
    .collect();

    let payload_result: JsonResult<Payload> = serde_json::from_slice(&filtered_bytes[1..]);

    let data = payload_result.unwrap();
    let result = fibonachi_logic(data.times, data.x, data.y);
    env::commit_slice(&result.abi_encode().as_slice());
}

fn fibonachi_logic(t: U256, mut x: U256, mut y: U256) -> U256 {
    if t.is_zero() {
        return x;
    } else if t == U256::from(1) {
        return y;
    } else {
        let mut i = U256::from(2);
        while i <= t {
            let z: U256 = x + y;
            x = y;
            y = z;
            i += U256::from(1);
        }
        return y;
    }
}