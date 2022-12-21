import pathlib
import typing

import flour


def benchmark_read_cube(benchmark: typing.Any, tmp_path: pathlib.Path) -> None:
    benchmark(flour.read_cube, 0.1)
