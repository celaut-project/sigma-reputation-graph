mod test;

#[derive(PartialEq)]
enum PointerBox<'a> {
    ReputationProof(&'a ReputationProof<'a>),
    String
}

#[derive(Clone)]
struct ReputationProof<'a> {
    box_id: Vec<u8>,
    token_id: Vec<u8>,
    total_amount: i64,
    expended_amount: i64,
    free_amount: i64,
    expended_proportion: f64,
    free_proportion: f64,
    outputs: Vec<&'a ReputationProof<'a>>,
    pointer_box: Option<&'a PointerBox<'a>>,
}

impl<'a> PartialEq for ReputationProof<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.box_id == other.box_id
    }
}

impl <'a> ReputationProof<'a> {
    fn new(
        box_id: Vec<u8>,
        token_id: Vec<u8>,
        total_amount: i64,
        expended_amount: i64,
        outputs: Vec<&'a ReputationProof<'a>>,
        pointer_box: Option<&'a PointerBox<'a>>,
    ) -> ReputationProof<'a> {
        let free_amount = total_amount - expended_amount;
        let expended_percentage = expended_amount as f64 / total_amount as f64;
        let free_percentage = free_amount as f64 / total_amount as f64;

        ReputationProof {
            box_id,
            token_id,
            total_amount,
            expended_amount,
            free_amount,
            expended_proportion: expended_percentage,
            free_proportion: free_percentage,
            outputs,
            pointer_box,
        }
    }

    fn compute(&self, pointer: Option<&'a PointerBox<'a>>) -> f64 {
        if self.pointer_box.is_some() {
            // Recursive case: if there is pointer, uses the pointer_box's reputation.
            if pointer.is_some() && self.pointer_box == pointer {
                1.00
            } else {
                0.00 // ptr.compute(None)  // TODO
            }
        } else {
            // Base case: if there is not pointer, computes the reputation directly.
            let reputation: f64 = self
                .outputs
                .iter()
                .map(|out| self.expended_proportion * out.compute(pointer))
                .sum();
            reputation
        }
    }

}
