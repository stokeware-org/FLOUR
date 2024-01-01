import pathlib

import flour
import numpy as np


def test_xyz(
    tmp_path: pathlib.Path,
) -> None:

    xyz_path = tmp_path / "molecule.xyz"
    comment = "Test comment!"
    elements = ['Pd', 'C', 'O', 'H']
    positions = np.array(
        [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ]
    )
    xyz_data_0 = flour.XyzData(
        comment=comment,
        elements=elements,
        positions=positions
    )
    xyz_data_1 = flour.XyzData(
        comment=comment+'!',
        elements=elements[::-1],
        positions=positions*-1
    )
    
    flour.write_xyz(
        path=xyz_path,
        xyz_structures=[xyz_data_0, xyz_data_1],
    )
    
    xyz_structures = flour.read_xyz(xyz_path)
    assert comment == xyz_structures[0].comment
    assert np.all(np.equal(elements, xyz_structures[0].elements))
    assert np.all(np.isclose(positions, xyz_structures[0].positions))
    
    assert xyz_data_1.comment == xyz_structures[1].comment
    assert np.all(np.equal(xyz_data_1.elements, xyz_structures[1].elements))
    assert np.all(np.isclose(xyz_data_1.positions, xyz_structures[1].positions))
