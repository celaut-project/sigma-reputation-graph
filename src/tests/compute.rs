
mod compute {
    use crate::proof::{ReputationProof, PointerBox};
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

        let object_proof2 = (&object).spend(100, Some(pointer2.clone())).expect("Oops, not Ok.");
        object.outputs.push(object_proof2);

        let pointer = PointerBox::ReputationProof(&object);

        let proof = ReputationProof::create(generate_random_vec_u8(), 100, Some(pointer.clone()));

        assert_approx_eq!((&proof).compute(pointer2.clone()), 1.00);
    }

    #[test]
    fn test_reputation_graph() {

        let pointer3 = PointerBox::String(String::from("nodo-c57"));

        let mut object1 = ReputationProof::create(generate_random_vec_u8(), 100, None);
        let object1_1 = (&object1).spend(80, Some(pointer3.clone())).expect("Oops, not Ok.");
        object1.outputs.push(object1_1);

        let pointer1 = PointerBox::ReputationProof(&object1);

        let mut object2 = ReputationProof::create(generate_random_vec_u8(), 100000000, None);
        let object2_1 = (&object2).spend(10000000, Some(pointer1.clone())).expect("Oops, not Ok.");  // Spend 10%
        object2.outputs.push(object2_1);
        let pointer2 = PointerBox::ReputationProof(&object2);


        let mut proof1 = ReputationProof::create(generate_random_vec_u8(), 100, None);

        let mut proof1_1 = (&proof1).spend(60, None).expect("Oops, not Ok.");
        let proof1_1_1 = (&proof1_1).spend(30, Some(pointer1.clone())).expect("Oops, not Ok.");
        proof1_1.outputs.push(proof1_1_1);

        let proof1_1_2 = (&proof1_1).spend(30, Some(pointer2.clone())).expect("Oops, not Ok.");
        proof1_1.outputs.push(proof1_1_2);
        proof1.outputs.push(proof1_1);

        let mut proof1_2 = (&proof1).spend(10, None).expect("Oops, not Ok.");
        let proof1_2_1 = (&proof1_2).spend(7, Some(pointer3.clone())).expect("Oops, not Ok.");
        proof1_2.outputs.push(proof1_2_1);
        proof1.outputs.push(proof1_2);

        assert_approx_eq!((&proof1).compute(pointer1.clone()), 0.33, 0.01);
        assert_approx_eq!((&proof1).compute(pointer2.clone()), 0.30, 0.01);
        assert_approx_eq!((&proof1).compute(pointer3.clone()), 0.334, 0.001);
    }

    #[test]
    fn test_overspend() {
        let object = ReputationProof::create(generate_random_vec_u8(), 100, None);

        let result = (&object).spend(110, None).map_err(|e| e.kind());
        let expected = Err(std::io::ErrorKind::InvalidData);
        assert_eq!(expected, result);
    }
}