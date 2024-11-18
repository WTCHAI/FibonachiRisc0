use std::io::Read;

use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use serde_json::{Result as JsonResult, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    times: u32,
    x: u32,
    y: u32,
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

    match payload_result {
        Ok(payload) => {
            let result = fibonachi_logic(payload.times, payload.x, payload.y);
            env::commit(&result);
        }
        Err(e) => {
            println!("Error deserializing JSON payload: {:?}", e);
            let error_code = -1;
            env::commit(&error_code);
        }
    }
}

fn fibonachi_logic(t: u32, mut x: u32, mut y: u32) -> u32 {
    if t == 0 {
        return x;
    } else if t == 1 {
        return y;
    } else {
        for _ in 2..=t {
            let z: u32 = x + y;
            x = y;
            y = z;
        }
        return y;
    }
}