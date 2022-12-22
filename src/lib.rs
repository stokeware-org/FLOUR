use std::{fs, path::PathBuf};

use itertools::izip;
use numpy::convert::IntoPyArray;
use numpy::ndarray::Axis;
use numpy::{PyArray1, PyArray2, PyArray3, PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3};
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

    let first_voxel_line_location = lines
        .next()
        .ok_or(PyRuntimeError::new_err("missing voxel line"))?
        .as_ptr() as usize;
    let (_, voxels) = contents.split_at(first_voxel_line_location - contents.as_ptr() as usize);
    let voxels: Vec<_> = voxels
        .split_ascii_whitespace()
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

#[pyfunction]
fn write_cube(
    path: PathBuf,
    title1: &str,
    title2: &str,
    atoms: PyReadonlyArray1<u8>,
    charges: PyReadonlyArray1<f32>,
    positions: PyReadonlyArray2<f32>,
    voxel_origin: PyReadonlyArray1<f32>,
    voxel_size: PyReadonlyArray2<f32>,
    voxels: PyReadonlyArray3<f32>,
) -> PyResult<()> {
    let atoms = atoms.as_array();
    let charges = charges.as_array();
    let positions = positions.as_array();
    let voxel_origin = voxel_origin.as_array();
    let voxel_size = voxel_size.as_array();
    let voxels = voxels.as_array();

    let mut content = format!(
        "{}\n{}\n\
        {: >5} {: >11.6} {: >11.6} {: >11.6}\n\
        {: >5} {: >11.6} {: >11.6} {: >11.6}\n\
        {: >5} {: >11.6} {: >11.6} {: >11.6}\n\
        {: >5} {: >11.6} {: >11.6} {: >11.6}\n",
        title1,
        title2,
        atoms.len(),
        voxel_origin[0],
        voxel_origin[1],
        voxel_origin[2],
        voxels.shape()[0],
        voxel_size[[0, 0]],
        voxel_size[[0, 1]],
        voxel_size[[0, 2]],
        voxels.shape()[1],
        voxel_size[[1, 0]],
        voxel_size[[1, 1]],
        voxel_size[[1, 2]],
        voxels.shape()[2],
        voxel_size[[2, 0]],
        voxel_size[[2, 1]],
        voxel_size[[2, 2]],
    );
    izip!(atoms, charges, positions.axis_iter(Axis(0))).for_each(|(atom, charge, position)| {
        content.push_str(&format!(
            "{: >5} {: >11.6} {: >11.6} {: >11.6} {: >11.6}\n",
            atom, charge, position[0], position[1], position[2],
        ))
    });
    let mut column_number = 0;
    for i in 0..voxels.shape()[0] {
        for j in 0..voxels.shape()[1] {
            for k in 0..voxels.shape()[2] {
                content.push_str(&format!(" {: >12.5E}", voxels[[i, j, k]]));
                column_number += 1;
                if column_number == 6 {
                    column_number = 0;
                    content.push('\n');
                }
            }
            content.push('\n');
            column_number = 0;
        }
    }
    fs::write(path, content)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn flour(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_cube, m)?)?;
    m.add_function(wrap_pyfunction!(write_cube, m)?)?;
    Ok(())
}
