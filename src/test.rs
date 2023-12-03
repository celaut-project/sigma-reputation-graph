

use crate::{PointerBox, ReputationProof};
use assert_approx_eq::assert_approx_eq;

#[deprecated]
fn test_simple() {
    let pointer2 = PointerBox::String(String::from("nodo-c57"));

    let mut object = ReputationProof::create(100, None);
    let object_proof2 = (&object).spend(100, Some(&pointer2));
    object.outputs.push(&object_proof2);

    let pointer = PointerBox::ReputationProof(&object);

    let proof = ReputationProof::create(100, Some(&pointer));

    assert_approx_eq!((&proof).compute(&pointer2), 1.00);
}

#[test]
fn test_reputation_graph() {

    let pointer3 = PointerBox::String(String::from("nodo-c57"));

    let mut object1 = ReputationProof::create(100, None);
    let object1_1 = (&object1).spend(80, Some(&pointer3));
    object1.outputs.push(&object1_1);
    let pointer1 = PointerBox::ReputationProof(&object1);

    let mut object2 = ReputationProof::create(100000000, None);
    let object2_1 = (&object2).spend(10000000, Some(&pointer1));  // Spend 10%
    object2.outputs.push(&object2_1);
    let pointer2 = PointerBox::ReputationProof(&object2);


    let mut proof1 = ReputationProof::create(100, None);

    let mut proof1_1 = (&proof1).spend(60, None);
    let proof1_1_1 = (&proof1_1).spend(30, Some(&pointer1));
    proof1_1.outputs.push(&proof1_1_1);

    let proof1_1_2 =(&proof1_1).spend(30, Some(&pointer2));
    proof1_1.outputs.push(&proof1_1_2);
    proof1.outputs.push(&proof1_1);

    let mut proof1_2 = (&proof1).spend(10, None);
    let proof1_2_1 = (&proof1_2).spend(7, Some(&pointer3));
    proof1_2.outputs.push(&proof1_2_1);
    proof1.outputs.push(&proof1_2);

    assert_approx_eq!((&proof1).compute(&pointer1), 0.33, 0.01);
    assert_approx_eq!((&proof1).compute(&pointer2), 0.30, 0.01);
    assert_approx_eq!((&proof1).compute(&pointer3), 0.334, 0.001);
}