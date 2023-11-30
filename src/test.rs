
#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use crate::{PointerBox, ReputationProof};

    #[test]
    fn test_reputation_graph() {

        let owner_obj1 = ReputationProof::create(100, None);
        let pointer1 = PointerBox::ReputationProof(&owner_obj1);

        let owner_obj2 = ReputationProof::create(100000000, None);
        let pointer2 = PointerBox::ReputationProof(&owner_obj2);

        let pointer3 = PointerBox::String(String::from("nodo-c57"));

        // box 1
        let owner_box1 = ReputationProof::create(100, None);
        let cell1 = RefCell::new(owner_box1);

        let cell2 = cell1.borrow_mut().spend(60, None);
        let cell3 = cell1.borrow_mut().spend(10, Some(&pointer1));

        // box 2
        cell2.borrow_mut().spend(30, Some(&pointer1));
        cell2.borrow_mut().spend(30, Some(&pointer2));

        // box 3
        cell3.borrow_mut().spend(7, Some(&pointer3));

        assert_eq!(cell1.borrow_mut().compute(Some(&pointer1)), 0.00);
        assert_eq!(cell1.borrow_mut().compute(Some(&pointer2)), 0.00);
        assert_eq!(cell1.borrow_mut().compute(Some(&pointer3)), 0.00);

        assert_eq!(cell2.borrow_mut().compute(Some(&pointer1)), 0.00);
        assert_eq!(cell2.borrow_mut().compute(Some(&pointer2)), 0.00);
        assert_eq!(cell2.borrow_mut().compute(Some(&pointer3)), 0.00);
    }
}
