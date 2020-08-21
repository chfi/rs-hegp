mod utils;

use rand::prelude::*;

use nalgebra as na;

use nalgebra::{DMatrix, DVector, Matrix, Unit};

use wasm_bindgen::prelude::*;

fn rotation_mat(dim: usize, angle: f32, a: usize, b: usize) -> DMatrix<f32> {
    assert!(dim > 0 && a < dim && b < dim && a != b);
    let mut mat = DMatrix::identity(dim, dim);
    mat[(a, a)] = angle.cos();
    mat[(a, b)] = -angle.sin();
    mat[(b, a)] = angle.sin();
    mat[(b, b)] = angle.cos();
    mat
}

fn rand_rot_mat(dim: usize) -> DMatrix<f32> {
    assert!(dim > 0);
    let mut rng = thread_rng();
    let angle: f32 = rng.gen();
    let a: usize = rng.gen_range(0, dim);
    let mut b: usize = rng.gen_range(0, dim);
    while a == b {
        b = rng.gen_range(0, dim);
    }

    rotation_mat(dim, angle, a, b)
}

fn gen_key_series(dim: usize, len: usize) -> Vec<DMatrix<f32>> {
    (0..len).into_iter().map(|_| rand_rot_mat(dim)).collect()
}

fn gen_plaintext(rows: usize, cols: usize) -> DMatrix<f32> {
    DMatrix::new_random(rows, cols)
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[wasm_bindgen]
pub struct AnimState {
    keys: Vec<DMatrix<f32>>,
    size: Size,
    plaintext: DMatrix<f32>,
    current_matrix: DMatrix<f32>,
    pub current_index: usize,
}

#[wasm_bindgen]
impl AnimState {
    pub fn init(rows: usize, cols: usize, num_keys: usize) -> Self {
        let keys = gen_key_series(rows, num_keys);
        let plaintext = gen_plaintext(rows, cols);
        let current_matrix = plaintext.clone();
        let current_index = 0;
        let size = Size {
            width: cols,
            height: rows,
        };

        AnimState {
            keys,
            size,
            plaintext,
            current_matrix,
            current_index,
        }
    }

    pub fn next_step(&mut self) {
        // if we have keys 0..n,
        // and the current index is 0, we're at the plaintext.
        // So we encrypt with key 0.
        if self.current_index < self.keys.len() {
            let i = self.current_index;
            self.current_matrix = &self.keys[i] * &self.current_matrix;
            self.current_index += 1;
        }
    }

    pub fn prev_step(&mut self) {
        if self.current_index > 0 {
            let i = self.current_index;
            let inv = self.keys[i - 1].clone().try_inverse().unwrap();
            self.current_matrix = inv * &self.current_matrix;
            self.current_index -= 1;
        }
    }

    pub fn size(&self) -> Size {
        self.size.clone()
    }

    pub fn plaintext(&self) -> *const f32 {
        self.plaintext.as_slice().as_ptr()
    }

    pub fn current_matrix(&self) -> *const f32 {
        self.current_matrix.as_slice().as_ptr()
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hegp-rust-anim!");
}
