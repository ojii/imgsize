def test_eq(imgsize):
    assert imgsize.Size(1, 2, "a", True) == imgsize.Size(1, 2, "a", True)
    assert imgsize.Size(1, 2, "a", True) != imgsize.Size(2, 2, "a", True)
    assert imgsize.Size(1, 2, "a", True) != imgsize.Size(1, 1, "a", True)
    assert imgsize.Size(1, 2, "a", True) != imgsize.Size(1, 2, "b", True)
    assert imgsize.Size(1, 2, "a", True) != imgsize.Size(1, 2, "a", False)


def test_iter(imgsize):
    assert list(imgsize.Size(1, 2, "a", True)) == [1, 2]
