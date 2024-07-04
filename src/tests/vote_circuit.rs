use halo2_proofs::dev::MockProver;
use halo2_proofs::pasta::Fp;

use vote_circuit::VoteCircuit;

#[test]
fn test_vote_circuit() {
    let max_votes = Fp::from(100);
    let circuit = VoteCircuit {
        vote: Some(Fp::from(42)),
        max_votes,
    };

    let prover = MockProver::run(8, &circuit, vec![]).unwrap();
    assert_eq!(prover.verify(), Ok(()));
}
