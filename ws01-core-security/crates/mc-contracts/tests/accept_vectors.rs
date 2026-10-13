// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
use mc_contracts::round_trip_projection;
use serde_json::Value;
use std::collections::BTreeSet;

fn vectors() -> Vec<Value> {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../ARCHITECTURE/V2_1/TEST_VECTORS/wire-vectors.json"
    )))
    .expect("authoritative fixture JSON")
}

#[test]
fn all_85_projections_round_trip() {
    let mut names = BTreeSet::new();
    for vector in vectors()
        .into_iter()
        .filter(|vector| vector["expect"] == "ACCEPT_ENCODING")
    {
        let projection = vector["projection"].clone();
        let actual = round_trip_projection(projection.clone())
            .unwrap_or_else(|error| panic!("{}: {error}", vector["id"]));
        assert_eq!(actual, projection, "{}", vector["id"]);
        assert!(names.insert(vector["contract"].as_str().unwrap().to_owned()));
    }
    assert_eq!(names.len(), 85, "every registered contract needs a control");
    let registry: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json"
    )))
    .unwrap();
    assert_eq!(
        names,
        registry["contracts"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    );
}

#[test]
fn registered_field_order_matches_every_schema() {
    let registry: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json"
    )))
    .unwrap();
    let mut actual_names = mc_contracts::ALL_TYPE_NAMES.to_vec();
    actual_names.sort();
    let expected_names: Vec<_> = registry["contracts"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(actual_names, expected_names);
    for (name, contract) in registry["contracts"].as_object().unwrap() {
        let expected: Vec<_> = ["type", "wire_revision"]
            .into_iter()
            .chain(
                contract["fields"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|field| field["name"].as_str().unwrap()),
            )
            .collect();
        assert_eq!(mc_contracts::field_order(name).unwrap(), expected, "{name}");
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../ARCHITECTURE/V2_1/SCHEMAS")
            .join(format!("{name}.schema.json"));
        let schema: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(schema["x-cbor-order"], serde_json::json!(expected));
        assert_eq!(schema["required"], serde_json::json!(expected));
        assert_eq!(schema["additionalProperties"], false);
    }
}
