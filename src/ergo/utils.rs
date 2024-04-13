use bs58;
use std::str::from_utf8;
use ergo_lib::ergotree_ir::base16_str::Base16Str;
use ergo_lib::ergotree_ir::chain::address::{AddressEncoder, NetworkPrefix};
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;
use crate::ergo::decoder::CoderType::Base64;
use ergo_lib::ergotree_ir::mir::value::NativeColl;
use ergo_lib::ergotree_ir::types::stype::SType;
use ergo_lib::ergotree_ir::mir::constant::{Constant, Literal};
use ergo_lib::ergotree_ir::mir::value::CollKind;

use crate::ergo::decoder::string_to_bytes;


pub fn generate_pk_proposition(base58_wallet_pk: &str) -> Result<String, anyhow::Error> {

    /**
     * 
    const pk = ErgoAddress.fromBase58(wallet_pk).getPublicKeys()[0];
    const encodedProp = SGroupElement(pk);
    return encodedProp.toHex();
     */

    let network_prefix = NetworkPrefix::Mainnet;
    let encoder = AddressEncoder::new(network_prefix);
    let pk = encoder.parse_address_from_str(base58_wallet_pk)?;
    let script = pk.script()?;
    
    let serialized = script.sigma_serialize_bytes()?;
    let serialized2 = serialized.clone();

    let bs58_str: String = bs58::encode(serialized).into_string();

    let hex_string: String = serialized2.iter().map(|byte| format!("{:02x}", byte)).collect();
    
    Ok(bs58_str)
}

// Convert a hex string to a UTF-8 string
fn hex_to_utf8(hex_string: &str) -> Result<String, anyhow::Error> {
    if hex_string.len() % 2 != 0 {
        eprintln!("The hexadecimal string has an odd length.");
        panic!()
    }

    let bytes = hex::decode(hex_string).expect("Decoding failed");
    let utf8_string = from_utf8(&bytes)?;

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
    let result = if serialized_value.starts_with(pattern_to_strip) {
        // Remove the pattern
        &serialized_value[pattern_to_strip.len()..]
    } else {
        serialized_value
    };
    let chars_to_skip = 2;
    result.chars().skip(chars_to_skip).collect()
}

// Convert a string to a rendered string
pub fn string_to_rendered(value: &str) -> anyhow::Result<String> {
    Ok(serialized_to_rendered(&string_to_serialized(value)?))
}