use std::fmt::{Debug, Formatter};
use pyo3::prelude::*;


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
        box_id: Vec<u8>,
        total_amount: i64,
        pointer_box: Option<&'b PointerBox<'a>>,
    ) -> ReputationProof<'b> {
        return ReputationProof::new(
            box_id, vec![],
            total_amount,  vec![],
            pointer_box
        )
    }

    fn can_be_spend(&self, amount: i64) -> bool
    {
        self.outputs.iter().map(|out| out.total_amount).sum::<i64>()
            + amount <= self.total_amount
    }

    /**
        Creates a new reputation proof from the current one.
        Raises exceptions if any rule is violated.
     */
    pub fn spend(&self,
                 amount: i64,
                 pointer_box: Option<&'b PointerBox<'a>>,
    ) -> Result<ReputationProof<'b>, std::io::Error> {
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
        return self.outputs[out_index].total_amount as f64 / self.total_amount as f64;
    }

    fn get_token_id(&self) -> Vec<u8> {
        return self.token_id.clone()  // TODO Optimize memory if the childs don't store the token_id and get it from the root.
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
        // TODO -> Add backtracking.
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
fn spend()
{
    /*
        Params
        - Ergo node url
        - SurrealDB endpoint  https://surrealdb.com/docs/embedding/rust#connect
        - proof
        - amount
        - pointer
    */
    println!("Spend function.");
}

#[pyfunction]
fn compute()
{
    /*
        Params
        - Ergo node url
        - SurrealDB endpoint
        - pointer to calculate
    */
    println!("Compute function.");
}


#[pymodule]
fn compute_reputation_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(spend, m)?)?;
    m.add_function(wrap_pyfunction!(compute, m)?)?;

    Ok(())
}



/**
    TESTS
*/

mod test {
    use super::{PointerBox, ReputationProof};
    use assert_approx_eq::assert_approx_eq;
    use rand::Rng;

    fn generate_random_vec_u8() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut vec_u8: Vec<u8> = Vec::new();
    
        for _ in 0..16 {
            vec_u8.push(rng.gen());
        }
    
        vec_u8
    }

    #[test]
    fn test_simple() {
        let pointer2 = PointerBox::String(String::from("nodo-c57"));
    
        let mut object = ReputationProof::create(generate_random_vec_u8(), 100, None);
        
        let object_proof2 = (&object).spend(100, Some(&pointer2)).expect("Oops, not Ok.");
        object.outputs.push(&object_proof2);
     
        let pointer = PointerBox::ReputationProof(&object);
    
        let proof = ReputationProof::create(generate_random_vec_u8(), 100, Some(&pointer));
    
        assert_approx_eq!((&proof).compute(&pointer2), 1.00);
    }
    
    #[test]
    fn test_reputation_graph() {
    
        let pointer3 = PointerBox::String(String::from("nodo-c57"));
    
        let mut object1 = ReputationProof::create(generate_random_vec_u8(), 100, None);
        let object1_1 = (&object1).spend(80, Some(&pointer3)).expect("Oops, not Ok.");
        object1.outputs.push(&object1_1);
        
        let pointer1 = PointerBox::ReputationProof(&object1);
    
        let mut object2 = ReputationProof::create(generate_random_vec_u8(), 100000000, None);
        let object2_1 = (&object2).spend(10000000, Some(&pointer1)).expect("Oops, not Ok.");  // Spend 10%
        object2.outputs.push(&object2_1);
        let pointer2 = PointerBox::ReputationProof(&object2);
    
    
        let mut proof1 = ReputationProof::create(generate_random_vec_u8(), 100, None);
    
        let mut proof1_1 = (&proof1).spend(60, None).expect("Oops, not Ok.");
        let proof1_1_1 = (&proof1_1).spend(30, Some(&pointer1)).expect("Oops, not Ok.");
        proof1_1.outputs.push(&proof1_1_1);
    
        let proof1_1_2 =(&proof1_1).spend(30, Some(&pointer2)).expect("Oops, not Ok.");
        proof1_1.outputs.push(&proof1_1_2);
        proof1.outputs.push(&proof1_1);
    
        let mut proof1_2 = (&proof1).spend(10, None).expect("Oops, not Ok.");
        let proof1_2_1 = (&proof1_2).spend(7, Some(&pointer3)).expect("Oops, not Ok.");
        proof1_2.outputs.push(&proof1_2_1);
        proof1.outputs.push(&proof1_2);
    
        assert_approx_eq!((&proof1).compute(&pointer1), 0.33, 0.01);
        assert_approx_eq!((&proof1).compute(&pointer2), 0.30, 0.01);
        assert_approx_eq!((&proof1).compute(&pointer3), 0.334, 0.001);
    }

    #[test]
    fn test_overspend() {
        let object = ReputationProof::create(generate_random_vec_u8(), 100, None);
        
        let result = (&object).spend(110, None).map_err(|e| e.kind());
        let expected = Err(std::io::ErrorKind::InvalidData);
        assert_eq!(expected, result);
    }

}
