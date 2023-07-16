import red_python.model


def test_rock_name() -> None:
    assert red_python.model.GABBRO == "Gabbro"
    assert red_python.model.MARBLE == "Marble"
    assert red_python.model.METAMORPHIC == "Metamorphic"


def test_fruit_amount() -> None:
    assert red_python.model.APPLES_COUNT == 236
    assert red_python.model.ORANGES_COUNT == 454588979794318
