import pathlib

import flour


def test_read_cube(
    tmp_path: pathlib.Path,
) -> None:

    cube_path = tmp_path / "molecule.cube"
    flour.read_cube(cube_path)
