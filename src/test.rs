#[cfg(test)]
mod tests {
    use crate::ReputationProof;

    #[test]
    fn test_reputation_graph() {

        // Create multiple reputation proofs
        let box1 = ReputationProof::new(vec![], vec![], 100,
                                        30, vec![], None);

        let box2 = ReputationProof::new(vec![], vec![], 50,
                                        10, vec![],
                                        Some(Box::new(box1.clone())));

        let box3 = ReputationProof::new(vec![], vec![], 60,
                                        20, vec![],
                                        Some(Box::new(box2.clone())));

        // Calculate the reputation of the graph
        let reputation = box3.compute();

        // Make assertions about the reputation
        assert_eq!(reputation, 0.00); // You should calculate the correct value based on your data

        // You can add more tests and assertions as needed.
    }
}
