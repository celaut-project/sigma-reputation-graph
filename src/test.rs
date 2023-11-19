
#[cfg(test)]
mod tests {
    use crate::{PointerBox, ReputationProof, static_spend, static_compute_reputation};

    #[test]
    fn test_reputation_graph() {

        let owner_obj1 = ReputationProof::create(100, None);
        let pointer1 = PointerBox::ReputationProof(&owner_obj1);

        let owner_obj2 = ReputationProof::create(100000000, None);
        let pointer2 = PointerBox::ReputationProof(&owner_obj2);

        let pointer3 = PointerBox::String(String::from("nodo-c57"));

        // box 1
        let mut owner_box1 = ReputationProof::create(100, None);
        let box1 = &mut owner_box1;

        let box2 = (*box1).spend(60, None);
        let box3 = (*box1).spend(10, Some(&pointer1));

        // box 2
        static_spend(box2, 30, Some(&pointer1));
        static_spend(box2, 30, Some(&pointer2));

        // box 3
        static_spend(box3, 7, Some(&pointer3));

        assert_eq!(static_compute_reputation(box1, &pointer1), 0.00);
        assert_eq!(static_compute_reputation(box1, &pointer2), 0.00);
        assert_eq!(static_compute_reputation(box2, &pointer3), 0.00);

        assert_eq!(static_compute_reputation(box2, &pointer1), 0.00);
        assert_eq!(static_compute_reputation(box2, &pointer2), 0.00);
        assert_eq!(static_compute_reputation(box2, &pointer3), 0.00);
    }
}
