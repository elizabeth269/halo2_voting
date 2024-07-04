use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Chip, Layouter, SimpleFloorPlanner},
    plonk::{Circuit, ConstraintSystem, Error},
    poly::Rotation,
};

#[derive(Clone, Debug)]
struct VoteCircuit<F: FieldExt> {
    pub vote: Option<F>,
    pub max_votes: F,
}

impl<F: FieldExt> Circuit<F> for VoteCircuit<F> {
    type Config = ();
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            vote: None,
            max_votes: self.max_votes,
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let vote = meta.advice_column();
        let max_votes = meta.fixed_column();

        meta.enable_equality(vote);
        meta.enable_equality(max_votes);

        meta.create_gate("vote within range", |meta| {
            let vote = meta.query_advice(vote, Rotation::cur());
            let max_votes = meta.query_fixed(max_votes, Rotation::cur());

            vec![vote.clone() * (vote - max_votes.clone())]
        });

        ()
    }

    fn synthesize(&self, cs: &mut impl Layouter<F>, _: Self::Config) -> Result<(), Error> {
        cs.assign_region(
            || "assign vote",
            |mut region| {
                let vote =
                    region.assign_advice(|| "vote", 0, 0, || self.vote.ok_or(Error::Synthesis))?;

                let max_votes = region.assign_fixed(|| "max votes", 0, 0, || Ok(self.max_votes))?;

                region.constrain_equal(vote.cell(), max_votes.cell())?;
                Ok(())
            },
        )
    }
}
