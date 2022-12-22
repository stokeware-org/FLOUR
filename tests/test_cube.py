import pathlib

import flour
import numpy as np


def test_read_cube(
    tmp_path: pathlib.Path,
) -> None:

    cube_path = tmp_path / "molecule.cube"
    atoms = np.array([1, 10, 20, 30], dtype=np.uint8)
    charges = np.array([12.32, -15.2, 7.0, 0.2e-15])
    positions = np.array(
        [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ]
    )
    voxel_origin = np.array([0.15e-13, -0.3, 15.3])
    voxel_size = np.array(
        [
            [100.0, 101.0, 202.0],
            [303.0, 404.0, 505.0],
            [606.0, 707.0, 808.0],
        ]
    )
    generator = np.random.default_rng(12)
    voxels = generator.random((5, 7, 3))
    voxels[0, 0, 0] = 12.3e-15
    voxels[1, 0, 1] = 101.332e-13
    flour.write_cube(
        path=cube_path,
        title1="title1",
        title2="title2",
        atoms=atoms,
        charges=charges,
        positions=positions,
        voxel_origin=voxel_origin,
        voxel_size=voxel_size,
        voxels=voxels,
    )

    cube_data = flour.read_cube(cube_path)
    assert np.all(np.isclose(atoms, cube_data.atoms))
    assert np.all(np.isclose(charges, cube_data.charges))
    assert np.all(np.isclose(positions, cube_data.positions))
    assert np.all(np.isclose(voxel_origin, cube_data.grid.origin))
    assert np.all(np.isclose(voxel_size, cube_data.grid.voxel_size))
    assert np.all(np.isclose(voxels, cube_data.grid.voxels))
