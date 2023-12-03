use std::fmt::{Debug, Formatter};
use rand::Rng;
use pyo3::prelude::*;
use std::cmp::Ordering;

fn generate_random_vec_u8(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vec_u8: Vec<u8> = Vec::new();

    for _ in 0..length {
        vec_u8.push(rng.gen());
    }

    vec_u8
}



#[derive(PartialEq, Debug)]
enum PointerBox<'a> {
    ReputationProof(&'a ReputationProof<'a>),
    String(String)
}

impl<'a, 'b> PointerBox<'a> {
    fn compute(&self, pointer: &'b PointerBox<'a>) -> f64 {
        match self {
            PointerBox::ReputationProof(proof) => proof.compute(pointer),
            PointerBox::String(..) => 0.00
        }
    }
}

#[derive(Clone)]
struct ReputationProof<'a> {
    box_id: Vec<u8>,
    token_id: Vec<u8>,
    total_amount: i64,
    outputs: Vec<&'a ReputationProof<'a>>,
    pointer_box: Option<&'a PointerBox<'a>>,
}

impl<'a> PartialEq for ReputationProof<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.box_id == other.box_id
    }
}

impl<'a> Debug for ReputationProof<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReputationProof {{ box id: {:?}", self.box_id)
    }
}

impl <'a, 'b> ReputationProof<'a> {
    fn new(
        box_id: Vec<u8>,
        token_id: Vec<u8>,
        total_amount: i64,
        outputs: Vec<&'b ReputationProof<'a>>,
        pointer_box: Option<&'b PointerBox<'a>>,
    ) -> ReputationProof<'b> {
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
        total_amount: i64,
        pointer_box: Option<&'b PointerBox<'a>>,
    ) -> ReputationProof<'b> {
        /*
        *   Supposed that ->  'b < 'a
         */
        return ReputationProof::new(
            generate_random_vec_u8(16), vec![],
            total_amount,  vec![],
            pointer_box
        )
    }

    /**
    Creates a new reputation proof from the current one.
    Raises exceptions if any rule is violated.
     */
    pub fn spend(&self,
                 amount: i64,
                 pointer_box: Option<&'b PointerBox<'a>>,
    ) -> ReputationProof<'b> {
        // TODO validate if waste amount is possible for self.

        let new = ReputationProof::new(
            vec![], self.get_token_id(),
            amount, vec![],
            pointer_box
        );
        // TODO Must execute self.outputs.push(new) after the function. How make it a rule?
        new
    }

    /**
    Get the proportion of reputation that have the out_index output over the total.
     */
    fn expended_proportion(&self, out_index: usize) -> f64 {
        return self.outputs[out_index].total_amount as f64 / self.total_amount as f64;
    }

    /**
    Optimize memory if the childs don't store the token_id and get it from the root.
     */
    fn get_token_id(&self) -> Vec<u8> {
        return self.token_id.clone()
    }


    /**
    Compute the reputation of a pointer searching on all the output tree.

    This configuration don't allow to have assigned reputation and delegated reputation
    at the same time.

    - If there is a pointer_box, it's a leaf.
      Recursive case: if there is pointer, uses the pointer_box's reputation.

    - If there are any pointer box, it's a node.
      Base case: if there is not pointer, computes the reputation directly.

     */
    pub fn compute(&self, pointer: &'b PointerBox<'a>) -> f64 {
        if self.pointer_box.is_some() {
            if self.pointer_box == Some(pointer) {
                1.00
            } else {
                self.pointer_box.unwrap().compute(pointer)
            }
        } else {
            self.outputs
                .iter()
                .enumerate()
                .map(
                    |(index, out)|
                        self.expended_proportion(index) * (*out).compute(pointer)
                )
                .sum()
        }
    }
}


#[pyfunction]
fn static_compute()
{
    println!("Static compute function");
}

#[pyfunction]
fn dynamic_compute()
{
    println!("Dynamic compute function.");
}


#[pymodule]
fn compute_reputation_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(static_compute, m)?)?;
    m.add_function(wrap_pyfunction!(dynamic_compute, m)?)?;

    Ok(())
}