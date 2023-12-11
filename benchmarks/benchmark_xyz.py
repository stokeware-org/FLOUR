import pathlib
import typing

import flour
import numpy as np
import pytest


def create_xyz_data(i: int):
    num_atoms = 100
    return flour.XyzData(
        comment="Test comment!",
        elements=['C']*num_atoms,
        positions=np.random.default_rng(i).random((num_atoms, 3)) * 100
    )


def create_xyz_structures():
    return [create_xyz_data(i) for i in range(500)]


@pytest.mark.benchmark(group="write_xyz")
def benchmark_write_xyz(
    benchmark: typing.Any,
    tmp_path: pathlib.Path,
) -> None:
    xyz_structures = create_xyz_structures()
    benchmark(
        flour.write_xyz,
        path=tmp_path / "bench.xyz",
        xyz_structures=xyz_structures
    )


@pytest.mark.benchmark(group="read_xyz")
def benchmark_read_xyz(
    benchmark: typing.Any,
    tmp_path: pathlib.Path,
) -> None:
    path = tmp_path / "bench.xyz"
    xyz_structures = create_xyz_structures()
    flour.write_xyz(
        path=path,
        xyz_structures=xyz_structures
    )
    benchmark(flour.read_xyz, path)
