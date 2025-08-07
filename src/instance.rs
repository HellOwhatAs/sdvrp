#![allow(dead_code)]

#[derive(Clone)]
pub enum InputFormat {
    DenseMatrix(Vec<Vec<i32>>),
    CoordList(Vec<(i32, i32)>),
}

impl InputFormat {
    pub fn to_str(&self) -> &'static str {
        match self {
            InputFormat::DenseMatrix(_) => "DENSE_MATRIX",
            InputFormat::CoordList(_) => "COORD_LIST",
        }
    }

    pub fn to_dense_matrix(&self) -> Vec<i32> {
        match self {
            InputFormat::DenseMatrix(matrix) => matrix.iter().flat_map(|row| row.clone()).collect(),
            InputFormat::CoordList(_) => vec![],
        }
    }

    pub fn to_coord_list_x(&self) -> Vec<i32> {
        match self {
            InputFormat::DenseMatrix(_) => vec![],
            InputFormat::CoordList(coords) => coords.iter().map(|(x, _)| *x).collect(),
        }
    }

    pub fn to_coord_list_y(&self) -> Vec<i32> {
        match self {
            InputFormat::DenseMatrix(_) => vec![],
            InputFormat::CoordList(coords) => coords.iter().map(|(_, y)| *y).collect(),
        }
    }
}

pub trait AlkaidInstance {
    fn capacity(&self) -> i32;
    fn demands(&self) -> &[i32];
    fn input_format(&self) -> &InputFormat;
}

pub struct Instance {
    capacity: i32,
    demands: Vec<i32>,
    input: InputFormat,
}

impl Instance {
    pub fn from_dense_matrix(capacity: i32, demands: Vec<i32>, matrix: Vec<Vec<i32>>) -> Self {
        assert!(
            demands.len() + 1 == matrix.len() && matrix.iter().all(|row| row.len() == matrix.len())
        );
        Self {
            capacity,
            demands,
            input: InputFormat::DenseMatrix(matrix),
        }
    }

    pub fn from_coord_list(capacity: i32, demands: Vec<i32>, coords: Vec<(i32, i32)>) -> Self {
        assert!(demands.len() + 1 == coords.len());
        Self {
            capacity,
            demands,
            input: InputFormat::CoordList(coords),
        }
    }
}

impl AlkaidInstance for Instance {
    fn capacity(&self) -> i32 {
        self.capacity
    }

    fn demands(&self) -> &[i32] {
        &self.demands
    }

    fn input_format(&self) -> &InputFormat {
        &self.input
    }
}
