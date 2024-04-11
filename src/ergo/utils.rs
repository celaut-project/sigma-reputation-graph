use std::str;
use ergo_lib::ergotree_ir::mir::value::NativeColl;
use ergo_lib::ergotree_ir::types::stype::SType;
use ergo_lib::ergotree_ir::mir::constant::{Constant, Literal};
use ergo_lib::ergotree_ir::mir::value::CollKind;

use crate::ergo::decoder::string_to_bytes;

use super::decoder::CoderError;


pub fn generate_pk_proposition(wallet_pl: &str) -> &str {
    // todo
    wallet_pl
}

// Convert a hex string to a UTF-8 string
fn hex_to_utf8(hex_string: &str) -> Result<String, std::str::Utf8Error> {
    if hex_string.len() % 2 != 0 {
        eprintln!("The hexadecimal string has an odd length.");
        panic!()
    }

    let bytes = hex::decode(hex_string).expect("Decoding failed");
    let utf8_string = str::from_utf8(&bytes)?;

    Ok(utf8_string.to_string())
}

// Serialize a string to a format suitable for rendering
fn string_to_serialized(value: &str) -> Result<String, CoderError> {
    // On TS with fleet is --> SConstant(SColl(SByte, stringToBytes('utf8', value)));

    // let decoded = string_to_bytes('utf8', value);
    match string_to_bytes(crate::ergo::decoder::CoderType::Base64, value) {
        Ok(value) => Ok(Constant {
            tpe: SType::SColl(Box::new(SType::SByte)),
            v: Literal::Coll(CollKind::NativeColl(NativeColl::CollByte(value)))
        }),
        Err(err) => Err(err)
    };
    unimplemented!()
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
pub fn string_to_rendered(value: &str) -> Result<String, CoderError> {
    match string_to_serialized(value) {
        Ok(value) => Ok(serialized_to_rendered(&value)),
        Err(err) => Err(err)
    }
}