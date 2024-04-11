use ergo_lib::ergoscript_compiler::compiler::CompileError;
use ergo_lib::ergotree_ir::chain::address::NetworkPrefix;
use ergo_lib::ergotree_ir::chain::address::AddressEncoder;
use ergo_lib::ergotree_ir::ergo_tree::ErgoTree;
use ergo_lib::ergotree_ir::ergo_tree::ErgoTreeError;
use ergo_lib::ergotree_ir::serialization::SigmaParsingError;
use ergo_lib::ergoscript_compiler::compiler::compile;
use ergo_lib::ergoscript_compiler::script_env::ScriptEnv;
use ergo_lib::ergotree_ir::chain::address::NetworkAddress;
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;
use hex::ToHex;
use sha2::{Digest, Sha256};
use hex;
use thiserror::Error;

const CONTRACT: &str = "
{
    proveDlog(SELF.R7[GroupElement].get) &&
    sigmaProp(SELF.tokens.size == 1) &&
    sigmaProp(OUTPUTS.forall { (x: Box) =>
      !(x.tokens.exists { (token: (Coll[Byte], Long)) => token._1 == SELF.tokens(0)._1 }) ||
      (
        x.R7[GroupElement].get == SELF.R7[GroupElement].get &&
        x.tokens.size == 1 &&
        x.propositionBytes == SELF.propositionBytes
      )
    })
}
";

#[derive(Clone, Debug)]
pub struct ProofContract {
    ergo_tree: ErgoTree
}

#[derive(Debug, Error)]
pub enum ProofContractError {
    #[error("proof contract: sigma parsing error {0}")]
    SigmaParsing(#[from] SigmaParsingError),
    #[error("proof contract: ergo tree error {0:?}")]
    ErgoTreeError(ErgoTreeError),
    #[error("proof contract: ergo tree compile error {0:?}")]
    CompileError(CompileError)
}

impl ProofContract {
    pub fn new() -> Result<Self, ProofContractError> {
        let envs = ScriptEnv::new();
        match compile(CONTRACT, envs) {
            Ok(ergo_tree) => Ok(Self {ergo_tree}),
            Err(err) => Err(ProofContractError::CompileError(err)),
        }
    }

    pub fn from_ergo_tree_bytes(ergo_tree_bytes: &[u8]) -> Result<Self, ProofContractError> {
        let ergo_tree = ErgoTree::sigma_parse_bytes(ergo_tree_bytes)?;
        Ok(Self { ergo_tree })
    }

    pub fn ergo_tree(&self) -> ErgoTree {
        self.ergo_tree.clone()
    }

    pub fn ergo_tree_address(&self, network_prefix: NetworkPrefix) -> Vec<NetworkAddress> {
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        let addr: String = self.ergo_tree.to_base16_bytes().unwrap_or_default().encode_hex();  // todo() needs to be checked.
        let address = encoder
            .parse_address_from_str(addr.as_str())
            .unwrap();
        vec![NetworkAddress::new(
            network_prefix,
            &address
        )]
    }

    pub fn ergo_tree_hash(&self) -> Result<String, ProofContractError> {
        match self.ergo_tree.template_bytes() {
            Ok(template) => Ok(hex::encode(Sha256::digest(template)).to_string()),
            Err(err) => Err(ProofContractError::ErgoTreeError(err)),
        }
    }
}