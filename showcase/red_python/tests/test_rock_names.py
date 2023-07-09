import red_python.model


def test_rock_name() -> None:
    assert red_python.model.GABBRO == "Gabbro"
    assert red_python.model.MARBLE == "Marble"
    assert red_python.model.METAMORPHIC == "Metamorphic"
