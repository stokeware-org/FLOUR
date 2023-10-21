import pathlib

import flour
import numpy as np


def test_xyz(
    tmp_path: pathlib.Path,
) -> None:

    xyz_path = tmp_path / "molecule.xyz"
    atoms = np.array([1, 10, 20, 30], dtype=np.uint8)
    positions = np.array(
        [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ]
    )
    flour.write_xyz(
        path=xyz_path,
        title1="title1",
        title2="title2",
        atoms=atoms,
        positions=positions,
    )

    xyz_data = flour.read_xyz(xyz_path)
    assert np.all(np.isclose(atoms, xyz_data.atoms))
    assert np.all(np.isclose(positions, xyz_data.positions))
