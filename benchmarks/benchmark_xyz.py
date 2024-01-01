import pathlib
import typing

import flour
import numpy as np
import pytest


@pytest.fixture
def structures() -> list[flour.XyzData]:
    num_atoms = 100
    rng = np.random.default_rng(11)
    return [
        flour.XyzData(
            comment="Test comment",
            elements=["C"]*num_atoms,
            positions=rng.random((num_atoms, 3)) * 100,
        )
        for _ in range(500)
    ]


@pytest.mark.benchmark(group="write_xyz")
def benchmark_write_xyz(
    benchmark: typing.Any,
    tmp_path: pathlib.Path,
    structures: list[flour.XyzData],
) -> None:
    benchmark(
        flour.write_xyz,
        path=tmp_path / "bench.xyz",
        xyz_structures=structures
    )


@pytest.mark.benchmark(group="read_xyz")
def benchmark_read_xyz(
    benchmark: typing.Any,
    tmp_path: pathlib.Path,
    structures: list[flour.XyzData],
) -> None:
    path = tmp_path / "bench.xyz"
    flour.write_xyz(
        path=path,
        xyz_structures=structures
    )
    benchmark(flour.read_xyz, path)
