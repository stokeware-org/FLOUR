flour
=====

``flour`` is a Python library for really fast chemical file reading and writing.

Installation
------------

.. code-block:: bash

  pip install flour

``.cube`` files
---------------

.. code-block:: python

  import numpy as np
  import flour

  path = "my_cube.cube"
  # The atomic number of each atom.
  atoms = np.array([1, 35], dtype=np.uint8)
  # The charge on each atom.
  charges = np.array([0., 0.])
  # The position matrix of the molecule.
  positions = np.array(
      [-0.5, 0., 0.],
      [1., 0., 0.],
  )
  # The origin of the voxel grid.
  voxel_origin = np.array([-5.5, -5.5, -5.5])
  # The a, b and c vectors of a single voxel.
  voxel_size = np.array(
      [
          [0.5, 0., 0.],
          [0., 0.5, 0.],
          [0., 0., 0.5],
      ],
  )
  # A 20 x 20 x 20 grid of random voxels.
  voxels = np.random.rand(20, 20, 20)

  flour.write_cube(
      path=path,
      title1="The first line of the cube file",
      title2="The second line of the cube file",
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
