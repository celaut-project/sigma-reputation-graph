
#[cfg(test)]
mod tests {
    use crate::{PointerBox, ReputationProof};

    #[test]
    fn test_reputation_graph() {

        let owner_obj1 = ReputationProof::create(100, None);
        let pointer1 = PointerBox::ReputationProof(&owner_obj1);

        let owner_obj2 = ReputationProof::create(100000000, None);
        let pointer2 = PointerBox::ReputationProof(&owner_obj2);

        let pointer3 = PointerBox::String(String::from("nodo-c57"));

        // proof 1
        let mut proof1 = ReputationProof::create(100, None);

        let mut proof2 = (&proof1).spend(60, None);
        proof1.outputs.push(&proof2);

        let mut proof3 = (&proof1).spend(10, Some(&pointer1));
        proof1.outputs.push(&proof3);

        // proof 2
        /*let (mut proof2, _) = proof2.spend(30, Some(&pointer1));
        let (mut proof2, _) = proof2.spend(30, Some(&pointer2));

        // proof 3
        let (mut proof3, _) = proof3.spend(7, Some(&pointer3));

        assert_eq!((&proof1).compute(Some(&pointer1)), 0.00);
        assert_eq!((&proof1).compute(Some(&pointer2)), 0.00);
        assert_eq!((&proof1).compute(Some(&pointer3)), 0.00);

        assert_eq!((&proof2).compute(Some(&pointer1)), 0.00);
        assert_eq!((&proof2).compute(Some(&pointer2)), 0.00);
        assert_eq!((&proof2).compute(Some(&pointer3)), 0.00); */
    }
}
