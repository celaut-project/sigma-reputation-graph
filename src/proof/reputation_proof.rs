use std::fmt::{Debug, Formatter};
use crate::proof::pointer_box::PointerBox;

use super::pointer_box::Pointer;

#[derive(Clone)]
pub struct ReputationProof {
    token_id: Vec<u8>,
    pub(crate) total_amount: i64,
    pub(crate) outputs: Vec<PointerBox>,
}

impl<'a> PartialEq for ReputationProof {
    fn eq(&self, other: &Self) -> bool {
        self.token_id == other.token_id
    }
}

impl<'a> Debug for ReputationProof {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReputationProof box id: {:?}, with amount {}. \n  out -> {:?}.\n",
               self.token_id, self.total_amount, self.outputs)
    }
}

impl <'a> ReputationProof {
    fn new(
        token_id: Vec<u8>,
        total_amount: i64,
        outputs: Vec<PointerBox>,
    ) -> ReputationProof {
        ReputationProof {
            token_id,
            total_amount,
            outputs
        }
    }

    /**
    Creates a new reputation proof from scratch.
     */
    pub fn create(
        token_id: Vec<u8>,
        total_amount: i64
    ) -> ReputationProof {
        return ReputationProof::new(
            token_id,
            total_amount,
            vec![],
        )
    }

    fn current_amount(&self) -> i64
    {
        self.total_amount - self.outputs.iter().map(|out| out.amount).sum::<i64>()
    }

    fn current_proportion(&self) -> f64 {
        self.current_amount() as f64 / self.total_amount as f64
    }


    /**
    Don't pub needed if push function can be used.
     */
    pub fn can_be_spend(&self, amount: i64) -> bool
    {
        self.current_amount() >= amount
    }

    /*          <!-- Difficult to use lifetimes here -->
        pub fn push(mut self, child: ReputationProof) -> Result<ReputationProof<'b>, std::io::Error>
        {
            match self.can_be_spend(child.total_amount) {
                true => {
                    self.outputs.push(child);
                    Ok(self)
                },
                false => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Can't spend this amount {}", child.total_amount),
                ))
            }
        }
    */

    /**
    Creates a new reputation proof from the current one.
    Raises exceptions if any rule is violated.
     */
    pub fn spend(&self,
                 amount: i64,
                 pointer_box: Option<PointerBox>,
    ) -> Result<ReputationProof, std::io::Error> {
        match self.can_be_spend(amount) {
            true => Ok(
                ReputationProof::new(
                    vec![],
                    amount, 
                    vec![]
                )
            ),
            false => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Can't spend this amount {}", amount),
            ))
        }
    }

    /**
    Get the proportion of reputation that have the out_index output over the total.
     */
    fn expended_proportion(&self, out_index: usize) -> f64 {
        if self.total_amount as f64 == 0.00 { 0.00 } 
        else {
            self.outputs[out_index].amount as f64 / self.total_amount as f64
        }
    }

    fn get_token_id(&self) -> Vec<u8> {
        self.token_id.clone()
    }


    /**
    Compute the reputation of a pointer searching on the proof boxes.
     */
    pub fn compute(&self, pointer: Pointer) -> f64 {
        self.outputs
            .iter()
            .enumerate()
            .map(
                |(index, out)|
                    self.expended_proportion(index) * (*out).compute(pointer.clone())
            )
            .sum::<f64>()
    }
}