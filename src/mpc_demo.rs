use bellperson::{gadgets::num::AllocatedNum, groth16, Circuit, ConstraintSystem, SynthesisError};
use bls12_381::{Bls12, Fr};
use ff::Field;

// 定义一个简单的 Circuit
struct SimpleCircuit {
    a: Option<Fr>,
    b: Option<Fr>,
}

impl<F: Field> Circuit<F> for SimpleCircuit {
    fn synthesize<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // 分配输入
        let a = AllocatedNum::alloc(cs, || {
            let value = self.a.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(value)
        })?;

        let b = AllocatedNum::alloc(cs, || {
            let value = self.b.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(value)
        })?;

        // 创建约束：a * b = c
        let c = a.mul(cs, &b)?;

        // 约束条件：c = 42
        let const_42 = AllocatedNum::alloc(cs, || Ok(Fr::from(42u32)))?;
        cs.enforce(
            || "c equals 42",
            |lc| lc + c.get_variable(),
            |lc| lc,
            |lc| lc + const_42.get_variable(),
        );

        Ok(())
    }
}

fn main() {
    // 创建一个 Bls12 pairing
    let rng = &mut rand::thread_rng();
    let params = {
        let c = SimpleCircuit { a: None, b: None };
        groth16::generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let a = Fr::from(3u32);
    let b = Fr::from(14u32);

    // 创建证明
    let proof = {
        let c = SimpleCircuit {
            a: Some(a),
            b: Some(b),
        };
        groth16::create_random_proof(c, &params, rng).unwrap()
    };

    let pvk = groth16::prepare_verifying_key(&params.vk);
    let mut public_inputs = vec![Fr::from(42u32)];

    // 验证证明
    assert!(groth16::verify_proof(&pvk, &proof, &public_inputs).unwrap());
}
