use core::array::ArrayTrait;
use starknet::testing::cheatcode;
use stark_vrf::ecvrf::{Point, Proof, ECVRF, ECVRFImpl};

#[derive(Drop, Serde)]
struct StarkVrfRequest {
    felt252_seed: Array<felt252>,
}
#[derive(Drop, Serde)]
struct StarkVrfProof {
    felt252_gamma_x: felt252,
    felt252_gamma_y: felt252,
    felt252_c: felt252,
    felt252_s: felt252,
    felt252_sqrt_ratio: felt252,
}
#[generate_trait]
impl StarkVrfOracle of StarkVrfOracleTrait {
    fn stark_vrf(arg: StarkVrfRequest) -> StarkVrfProof {
        let mut serialized = ArrayTrait::new();
        arg.serialize(ref serialized);
        let mut result = cheatcode::<'stark_vrf'>(serialized.span());
        Serde::deserialize(ref result).unwrap()
    }
}

fn proof_from_oracle(oracle_proof: StarkVrfProof) -> Proof {
    Proof {
        gamma: Point {
            x: oracle_proof.felt252_gamma_x,
            y: oracle_proof.felt252_gamma_y
        },
        c: oracle_proof.felt252_c,
        s: oracle_proof.felt252_s,
        sqrt_ratio_hint: oracle_proof.felt252_sqrt_ratio,
    }
}

pub fn request_vrf(seed: Span<felt252>) -> felt252 {
    let mut felt252_seed = ArrayTrait::new();
    felt252_seed.append_span(seed);

    let request = StarkVrfRequest { felt252_seed: felt252_seed };
    let proof = StarkVrfOracle::stark_vrf(request);
    let proof = proof_from_oracle(proof);

    let pk = Point {
        x: 2465182048640915825114623967805639036884813714770257338089158027381626459289,
        y: 3038635738014387716559859267483610492356329532552881764846792983975787300333
    }; 

    let ecvrf = ECVRFImpl::new(pk);

    ecvrf.verify(proof, seed).unwrap()
}
