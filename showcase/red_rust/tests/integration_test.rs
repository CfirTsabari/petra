use red_rust::model;

#[test]
fn test_rock_name() {
    assert_eq!(model::GABBRO, "Gabbro");
    assert_eq!(model::MARBLE, "Marble");
    assert_eq!(model::METAMORPHIC, "Metamorphic");
}
