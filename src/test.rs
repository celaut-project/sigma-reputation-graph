
#[cfg(test)]
mod tests {
    use crate::ReputationProof;

    #[test]
    fn test_reputation_graph() {
        // Crear múltiples pruebas de reputación
        let box1 = ReputationProof::new(vec![], vec![],
                                        100, 30, vec![],
                                        None);

        let box2 = ReputationProof::new(vec![], vec![],
                                                 50, 10, vec![],
                                                 Some(&box1));

        let box3 = ReputationProof::new(vec![], vec![],
                                                 60, 20, vec![],
                                                 Some(&box2));

        // Calcular la reputación de box3 con box1 como puntero
        let reputation = box3.compute(Some(&box1));

        assert_eq!(reputation, 1.00);
    }
}
