use std::fmt::{Debug, Formatter};
use crate::proof::reputation_proof::ReputationProof;

#[derive(PartialEq, Debug, Clone)]
pub enum Pointer<'a> {
    ReputationProof(&'a ReputationProof<'a>),
    String(String)
}

#[derive(Clone)]
pub struct  PointerBox<'a> {
    pointer: Pointer<'a>,
    box_id: Vec<u8>,
    pub(crate) amount: i64
}

impl<'a> PointerBox<'a> {
    pub(crate) fn compute(&self, pointer: Pointer<'a>) -> f64 {
        match self.pointer {
            Pointer::ReputationProof(proof) => proof.compute(pointer),
            Pointer::String(..) => if pointer == self.pointer { 1.00 } else { 0.00 }
        }
    }
}

impl<'a> Debug for PointerBox<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PointerBox box id: {:?}, with amount {}. \n",
               self.box_id, self.amount)
    }
}

impl <'a> PointerBox<'a> {
    pub fn new(
        box_id: Vec<u8>,
        amount: i64,
        pointer: Pointer<'a>
    ) -> PointerBox<'a> {
        PointerBox {
            pointer,
            box_id,
            amount,
        }
    }
}