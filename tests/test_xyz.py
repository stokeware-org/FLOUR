import pathlib

import flour
import numpy as np


def test_xyz(
    tmp_path: pathlib.Path,
) -> None:

    xyz_path = tmp_path / "molecule.xyz"
    comment = "comment"
    elements = ['Pd', 'C', 'O', 'H']
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
        comment=comment,
        elements=elements,
        positions=positions,
    )
    assert True
    assert xyz_path.exists()
    with open(xyz_path) as xyz_file:
        assert xyz_file.readline() == '4\n'
        assert xyz_file.readline() == f'{comment}\n'
        assert xyz_file.readline() == 'Pd    1.000000    2.000000    3.000000 \n'
    xyz_data = flour.read_xyz(xyz_path)
    return
    assert comment == xyz_data.comment
    assert np.all(np.isclose(elements, xyz_data.elements))
    assert np.all(np.isclose(positions, xyz_data.positions))
