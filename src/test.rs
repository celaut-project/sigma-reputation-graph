
#[cfg(test)]
mod tests {
    use crate::{PointerBox, ReputationProof};

    #[test]
    fn test_reputation_graph() {

        let object = ReputationProof::create(100, None);

        let pointer = PointerBox::ReputationProof(&object);

        let mut box1 = ReputationProof::create(100, None);

        let box2 = box1.spend(60, None);

        box2.spend(30, Some(&pointer));

        let reputation = box1.compute(Some(&pointer));

        assert_eq!(reputation, 0.3);
    }
}
