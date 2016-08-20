use std::cmp;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Matrix(Vec<Vec<f64>>);

macro_rules! matrix {
    ( $($($x:expr),+);* ) => ({
    	let mut tmp_vec: Vec<Vec<f64>> = Vec::new();
        $(
	        let mut tmp_vec_row: Vec<f64> = Vec::new();
        	$(
            	tmp_vec_row.push($x);
            )+
            tmp_vec.push(tmp_vec_row);
        )*
        Matrix(tmp_vec)
    });
}

#[allow(dead_code)]
impl Matrix {
	pub fn constant(constant: f64, rows: u64, cols: u64) -> Matrix {
		let v = vec![constant; cols as usize];
		let mut vec = Vec::with_capacity(rows as usize);
		for _ in 0..rows {
			vec.push(v.clone());
		}
		Matrix(vec)
	}

	pub fn zeros(rows: u64, cols: u64) -> Matrix {
		Matrix::constant(0.0, rows, cols)
	}

	pub fn ones(rows: u64, cols: u64) -> Matrix {
		Matrix::constant(1.0, rows, cols)
	}

	pub fn identity(rows: u64, cols: u64) -> Matrix {
		let mut mat = Matrix::zeros(rows, cols);
		for i in 0..cmp::min(rows, cols) {
			mat.set_inplace(1.0, i, i);
		}
		mat
	}

	pub fn size(&self) -> (u64, u64) {
		(self.0.len() as u64, self.0[0].len() as u64)
	}

	pub fn get(&self, row: u64, col: u64) -> f64 {
		self.0[row as usize][col as usize]
	}

	pub fn transpose(&self) -> Matrix {
		let (rows, cols) = self.size();
		let mut mat = Matrix::constant(0.0, rows, cols);
		for r in 0..rows {
			for c in 0..cols {
				mat.set_inplace(self.get(r, c), c, r);
			}
		}
		mat
	}

	pub fn scale(&self, scale: f64) -> Matrix {
		let (rows, cols) = self.size();
		let mut mat = self.clone();
		for r in 0..rows {
			for c in 0..cols {
				mat.set_inplace(scale * self.get(r, c), r, c);
			}
		}
		mat
	}

	pub fn offset(&self, constant: f64) -> Matrix {
		let (rows, cols) = self.size();
		let mut mat = self.clone();
		for r in 0..rows {
			for c in 0..cols {
				mat.set_inplace(constant + self.get(r, c), r, c);
			}
		}
		mat
	}

    pub fn dot_product(mat1: Matrix, mat2: Matrix) -> Matrix {
        let mut mat = Matrix::zeros(rows, cols);
        let (mat1_rows, mat1_cols)  = mat1.size();
        let (mat2_rows, mat2_cols)  = mat2.size();
        if mat1_rows != mat2_cols {
            return mat;
        }
        for r in 0..rows {
            for c in 0..cols {
                let mut product = mat1.get(r, c) * mat2.get(r, c);
                mat.set_inplace(product, r, c);
            }
        }
        return mat;
    }

	fn set_inplace(&mut self, val: f64, row: u64, col: u64) -> f64 {
		self.0[row as usize][col as usize] = val;
		val
	}
}

#[cfg(test)]
mod test {
	use super::*;

	 #[test]
    fn constant_test() {
    	let val = 2.15;
    	let rows = 5;
    	let cols = 4;
    	let m = Matrix::constant(val, rows, cols);
    	for r in 0..rows {
    		for c in 0..cols {
    			assert_eq!(val, m.get(r, c));
    		}
    	}
    	let rows = 2;
    	let cols = 12;
    	let m0 = Matrix::zeros(rows, cols);
    	for r in 0..rows {
    		for c in 0..cols {
    			assert_eq!(0.0, m0.get(r, c));
    		}
    	}
    	let rows = 11;
    	let cols = 1;
    	let m1 = Matrix::ones(rows, cols);
    	for r in 0..rows {
    		for c in 0..cols {
    			assert_eq!(1.0, m1.get(r, c));
    		}
    	}
    }

    #[test]
    fn identity_test() {
    	let rows = 17;
    	let cols = 13;
    	let m0 = Matrix::identity(rows, cols);
    	for r in 0..rows {
    		for c in 0..cols {
    			if r == c {
    				assert_eq!(1.0, m0.get(r, c));
    			} else {
    				assert_eq!(0.0, m0.get(r, c));
    			}
    		}
    	}
    }

    #[test]
    fn it_works() {
    	let m0 = Matrix::constant(0.0, 5, 4);
    	let m1 = Matrix::zeros(5, 4);
    	let m2 = Matrix::constant(1.2, 5, 4);
    	assert_eq!(m0, m1);
    	assert_eq!((5, 4), m1.size());
    	assert_eq!(1.2, m2.get(0, 0));
    	let m3 = m2.scale(10.0);
    	assert_eq!(12.0, m3.get(0, 0));
    	let m4 = Matrix::identity(2, 2);
    	assert_eq!(1.0, m4.get(0, 0));
    	assert_eq!(0.0, m4.get(1, 0));
    	let m5 = matrix![1.0, 2.0; 4.0, 3.0; 5.0, 6.0];
    	assert_eq!(2.0, m5.get(0, 1));
    	assert_eq!(3.0, m5.get(1, 1));
    }
}
