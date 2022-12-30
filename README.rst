FLOUR
=====

``flour`` is a Python library for really fast chemical file reading and writing.

----

.. contents:: Table of contents

Installation
------------

.. code-block:: bash

  pip install flour

``.cube`` files
---------------

.. code-block:: python

  import numpy as np
  import flour

  # Write the cube file.
  flour.write_cube(
      path="molecule.cube",
      title1="The first line of the cube file",
      title2="The second line of the cube file",
      # The atomic number of each atom.
      atoms=np.array([1, 35], dtype=np.uint8),
      # The charge on each atom.
      charges=np.array([0., 0.]),
      # The position matrix of the molecule.
      positions=np.array([
          [-0.5, 0., 0.],
          [1., 0., 0.],
      ]),
      # The origin of the voxel grid.
      voxel_origin=np.array([-4.5, -4.5, -4.5]),
      # The a, b, c vectors of a single voxel.
      voxel_size=np.array(
          [
              [0.5, 0., 0.],
              [0., 0.5, 0.],
              [0., 0., 0.5],
          ],
      ),
      # A 20 x 20 x 20 voxel grid.
      voxels=np.random.rand(20, 20, 20),
  )

  # Read the cube file.
  cube_data = flour.read_cube("molecule.cube")
  cube_data.atoms  # The atomic number of each atom.
  cube_data.charges  # The charge of each atom.
  cube_data.positions  # The position matrix of the molecule.
  cube_data.grid.origin  # The origin of the voxel grid.
  cube_data.grid.voxel_size  # The a, b, c vectors of a single voxel.
  cube_data.grid.voxels  # The voxel grid.


Developer guide
---------------

If you want to develop ``FLOUR`` there's a couple of handy things you should know.
``FLOUR`` is built using the excellent maturin_ library. This means that to build
the library you run

.. code-block:: bash

  maturin develop --extras=dev --profile=release

and that's more or less all there is to it. The only other thing to note is that we do have
a justfile_, which contains commands you might find helpful for formatting and linting your code.
To use the justfile_ you should grab just_.

.. _maturin: https://github.com/PyO3/maturin
.. _justfile: justfile
.. _just: https://github.com/casey/just
