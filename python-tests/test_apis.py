def test_eq(imgsize):
    assert imgsize.Size(1, 2, 'a', imgsize.Animation.Yes) == imgsize.Size(1, 2, 'a', imgsize.Animation.Yes)
    assert imgsize.Size(1, 2, 'a', imgsize.Animation.Yes) != imgsize.Size(2, 2, 'a', imgsize.Animation.Yes)
    assert imgsize.Size(1, 2, 'a', imgsize.Animation.Yes) != imgsize.Size(1, 1, 'a', imgsize.Animation.Yes)
    assert imgsize.Size(1, 2, 'a', imgsize.Animation.Yes) != imgsize.Size(1, 2, 'b', imgsize.Animation.Yes)
    assert imgsize.Size(1, 2, 'a', imgsize.Animation.Yes) != imgsize.Size(1, 2, 'a', imgsize.Animation.No)


def test_iter(imgsize):
    assert list(imgsize.Size(1, 2, 'a', imgsize.Animation.Yes)) == [1, 2]
