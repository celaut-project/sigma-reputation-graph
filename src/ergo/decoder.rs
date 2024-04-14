use base64::{Engine as _, engine::general_purpose};

// Define your coder types
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum CoderType {
    Base64,
    Hex,
    // Add more types as necessary
}

// Define a custom error to handle coding errors
#[derive(Debug)]
pub enum CoderError {
    UnknownCoderType,
    InvalidInput,
    // Add more errors as necessary
}

// Implement error conversion to a readable representation
impl std::fmt::Display for CoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CoderError::UnknownCoderType => write!(f, "Unknown coder type"),
            CoderError::InvalidInput => write!(f, "Invalid input string"),
            // Add more cases as necessary
        }
    }
}

// Implement conversion of CoderError to std::error::Error if necessary
impl std::error::Error for CoderError {}

// Define the coders as functions that take a string and return a Result<Vec<u8>, CoderError>
fn base64_decode(input: &str) -> Result<Vec<u8>, CoderError> {
    general_purpose::STANDARD.decode(input).map_err(|_| CoderError::InvalidInput)
}

fn hex_decode(input: &str) -> Result<Vec<u8>, CoderError> {
    hex::decode(input).map_err(|_| CoderError::InvalidInput)
}

pub fn vec_u8_to_vec_i8(vec_u8: Vec<u8>) -> Vec<i8> {
    vec_u8.into_iter().map(|x| x as i8).collect()
}


pub fn string_to_bytes(coder_type: CoderType, input: &str) -> Result<Vec<i8>, CoderError> {
    match coder_type {
        CoderType::Base64 => {
            let decoded = base64_decode(input)?;
            Ok(vec_u8_to_vec_i8(decoded))
        },
        CoderType::Hex => {
            let decoded = hex_decode(input)?;
            Ok(vec_u8_to_vec_i8(decoded))
        },
        // Add more cases as necessary
        _ => Err(CoderError::UnknownCoderType),
    }
}
