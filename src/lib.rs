use std::{fs, path::PathBuf};

use numpy::convert::IntoPyArray;
use numpy::{PyArray1, PyArray2, PyArray3};
use pyo3::{exceptions::PyRuntimeError, prelude::*};

#[pyclass]
struct VoxelGrid {
    #[pyo3(get)]
    voxels: Py<PyArray3<f32>>,
    #[pyo3(get)]
    origin: Py<PyArray1<f32>>,
    #[pyo3(get)]
    voxel_size: Py<PyArray2<f32>>,
}

#[pyclass]
struct CubeData {
    #[pyo3(get)]
    atoms: Py<PyArray1<u8>>,
    #[pyo3(get)]
    charges: Py<PyArray1<f32>>,
    #[pyo3(get)]
    positions: Py<PyArray2<f32>>,
    #[pyo3(get)]
    grid: Py<VoxelGrid>,
}

/// Read a `.cube` file.
#[pyfunction]
fn read_cube(py: Python, path: PathBuf) -> PyResult<CubeData> {
    let contents = fs::read_to_string(path)?;
    let mut lines = contents.lines().skip(2); // drop the title lines
    let third_line = lines
        .next()
        .ok_or(PyRuntimeError::new_err("cube file is missing lines"))?;
    let mut third_line_words = third_line.split_ascii_whitespace();
    let num_atoms = third_line_words
        .next()
        .ok_or(PyRuntimeError::new_err(
            "cube file does not define the number of atoms",
        ))?
        .parse::<usize>()?;
    let origin: Vec<f32> = third_line_words
        .map(|word| word.parse::<f32>().unwrap())
        .collect();

    let mut voxel_size = Vec::with_capacity(9);
    let num_voxels_x = parse_voxel_line(&mut lines, &mut voxel_size)?;
    let num_voxels_y = parse_voxel_line(&mut lines, &mut voxel_size)?;
    let num_voxels_z = parse_voxel_line(&mut lines, &mut voxel_size)?;

    let mut atoms: Vec<u8> = Vec::with_capacity(num_atoms);
    let mut charges: Vec<f32> = Vec::with_capacity(num_atoms);
    let mut positions = Vec::with_capacity(num_atoms * 3);
    for _ in 0..num_atoms {
        let atom_line = lines.next().ok_or(PyRuntimeError::new_err(
            "cube file is missing atom definition line",
        ))?;
        let mut words = atom_line.split_ascii_whitespace();
        atoms.push(
            words
                .next()
                .ok_or(PyRuntimeError::new_err(
                    "cube file is missing atomic number",
                ))?
                .parse::<u8>()?,
        );
        charges.push(
            words
                .next()
                .ok_or(PyRuntimeError::new_err("cube file is missing charge"))?
                .parse::<f32>()?,
        );
        positions.extend(words.map(|word| word.parse::<f32>().unwrap()));
    }

    let voxels: Vec<_> = contents
        .split_ascii_whitespace()
        .skip(69)
        // TODO: is there a way to just use split_ascii_whitespace without having to go over lines
        // and is that faster?
        // .flat_map(str::split_ascii_whitespace)
        .map(|word| word.parse::<f32>().unwrap())
        .collect();
    Ok(CubeData {
        atoms: atoms.into_pyarray(py).to_owned(),
        charges: charges.into_pyarray(py).to_owned(),
        positions: positions
            .into_pyarray(py)
            .reshape([num_atoms, 3])?
            .to_owned(),
        grid: Py::new(
            py,
            VoxelGrid {
                voxels: voxels
                    .into_pyarray(py)
                    .reshape([num_voxels_x, num_voxels_y, num_voxels_z])?
                    .to_owned(),
                origin: origin.into_pyarray(py).to_owned(),
                voxel_size: voxel_size.into_pyarray(py).reshape([3, 3])?.to_owned(),
            },
        )?,
    })
}

#[inline]
fn parse_voxel_line<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    voxel_size: &mut Vec<f32>,
) -> PyResult<usize> {
    let line = lines.next().ok_or(PyRuntimeError::new_err(
        "cube file is missing voxel definition line",
    ))?;
    let mut words = line.split_ascii_whitespace();
    let num_voxels = words
        .next()
        .ok_or(PyRuntimeError::new_err(
            "cube file is missing number of voxels",
        ))?
        .parse::<usize>()?;
    voxel_size.extend(words.map(|word| word.parse::<f32>().unwrap()));
    Ok(num_voxels)
}

/// A Python module implemented in Rust.
#[pymodule]
fn flour(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_cube, m)?)?;
    Ok(())
}
