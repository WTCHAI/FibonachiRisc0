use risc0_zkvm::guest::env;

use serde::Deserialize;  // Import Deserialize to read structs
#[derive(Deserialize)]
// Declare struct to use as input of guest program from env which set at host side 
struct Payload{
    times : u64,
    x : u64, 
    y : u64,
}

fn main() {
    let payload : Payload = env::read() ;     
    let result : u64 = fibonachi_logic(payload.times, payload.x,payload.y ) ;
    env::commit(&result);
}

fn fibonachi_logic(t : u64, mut  x : u64, mut  y : u64) -> u64  {
    if t == 0  { 
        return x ; 
    }else if  t == 1  { 
        return y ; 
    }
    // Consider doing loop way to find fibonachi 
    else { 
        for _ in 2..=t { 
            let z: u64  = x + y ; 
            x = y ;
            y = z  ;
        }
        return y  ; 
    }
    // OR recursively
    // fibonachi_logic(n-1 , y, y+x)
}