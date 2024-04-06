use std::str;


pub fn generate_pk_proposition(wallet_pl: &str) -> &str {
    // todo
    wallet_pl
}

// Convert a hex string to a UTF-8 string
pub fn hex_to_utf8(hex_string: &str) -> Result<String, std::str::Utf8Error> {
    if hex_string.len() % 2 != 0 {
        eprintln!("La cadena hexadecimal tiene una longitud impar");
        panic!()
    }

    let bytes = hex::decode(hex_string).expect("Decoding failed");
    let utf8_string = str::from_utf8(&bytes)?;

    Ok(utf8_string.to_string())
}

// Serialize a string to a format suitable for rendering
pub fn string_to_serialized(value: &str) -> String {
    // Assuming `string_to_bytes` is a function you will define to convert a string to bytes
    // and `SConstant` and `SColl` are types or functions you need to define or import.
    // let bytes = string_to_bytes("utf8", value);
    // SConstant(SColl(SByte, bytes))
    unimplemented!() // You need to implement this based on your actual types and logic
}

// Convert a serialized value to a rendered string
pub fn serialized_to_rendered(serialized_value: &str) -> String {
    let pattern_to_strip = "0e";
    if serialized_value.starts_with(pattern_to_strip) {
        serialized_value[pattern_to_strip.len()..].to_string()
    } else {
        serialized_value.to_string() // or handle this case as needed
    }
}

// Convert a string to a rendered string
pub fn string_to_rendered(value: &str) -> String {
    serialized_to_rendered(&string_to_serialized(value))
}