//! Multiplier Circuit Demo
//!
//! Demonstrates a simple multiplier circuit: a × b = c
//! where c is public and a, b are private (zero-knowledge)

use groth16_circuits::multiplier::MultiplierCircuit;
use groth16_math::fields::FieldWrapper;
use groth16_qap::{check_divisibility, r1cs_to_qap, target_polynomial};

fn main() {
    println!("Groth16 Multiplier Circuit Demo");
    println!("===============================");
    println!();

    // Step 1: Create circuit with private inputs a=3, b=4, public output c=12
    println!("Step 1: Creating circuit");
    println!("--------------------");
    let a = 3u64;
    let b = 4u64;
    let c = 12u64;
    println!("Private inputs: a = {}, b = {}", a, b);
    println!("Public output:  c = {}", c);
    println!();

    let circuit = MultiplierCircuit::new(a, b, c);

    // Verify the computation is correct
    assert!(circuit.verify(), "Computation should be correct");
    println!("✓ Computation verified: {} × {} = {}", a, b, c);
    println!();

    // Step 2: Convert to R1CS constraints
    println!("Step 2: Converting to R1CS");
    println!("-------------------------");
    let constraints = circuit.to_r1cs();
    println!("Number of R1CS constraints: {}", constraints.len());
    println!(
        "Number of variables per constraint: {}",
        constraints[0].unique_variable_count()
    );

    // Show the constraint structure
    println!();
    println!("R1CS Constraint Structure:");
    println!("A = [0, 0, 1, 0]  (selects variable 'a')");
    println!("B = [0, 0, 0, 1]  (selects variable 'b')");
    println!("C = [0, 1, 0, 0]  (selects variable 'c')");
    println!("Verification: a · b = c");
    println!();

    // Step 3: Generate witness
    println!("Step 3: Generating witness");
    println!("--------------------------");
    let witness = circuit.witness();
    println!(
        "Witness assignment: [1, c, a, b] = [{}, {}, {}, {}]",
        witness[0].value, witness[1].value, witness[2].value, witness[3].value
    );
    println!();

    // Step 4: Verify R1CS satisfaction
    println!("Step 4: Verifying R1CS satisfaction");
    println!("-----------------------------------");
    let satisfied = constraints[0].is_satisfied(&witness);
    println!("R1CS constraint satisfied: {}", satisfied);
    assert!(satisfied, "Witness should satisfy the constraint");
    println!("✓ R1CS verification passed");
    println!();

    // Step 5: Convert R1CS to QAP
    println!("Step 5: Converting R1CS to QAP");
    println!("----------------------------");
    // Note: Need at least 2 constraints for QAP transformation
    // For this demo with 1 constraint, we'll show what would happen

    // Create a second constraint to enable QAP transformation
    // This demonstrates the QAP transformation with multiple constraints
    println!("Note: QAP transformation requires at least 2 constraints.");
    println!("For demonstration, let's create a second constraint...");
    println!();

    // Create a second identical constraint (same computation)
    let circuit2 = MultiplierCircuit::new(a, b, c);
    let constraints2 = circuit2.to_r1cs();
    let all_constraints = vec![constraints[0].clone(), constraints2[0].clone()];

    println!("Now we have {} constraints", all_constraints.len());

    // Perform R1CS to QAP transformation
    let num_variables = 4; // [1, c, a, b]
    let (a_polys, b_polys, c_polys) =
        r1cs_to_qap(&all_constraints, num_variables).expect("QAP transformation should succeed");

    println!("QAP polynomials created:");
    println!("  - {} A-polynomials (one per variable)", a_polys.len());
    println!("  - {} B-polynomials (one per variable)", b_polys.len());
    println!("  - {} C-polynomials (one per variable)", c_polys.len());
    println!();

    // Show polynomial evaluation at constraint points
    println!("Evaluating A[1] (polynomial for variable 'a'):");
    let x1 = FieldWrapper::<ark_bn254::Fr>::from(1u64);
    let x2 = FieldWrapper::<ark_bn254::Fr>::from(2u64);
    println!(
        "  A[1]({}) = {} (should be 1, coefficient in constraint 1)",
        1,
        a_polys[1].evaluate(&x1).value
    );
    println!(
        "  A[1]({}) = {} (should be 1, coefficient in constraint 2)",
        2,
        a_polys[1].evaluate(&x2).value
    );
    println!();

    // Step 6: Compute target polynomial
    println!("Step 6: Computing target polynomial");
    println!("----------------------------------");
    let num_constraints = all_constraints.len();
    let target = target_polynomial::<ark_bn254::Fr>(num_constraints);
    println!("Target polynomial t(x) = ∏ᵢ₌₁ⁿ (x - i)");
    println!("For n = {} constraints:", num_constraints);
    println!("  t(x) has degree {}", target.coeffs.len() - 1);
    println!("  t(1) = {} (should be 0)", target.evaluate(&x1).value);
    println!("  t(2) = {} (should be 0)", target.evaluate(&x2).value);
    println!();

    // Step 7: Check divisibility
    println!("Step 7: Checking divisibility");
    println!("----------------------------");
    let is_valid = check_divisibility(&witness, &a_polys, &b_polys, &c_polys, &target)
        .expect("Divisibility check should succeed");

    println!("Witness polynomial p(x) divisible by t(x): {}", is_valid);
    if is_valid {
        println!("✓ Divisibility check passed");
        println!();
        println!("This means:");
        println!("  - The witness satisfies all R1CS constraints");
        println!("  - The prover knows valid (a, b) for the public c");
        println!("  - Zero-knowledge proof can be generated");
    } else {
        println!("✗ Divisibility check failed");
    }
    println!();

    // Summary
    println!("=================================");
    println!("Demo Summary");
    println!("=================================");
    println!("Demonstrated the complete pipeline:");
    println!("  1. Circuit definition: a × b = c");
    println!("  2. R1CS constraints: a · b = c");
    println!("  3. Witness generation: [1, c, a, b]");
    println!("  4. R1CS satisfaction verified");
    println!("  5. QAP transformation (R1CS → polynomials)");
    println!("  6. Target polynomial computation");
    println!("  7. Divisibility checking (QAP verification)");
    println!();
    println!("Zero-Knowledge Property:");
    println!("  - The proof shows valid (a, b) exist for c = {}", c);
    println!("  - But does NOT reveal that a = {}, b = {}", a, b);
    println!("  - Verifier only learns that the computation is correct");
    println!();
    println!("Demo completed successfully!");
}
