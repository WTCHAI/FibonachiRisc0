
use risc0_zkvm::guest::env;

use serde::Deserialize;  // Import Deserialize to read structs
#[derive(Deserialize)]
// Declare struct to use as input of guest program from env which set at host side 
struct Payload{
    times : u32,
    x : u32, 
    y : u32,
}

fn main() {

    let payload : Payload = env::read() ;     
    const U32_MAX: u32 = u32::max_value() ; 

    let result : u32 = fibonachi_logic(
        payload.times,
        payload.x,
        payload.y,
        U32_MAX
    ) ;
    env::commit(&result) ; 
}


fn fibonachi_logic(t : u32, mut  x : u32, mut  y : u32, max :u32) -> u32  {
    if x > max || y > max {
        panic!("Initial values exceed u32::MAX!");
    }
    if t == 0  {
        return x ; 
    }else if  t == 1  { 
        return y ; 
    }
    else { 
        for _ in 2..=t { 
            let z: u32  = x + y ; 
            x = y ;
            y = z  ;
        }
        return y  ; 
    }
    // OR recursively
    // fibonachi_logic(n-1 , y, y+x)
}