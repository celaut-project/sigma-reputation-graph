use std::{str, vec};
use anyhow::Ok;
use ergo_lib::ergotree_ir::base16_str::Base16Str;
use crate::ergo::decoder::CoderType::Base64;
use ergo_lib::ergotree_ir::mir::value::NativeColl;
use ergo_lib::ergotree_ir::types::stype::SType;
use ergo_lib::ergotree_ir::mir::constant::{Constant, Literal};
use ergo_lib::ergotree_ir::mir::value::CollKind;

use crate::ergo::decoder::string_to_bytes;


pub fn generate_pk_proposition(wallet_pl: &str) -> &str {
    unimplemented!()
}

// Convert a hex string to a UTF-8 string
fn hex_to_utf8(hex_string: &str) -> Result<String, anyhow::Error> {
    if hex_string.len() % 2 != 0 {
        eprintln!("The hexadecimal string has an odd length.");
        panic!()
    }

    let bytes = hex::decode(hex_string).expect("Decoding failed");
    let utf8_string = str::from_utf8(&bytes)?;

    Ok(utf8_string.to_string())
}

// Serialize a string to a format suitable for rendering
fn string_to_serialized(value: &str) -> anyhow::Result<String> {
    let bytes = string_to_bytes(Base64, value)?;

    let constant = Constant {
        tpe: SType::SColl(Box::new(SType::SByte)),
        v: Literal::Coll(CollKind::NativeColl(NativeColl::CollByte(bytes)))
    };

    let base16_encoded = constant.base16_str().map_err(anyhow::Error::from)?;

    Ok(base16_encoded)
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
pub fn string_to_rendered(value: &str) -> anyhow::Result<String> {
    Ok(serialized_to_rendered(&string_to_serialized(value)?))
}