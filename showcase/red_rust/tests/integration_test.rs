use red_rust::model;

#[test]
fn test_rock_name() {
    assert_eq!(model::GABBRO, "Gabbro");
    assert_eq!(model::MARBLE, "Marble");
    assert_eq!(model::METAMORPHIC, "Metamorphic");
}

#[test]
fn test_fruit_amount() {
    assert_eq!(model::ORANGES_COUNT, 454588979794318);
    assert_eq!(model::APPLES_COUNT, 236);
}
