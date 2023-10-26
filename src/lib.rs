mod test;

#[derive(Clone)]
struct ReputationProof {
    box_id: Vec<u8>,
    token_id: Vec<u8>,
    total_amount: i64,
    expended_amount: i64,
    free_amount: i64,
    expended_percentage: f64,
    free_percentage: f64,
    outputs: Vec<ReputationProof>,
    pointer_box: Option<Box<ReputationProof>>,
}

impl ReputationProof {
    fn new(
        box_id: Vec<u8>,
        token_id: Vec<u8>,
        total_amount: i64,
        expended_amount: i64,
        outputs: Vec<ReputationProof>,
        pointer_box: Option<Box<ReputationProof>>,
    ) -> ReputationProof {
        let free_amount = total_amount - expended_amount;
        let expended_percentage = (expended_amount as f64 / total_amount as f64) * 100.0;
        let free_percentage = (free_amount as f64 / total_amount as f64) * 100.0;

        ReputationProof {
            box_id,
            token_id,
            total_amount,
            expended_amount,
            free_amount,
            expended_percentage,
            free_percentage,
            outputs,
            pointer_box,
        }
    }

    fn compute(&self) -> f64 {
        if let Some(ref ptr) = self.pointer_box {
            // Recursive case: if there is pointer, uses the pointer_box's reputation.
            ptr.compute()
        } else {
            // Base case: if there is not pointer, computes the reputation directly.
            let reputation: f64 = self
                .outputs
                .iter()
                .map(|out| self.expended_percentage * out.compute())
                .sum();
            reputation
        }
    }

    /*
    fn to_dict(&self) -> HashMap<String, serde_json::Value> {
        let mut dict = HashMap::new();
        dict.insert("box_id".to_string(), serde_json::Value::String(String::from_utf8_lossy(&self.box_id).to_string()));
        dict.insert("token_id".to_string(), serde_json::Value::String(String::from_utf8_lossy(&self.token_id).to_string()));
        dict.insert("total_amount".to_string(), serde_json::Value::Number(serde_json::Number::from(self.total_amount)));
        dict.insert("free_amount".to_string(), serde_json::Value::Number(serde_json::Number::from(self.free_amount)));
        dict.insert("free_percentage".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.free_percentage).unwrap()));
        dict
    }*/
}
