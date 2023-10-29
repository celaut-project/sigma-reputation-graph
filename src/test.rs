
#[cfg(test)]
mod tests {
    use crate::ReputationProof;

    #[test]
    fn test_reputation_graph() {

        let object = ReputationProof::new(vec![], vec![],
                                          100, 0, vec![],
                                          None);

        let box1 = ReputationProof::new(vec![], vec![],
                                        100, 0, vec![],
                                        Some(&object));

        let box2 = ReputationProof::new(vec![], vec![],
                                                 100, 10,
                                        vec![&box1], None);

        let box3 = ReputationProof::new(vec![], vec![],
                                                 100, 50,
                                        vec![&box2], None);


        let reputation = box3.compute(Some(&object));

        assert_eq!(reputation, 0.05);
    }
}
