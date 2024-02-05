use std::fmt::{Debug, Formatter};
use crate::proof::pointer_box::{PointerBox};

#[derive(Clone)]
pub struct ReputationProof<'a> {
    box_id: Vec<u8>,
    token_id: Vec<u8>,
    pub(crate) total_amount: i64,
    pub(crate) outputs: Vec<ReputationProof<'a>>,
    pointer_box: Option<PointerBox<'a>>,
}

impl<'a> PartialEq for ReputationProof<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.box_id == other.box_id
    }
}

impl<'a> Debug for ReputationProof<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReputationProof box id: {:?}, with amount {}. \n  out -> {:?}.\n",
               self.box_id, self.total_amount, self.outputs)
    }
}

impl <'a> ReputationProof<'a> {
    fn new(
        box_id: Vec<u8>,
        token_id: Vec<u8>,
        total_amount: i64,
        outputs: Vec<ReputationProof<'a>>,
        pointer_box: Option<PointerBox<'a>>,
    ) -> ReputationProof<'a> {
        ReputationProof {
            box_id,
            token_id,
            total_amount,
            outputs,
            pointer_box,
        }
    }

    /**
    Creates a new reputation proof from scratch.
     */
    pub fn create(
        box_id: Vec<u8>,
        total_amount: i64,
        pointer_box: Option<PointerBox<'a>>,
    ) -> ReputationProof<'a> {
        return ReputationProof::new(
            box_id, vec![],
            total_amount,  vec![],
            pointer_box
        )
    }

    fn current_amount(&self) -> i64
    {
        self.total_amount - self.outputs.iter().map(|out| out.total_amount).sum::<i64>()
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
                 pointer_box: Option<PointerBox<'a>>,
    ) -> Result<ReputationProof<'a>, std::io::Error> {
        match self.can_be_spend(amount) {
            true => Ok(
                ReputationProof::new(
                    vec![], self.get_token_id(),
                    amount, vec![],
                    pointer_box
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
            self.outputs[out_index].total_amount as f64 / self.total_amount as f64
        }
    }

    fn get_token_id(&self) -> Vec<u8> {
        self.token_id.clone()  // TODO Optimize memory if the childs don't store the token_id and get it from the root.
    }


    /**
    Compute the reputation of a pointer searching on all the output tree.

    This configuration assumes that the amount of reputation that a test assigns to its
    pointer box is the amount that is not spent. In other words, if its outputs have
    spent their total_amount, it is not assigning any amount to its pointer box.

    - If there is a pointer_box, it's a leaf.
    Recursive case: if there is pointer, uses the pointer_box's reputation.

    - If there are any pointer box, it's a node.
    Base case: if there is not pointer, computes the reputation directly.

     */
    pub fn compute(&self, pointer: PointerBox<'a>) -> f64 {
        let mut total: f64 = 0.00;
        if self.pointer_box.is_some() {
            total += self.current_proportion() * {
                if self.pointer_box == Some(pointer.clone()) { 1.00 }
                else { self.pointer_box.as_ref().unwrap().compute(pointer.clone()) }
            }
        };
        total += self.outputs
            .iter()
            .enumerate()
            .map(
                |(index, out)|
                    self.expended_proportion(index) * (*out).compute(pointer.clone())
            )
            .sum::<f64>();
        total
    }
}