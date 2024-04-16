use std::fmt;
use thiserror::Error;
use ergo_lib::ergotree_ir::base16_str::Base16Str;
use ergo_lib::ergotree_ir::chain::address::{AddressEncoder, NetworkPrefix};
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;
use ergo_lib::ergotree_ir::mir::value::NativeColl;
use ergo_lib::ergotree_ir::types::stype::SType;
use ergo_lib::ergotree_ir::mir::constant::{Constant, Literal};
use ergo_lib::ergotree_ir::mir::value::CollKind;
use ergo_lib::ergotree_ir::serialization::SigmaSerializationError;
use ergo_lib::ergotree_ir::chain::address::AddressEncoderError;
use ergo_lib::ergotree_ir::serialization::SigmaParsingError;
use crate::ergo::decoder::vec_u8_to_vec_i8;

#[derive(Debug, Error)]
pub enum UtilError {
    #[error("Input too short")]
    InputTooShort,

    #[error("Sigma serialization error")]
    SigmaSerializationError(#[from] SigmaSerializationError),

    #[error("Sigma parsing error")]
    SigmaParsingError(#[from] SigmaParsingError),

    #[error("Address encoder error")]
    AddressEncoderError(#[from] AddressEncoderError),
}

/**
 * Converts a given byte vector to a new byte vector representing a group element.
 * This function is specifically tailored based on the observed pattern from two provided strings.
 * It removes the first three bytes from the input and prepends the byte `0x07`.
 *
 * The observed pattern dictates that the first three bytes are to be discarded and that
 * the byte `0x07` is to be prepended to the remaining bytes. This hardcoded behavior is
 * derived from comparing two strings where the first string starts with "0008cd" and the
 * second string starts with "07", with the rest of the strings being identical after the
 * first six characters.
 *
 * # Arguments
 *
 * * `b` - A byte vector from which the first three bytes will be removed.
 *
 * # Returns
 *
 * A `Result` which is either:
 * - `Ok`: A new byte vector with the byte `0x07` prepended after removing the first three bytes.
 * - `Err`: A `GroupElementError` indicating that the input vector is too short.
 */
fn script_to_group_element(b: Vec<u8>) -> Result<Vec<u8>, UtilError> {
    // Check if the input vector has at least three bytes to remove.
    // If not, return an error.
    if b.len() < 3 {
        return Err(UtilError::InputTooShort);
    }

    // Create a new Vec with the capacity for the original length minus 3 (bytes removed) plus 1 (byte added).
    // This is to prevent reallocation when extending the vector.
    let mut new_vec = Vec::with_capacity(b.len() - 2);

    // Add the byte `0x07` to the new Vec, based on the observed pattern.
    new_vec.push(0x07);

    // Extend the new Vec with the elements from the original Vec, skipping the first three bytes.
    // This aligns with the observed requirement to remove the first three bytes.
    new_vec.extend_from_slice(&b[3..]);

    // Return the new vector wrapped in an Ok.
    Ok(new_vec)
}

pub fn generate_pk_proposition(base58_wallet_pk: &str) -> Result<String, UtilError> {

    let network_prefix = NetworkPrefix::Testnet;
    let encoder = AddressEncoder::new(network_prefix);
    let pk = encoder.parse_address_from_str(base58_wallet_pk)?;
    let script = pk.script()?;
    
    let serialized = script.sigma_serialize_bytes()?;

    let encoded_prop = script_to_group_element(serialized)?;

    // The `format!` macro is used to convert each byte into a hexadecimal string representation.
    // `{:02x}` is a formatting specifier that instructs the macro to:
    // - Convert the number to a hexadecimal string (`x`).
    // - Ensure it has at least two digits (`02`), padding with zeroes if needed.
    let hex_string: String = encoded_prop.iter().map(|byte| format!("{:02x}", byte)).collect();
    
    Ok(hex_string)
}

// Serialize a string to a format suitable for rendering
fn string_to_serialized(value: &str) -> Result<String, UtilError> {
    let bytes = vec_u8_to_vec_i8(value.as_bytes().to_vec());
    let constant = Constant {
        tpe: SType::SColl(Box::new(SType::SByte)),
        v: Literal::Coll(CollKind::NativeColl(NativeColl::CollByte(bytes)))
    };
    let base16_encoded = constant.base16_str()?;
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
pub fn string_to_rendered(value: &str) -> Result<String, UtilError> {
    Ok(serialized_to_rendered(&string_to_serialized(value)?))
}