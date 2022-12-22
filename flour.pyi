import pathlib

import numpy as np
import numpy.typing as npt

class VoxelGrid:
    voxels: npt.NDArray[np.float64]
    origin: npt.NDArray[np.float64]
    voxel_size: npt.NDArray[np.float64]

class CubeData:
    atoms: npt.NDArray[np.uint8]
    charges: npt.NDArray[np.float64]
    positions: npt.NDArray[np.float64]
    grid: VoxelGrid

def read_cube(path: pathlib.Path | str) -> CubeData:
    pass

def write_cube(
    path: pathlib.Path | str,
    title1: str,
    title2: str,
    atoms: npt.NDArray[np.uint8],
    charges: npt.NDArray[np.float64],
    positions: npt.NDArray[np.float64],
    voxel_origin: npt.NDArray[np.float64],
    voxel_size: npt.NDArray[np.float64],
    voxels: npt.NDArray[np.float64],
) -> None:
    pass
