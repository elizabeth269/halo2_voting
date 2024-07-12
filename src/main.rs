mod circuit;

use circuit::VoteCircuit;
use halo2_proofs::{
    dev::MockProver,
    pasta::Fp,
    plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, ProvingKey, VerifyingKey},
    poly::commitment::Params,
    poly::commitment::*,
    transcript::{Blake2bWrite, Challenge255},
};
use rand::rngs::OsRng;

fn main() {
    // Define the maximum number of votes
    let max_votes = Fp::from(100);

    // Create an instance of the vote circuit
    let circuit = VoteCircuit {
        vote: Some(Fp::from(42)),
        max_votes,
    };

    // Generate proving and verifying keys
    let params: Blake2bparams<Fp> = Params::new(8);
    let vk = keygen_vk(&params, &circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk.clone(), &circuit).expect("keygen_pk should not fail");

    // Create a proof
    let mut transcript = Blake2bWrite::<_, _, Challenge255<Fp>>::init(vec![]);
    create_proof(&params, &pk, &[circuit], &[&[]], OsRng, &mut transcript)
        .expect("proof generation should not fail");

    let proof = transcript.finalize();

    // Verify the proof
    let mut transcript = TranscriptReaderBuffer::init(&proof[..]);
    assert!(verify_proof(&params, &vk, &mut transcript, &[&[]],).is_ok());

    println!("Proof verified successfully!");
}
