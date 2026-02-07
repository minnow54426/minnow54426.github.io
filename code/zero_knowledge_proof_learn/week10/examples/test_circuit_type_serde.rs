use zk_proof_artifacts::CircuitType;

fn main() {
    // Test serialization
    let ct = CircuitType::Identity;
    let json = serde_json::to_string(&ct).unwrap();
    println!("Serialized Identity: {}", json);
    assert_eq!(json, "\"identity\"");

    // Test deserialization
    let deserialized: CircuitType = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {:?}", deserialized);
    assert_eq!(deserialized, CircuitType::Identity);

    // Test all variants
    println!("\nAll CircuitType variants:");
    for variant in [
        CircuitType::Identity,
        CircuitType::Membership,
        CircuitType::Privacy,
    ] {
        let json = serde_json::to_string(&variant).unwrap();
        println!("  {:?} -> {}", variant, json);
    }

    // Test FromStr
    use std::str::FromStr;
    println!("\nFromStr tests:");
    assert_eq!(
        CircuitType::from_str("identity").unwrap(),
        CircuitType::Identity
    );
    assert_eq!(
        CircuitType::from_str("IDENTITY").unwrap(),
        CircuitType::Identity
    );
    assert_eq!(
        CircuitType::from_str("Identity").unwrap(),
        CircuitType::Identity
    );
    assert_eq!(
        CircuitType::from_str("membership").unwrap(),
        CircuitType::Membership
    );
    assert_eq!(
        CircuitType::from_str("privacy").unwrap(),
        CircuitType::Privacy
    );

    // Test invalid input
    let result = CircuitType::from_str("invalid");
    assert!(result.is_err());
    println!("  Invalid input: {:?}", result.unwrap_err());

    // Test Display
    println!("\nDisplay tests:");
    println!("  Identity: {}", CircuitType::Identity);
    println!("  Membership: {}", CircuitType::Membership);
    println!("  Privacy: {}", CircuitType::Privacy);

    println!("\n✓ All serde tests passed!");
}
