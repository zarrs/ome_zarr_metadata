use serde::{Deserialize, Serialize, ser::SerializeSeq};
use std::ops::{Index, IndexMut};

/// Lexicographic ("C") order 2D array representation.
#[derive(Debug, Clone)]
pub struct Matrix {
    values: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Serialize for Matrix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut row_ser = serializer.serialize_seq(Some(self.rows))?;
        for row in self.iter_rows() {
            row_ser.serialize_element(row)?;
        }
        row_ser.end()
    }
}

impl<'de> Deserialize<'de> for Matrix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec_of_vecs: Vec<Vec<f64>> = Vec::deserialize(deserializer)?;
        Self::from_vecs(&vec_of_vecs).map_err(serde::de::Error::custom)
    }
}

impl Matrix {
    pub fn new(values: Vec<f64>, rows: usize, cols: usize) -> crate::Result<Self> {
        if values.len() != rows * cols {
            return Err(crate::Error::InvalidMatrix {
                numel: values.len(),
                rows,
                cols,
            });
        }
        Ok(Self { values, rows, cols })
    }

    pub fn full(value: f64, rows: usize, cols: usize) -> Self {
        Self {
            values: vec![value; rows * cols],
            rows,
            cols,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    fn from_vecs(vecs: &[Vec<f64>]) -> crate::Result<Self> {
        let rows = vecs.len();
        let mut viter = vecs.iter();
        let Some(first) = viter.next() else {
            return Err(crate::Error::InvalidMatrix {
                numel: 0,
                rows: 0,
                cols: 0,
            });
        };
        let cols = first.len();
        let mut vals = Vec::with_capacity(rows * cols);
        vals.extend_from_slice(first);
        for row in viter {
            if row.len() != cols {
                return Err(crate::Error::InvalidMatrix {
                    numel: vals.len() + row.len(),
                    rows,
                    cols: row.len(),
                });
            }
            vals.extend_from_slice(row);
        }
        Ok(Self {
            values: vals,
            rows,
            cols,
        })
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&f64> {
        self.values.get(row * self.cols + col)
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[f64]> {
        self.values.chunks(self.cols)
    }

    /// Whether the matrix is square.
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    /// Determinant of the matrix, or `None` if it is not square.
    ///
    /// Computed by LU decomposition with partial pivoting, in `O(n^3)`.
    /// The determinant of a 0x0 matrix is 1.
    pub fn determinant(&self) -> Option<f64> {
        if !self.is_square() {
            return None;
        }
        let n = self.rows;
        let mut a = self.values.clone();
        let mut det = 1.0;

        for col in 0..n {
            // partial pivoting: choose the row with the largest absolute value in this column
            let pivot = (col..n)
                .max_by(|&i, &j| a[i * n + col].abs().total_cmp(&a[j * n + col].abs()))
                .expect("non-empty range");
            let pivot_val = a[pivot * n + col];
            if pivot_val == 0.0 {
                return Some(0.0);
            }
            if pivot != col {
                for k in 0..n {
                    a.swap(col * n + k, pivot * n + k);
                }
                det = -det;
            }
            det *= pivot_val;

            for row in (col + 1)..n {
                let factor = a[row * n + col] / pivot_val;
                if factor == 0.0 {
                    continue;
                }
                for k in col..n {
                    a[row * n + k] -= factor * a[col * n + k];
                }
            }
        }
        Some(det)
    }

    /// Whether the matrix is orthogonal, i.e. square with orthonormal rows and columns
    /// (`M * M^T = I`), to within an absolute tolerance on each element.
    pub fn is_orthogonal(&self, tolerance: f64) -> bool {
        if !self.is_square() {
            return false;
        }
        let n = self.rows;
        // (M * M^T)[i][j] is the dot product of rows i and j
        for i in 0..n {
            for j in i..n {
                let dot: f64 = (0..n).map(|k| self[(i, k)] * self[(j, k)]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                if (dot - expected).abs() > tolerance {
                    return false;
                }
            }
        }
        true
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).expect("Index out of bounds")
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = index.0 * self.cols + index.1;
        &mut self.values[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn det_non_square() {
        let m = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
        assert_eq!(m.determinant(), None);
    }

    #[test]
    fn det_2x2() {
        let m = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2).unwrap();
        assert!(approx(m.determinant().unwrap(), -2.0));
    }

    #[test]
    fn det_3x3_needs_pivot() {
        // zero in the top-left forces a row swap
        let m = Matrix::new(vec![0.0, 2.0, 1.0, 1.0, 0.0, 3.0, 4.0, 1.0, 0.0], 3, 3).unwrap();
        // 0*(0-3) - 2*(0-12) + 1*(1-0) = 25
        assert!(approx(m.determinant().unwrap(), 25.0));
    }

    #[test]
    fn det_singular() {
        let m = Matrix::new(vec![1.0, 2.0, 2.0, 4.0], 2, 2).unwrap();
        assert!(approx(m.determinant().unwrap(), 0.0));
    }

    #[test]
    fn det_rotation() {
        let (s, c) = 0.3_f64.sin_cos();
        let m = Matrix::new(vec![c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0], 3, 3).unwrap();
        assert!(approx(m.determinant().unwrap(), 1.0));
    }

    #[test]
    fn orthogonal_rotation() {
        let (s, c) = 0.3_f64.sin_cos();
        let m = Matrix::new(vec![c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0], 3, 3).unwrap();
        assert!(m.is_orthogonal(1e-10));
    }

    #[test]
    fn orthogonal_reflection() {
        let m = Matrix::new(vec![1.0, 0.0, 0.0, -1.0], 2, 2).unwrap();
        assert!(m.is_orthogonal(1e-10));
    }

    #[test]
    fn not_orthogonal_scaled() {
        let m = Matrix::new(vec![2.0, 0.0, 0.0, 1.0], 2, 2).unwrap();
        assert!(!m.is_orthogonal(1e-10));
    }

    #[test]
    fn not_orthogonal_sheared() {
        let m = Matrix::new(vec![1.0, 1.0, 0.0, 1.0], 2, 2).unwrap();
        assert!(!m.is_orthogonal(1e-10));
    }

    #[test]
    fn not_orthogonal_non_square() {
        let m = Matrix::new(vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0], 2, 3).unwrap();
        assert!(!m.is_orthogonal(1e-10));
    }
}
