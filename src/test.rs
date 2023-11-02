
#[cfg(test)]
mod tests {
    use crate::{PointerBox, ReputationProof};

    #[test]
    fn test_reputation_graph() {

        let object = ReputationProof::new(vec![], vec![],
                                          100,  vec![],
                                          None);

        let pointer = PointerBox::ReputationProof(&object);

        let box3 = ReputationProof::new(vec![], vec![],
                                        100,  vec![],
                                        Some(&pointer));

        let box2 = ReputationProof::new(vec![], vec![],
                                                 100,
                                        vec![&box3], None);

        let box1 = ReputationProof::new(vec![], vec![],
                                                 100,
                                        vec![&box2], None);


        let reputation = box1.compute(Some(&pointer));

        assert_eq!(reputation, 0.05);
    }
}
