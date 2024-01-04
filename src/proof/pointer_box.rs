use std::fmt::{Debug};
use crate::proof::reputation_proof::ReputationProof;

#[derive(PartialEq, Debug, Clone)]
pub enum PointerBox<'a> {
    ReputationProof(&'a ReputationProof<'a>),
    String(String)
}

impl<'a> PointerBox<'a> {
    pub(crate) fn compute(&self, pointer: PointerBox<'a>) -> f64 {
        match self {
            PointerBox::ReputationProof(proof) => proof.compute(pointer),
            PointerBox::String(..) => 0.00
        }
    }
}