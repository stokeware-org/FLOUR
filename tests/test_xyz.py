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
    flour.write_xyz(
        path=xyz_path,
        comment=comment,
        elements=elements,
        positions=positions,
    )
    
    xyz_data = flour.read_xyz(xyz_path)
    assert comment == xyz_data[0].comment
    assert np.all(np.equal(elements, xyz_data[0].elements))
    assert np.all(np.isclose(positions, xyz_data[0].positions))
    
    # test reading an XYZ file with multiple structures
    multi_xyz_path = pathlib.Path('C:/Users/Joshua/OneDrive/Desktop/iqmol_scratch/conformers_4_dft.xyz')
    multi_xyz_data = flour.read_xyz(multi_xyz_path)
    assert (len(multi_xyz_data)) == 28
    assert multi_xyz_data[0].comment == '-33286.03927246263'
    for i, current_xyz_data in enumerate(multi_xyz_data):
        assert len(current_xyz_data.elements) == 61
    assert multi_xyz_data[27].elements[60] == 'H'
    assert np.isclose(multi_xyz_data[27].positions[60,2], -3.171669)
