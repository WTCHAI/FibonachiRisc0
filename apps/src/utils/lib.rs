pub fn convertImageToU8(vector32: [u32;8]) -> [u8;32] {
    let mut result : [u8;32] = [0u8;32] ; 

    for (i, &value) in vector32.iter().enumerate() {
        let bytes = value.to_le_bytes(); // Convert each u32 to 4 bytes
        let start = i * 4;
        result[start..start + 4].copy_from_slice(&bytes);
    }


    result
}