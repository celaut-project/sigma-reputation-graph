use std::rc::Rc;
use std::str::FromStr;

use derive_more::{From, Into};
use ergo_lib::chain::ergo_state_context::ErgoStateContext;
use ergo_lib::chain::transaction::prover_result::ProverResult;
use ergo_lib::chain::transaction::unsigned::UnsignedTransaction;
use ergo_lib::chain::transaction::{Input, Transaction, UnsignedInput};
use ergo_lib::ergo_chain_types::{Header, PreHeader};
use ergo_lib::ergotree_interpreter::eval::env::Env;
use ergo_lib::ergotree_interpreter::sigma_protocol::private_input::{DlogProverInput, PrivateInput};
use ergo_lib::ergotree_interpreter::sigma_protocol::prover::hint::HintsBag;
use ergo_lib::ergotree_interpreter::sigma_protocol::prover::{ProofBytes, Prover};
use ergo_lib::ergotree_ir::chain::address::{Address, AddressEncoder, NetworkPrefix};
use ergo_lib::wallet::derivation_path::{ChildIndex, DerivationPath};
use ergo_lib::wallet::ext_secret_key::ExtSecretKey;
use ergo_lib::wallet::mnemonic::Mnemonic;
use ergo_lib::wallet::secret_key::SecretKey;
use ergo_lib::wallet::signing::{make_context, TransactionContext, TxSigningError};
use ergo_lib::wallet::tx_context::TransactionContextError;
use serde::Deserialize;
use crate::ergo::submit::transaction::{TransactionCandidate, UnsignedTransactionOps};

pub fn generate_headers() -> [Header; 10]
{
    let json = r#"{
        "extensionId": "d16f25b14457186df4c5f6355579cc769261ce1aebc8209949ca6feadbac5a3f",
        "difficulty": "626412390187008",
        "votes": "040000",
        "timestamp": 1618929697400,
        "size": 221,
        "stateRoot": "8ad868627ea4f7de6e2a2fe3f98fafe57f914e0f2ef3331c006def36c697f92713",
        "height": 471746,
        "nBits": 117586360,
        "version": 2,
        "id": "4caa17e62fe66ba7bd69597afdc996ae35b1ff12e0ba90c22ff288a4de10e91b",
        "adProofsRoot": "d882aaf42e0a95eb95fcce5c3705adf758e591532f733efe790ac3c404730c39",
        "transactionsRoot": "63eaa9aff76a1de3d71c81e4b2d92e8d97ae572a8e9ab9e66599ed0912dd2f8b",
        "extensionHash": "3f91f3c680beb26615fdec251aee3f81aaf5a02740806c167c0f3c929471df44",
        "powSolutions": {
          "pk": "02b3a06d6eaa8671431ba1db4dd427a77f75a5c2acbd71bfb725d38adc2b55f669",
          "w": "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
          "n": "5939ecfee6b0d7f4",
          "d": "1234000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        },
        "adProofsId": "86eaa41f328bee598e33e52c9e515952ad3b7874102f762847f17318a776a7ae",
        "transactionsId": "ac80245714f25aa2fafe5494ad02a26d46e7955b8f5709f3659f1b9440797b3e",
        "parentId": "6481752bace5fa5acba5d5ef7124d48826664742d46c974c98a2d60ace229a34"
    }"#;
    let header: Header = serde_json::from_str(json).unwrap();
    let headers = std::array::from_fn(|_| header.clone());
    headers
}

pub fn get_ergo_state_context() -> ErgoStateContext
{
    let headers = generate_headers();
    let pre_header = PreHeader::from(headers.last().unwrap().clone());
    let ergo_state_context = ErgoStateContext { pre_header, headers };
    ergo_state_context
}

pub trait SigmaProver {
    fn sign(&self, tx: TransactionCandidate) -> Result<Transaction, TxSigningError>;
}

#[derive(Deserialize, Into, From)]
pub struct SeedPhrase(String);

#[derive(Clone, Deserialize, Into, From)]
#[serde(try_from = "String")]
pub struct WalletSecret(DlogProverInput);

impl TryFrom<String> for WalletSecret {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        DlogProverInput::from_base16_str(value)
            .map(Self)
            .ok_or("Private inputs must be provided in Base16".to_string())
    }
}

pub struct Wallet {
    pub(crate) secrets: Vec<PrivateInput>,
    /// Necessary to use `sign_transaction` function from `sigma-rust`. If we're only signing P2PK
    /// inputs, then this field can be any arbitrary value.
    pub(crate) ergo_state_context: ErgoStateContext,
}

impl Wallet {
    pub fn try_from_seed(seed: String) -> Option<(Self, Address)> {
        if let Ok(sk) = ExtSecretKey::derive_master(Mnemonic::to_seed(&<String>::from(seed), "")) {
            if let SecretKey::DlogSecretKey(dpi) = sk.secret_key() {


                /*
                    TODO The derivation path should be m/44'/429'/0'/0
                    Based on: https://discord.com/channels/668903786361651200/669989266478202917/1232355179232362566
                */

                // println!("derivation default path -> {:?}", sk.path().to_string());  // "m/"
                let addr = Address::P2Pk(sk.public_image());
                /* println!(
                    "Wallet address: {:?}",
                    AddressEncoder::encode_address_as_string(NetworkPrefix::Testnet, &addr)  // 3WvdKWY5dHf4zMPHWTjWKvF7BwpzNJzC72HPKCWwLcde6TdK9ht2
                );*/ 

                let wallet = Self {
                    secrets: vec![PrivateInput::DlogProverInput(dpi)],
                    ergo_state_context: get_ergo_state_context(),
                };
                Some((wallet, addr))
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn trivial(secrets: Vec<WalletSecret>) -> Self {
        Self {
            secrets: secrets
                .into_iter()
                .map(|WalletSecret(pi)| PrivateInput::DlogProverInput(pi))
                .collect(),
            ergo_state_context: get_ergo_state_context(),
        }
    }
}

impl Prover for Wallet {
    fn secrets(&self) -> &[PrivateInput] {
        self.secrets.as_ref()
    }

    fn append_secret(&mut self, input: PrivateInput) {
        self.secrets.push(input);
    }
}

impl SigmaProver for Wallet {
    fn sign(&self, tx: TransactionCandidate) -> Result<Transaction, TxSigningError> {
        let TransactionCandidate {
            inputs,
            data_inputs,
            output_candidates,
        } = tx;

        let unsigned_inputs = inputs
            .iter()
            .map(|(bx, ext)| UnsignedInput {
                box_id: bx.box_id(),
                extension: ext.clone(),
            })
            .collect();

        let unsigned_tx = UnsignedTransaction::new_from_vec(
            unsigned_inputs,
            data_inputs
                .clone()
                .map(|d| d.mapped(|b| b.box_id().into()).to_vec())
                .unwrap_or_else(Vec::new),
            output_candidates.to_vec(),
        )
        .unwrap();
        let tx_context = TransactionContext::new(
            unsigned_tx,
            inputs.into_iter().map(|(b, _)| b).collect(),
            data_inputs.map(|d| d.to_vec()).unwrap_or_else(Vec::new),
        )?;
        let tx = tx_context.spending_tx.clone();
        let message_to_sign = tx.bytes_to_sign()?;
        let signed_inputs =
            tx.inputs.enumerated().try_mapped(|(idx, input)| {
                let input_box = tx_context.get_input_box(&input.box_id).ok_or(
                    TxSigningError::TransactionContextError(TransactionContextError::InputBoxNotFound(idx)),
                )?;
                let addr = Address::recreate_from_ergo_tree(&input_box.ergo_tree).unwrap();
                if let Address::P2Pk(_) = addr {
                    let ctx = Rc::new(make_context(&self.ergo_state_context, &tx_context, idx)?);
                    let hints_bag = HintsBag::empty();
                    self.prove(
                        &input_box.ergo_tree,
                        &Env::empty(),
                        ctx,
                        message_to_sign.as_slice(),
                        &hints_bag,
                    )
                    .map(|proof| Input::new(input.box_id, proof.into()))
                    .map_err(|e| TxSigningError::ProverError(e, idx))
                } else {
                    Ok(Input::new(
                        input_box.box_id(),
                        ProverResult {
                            proof: ProofBytes::Empty,
                            extension: input.extension,
                        },
                    ))
                }
            })?;
        Ok(Transaction::new(
            signed_inputs,
            tx.data_inputs,
            tx.output_candidates,
        )?)
    }
}

pub struct NoopProver;

impl SigmaProver for NoopProver {
    fn sign(&self, tx: TransactionCandidate) -> Result<Transaction, TxSigningError> {
        Ok(tx.into_tx_without_proofs())
    }
}