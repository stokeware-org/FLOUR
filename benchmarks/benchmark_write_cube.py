import pathlib
import typing

import flour
import numpy as np
import pytest


@pytest.mark.benchmark(group="write_cube")
def benchmark_write_cube(
    benchmark: typing.Any,
    tmp_path: pathlib.Path,
) -> None:
    benchmark(
        flour.write_cube,
        path=tmp_path / "bench.cube",
        title1="first line",
        title2="second line",
        atoms=np.array([1, 35], dtype=np.uint8),
        charges=np.array([0.1, -0.5]),
        positions=np.array(
            [
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        ),
        voxel_origin=np.array([1.0, 2.0, 3.0]),
        voxel_size=np.array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ],
        ),
        voxels=np.random.default_rng(12).random((50, 50, 50)) * 100,
    )
