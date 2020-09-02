mod utils;

use rand::prelude::*;

use std::time::Duration;

// use nalgebra as na;
use nalgebra::{DMatrix, DVector, Matrix, Unit};

use wasm_bindgen::{closure::Closure, prelude::*, JsCast};

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use colorous::{
    Gradient, CIVIDIS, COOL, CUBEHELIX, INFERNO, MAGMA, PLASMA, TURBO, VIRIDIS,
    WARM,
};

static GRADIENT_NAMES: [&'static str; 9] = [
    "CIVIDIS",
    "COOL",
    "CUBEHELIX",
    "INFERNO",
    "MAGMA",
    "PLASMA",
    "TURBO",
    "VIRIDIS",
    "WARM",
];

fn pick_gradient(name: &str) -> Option<Gradient> {
    match name {
        "CIVIDIS" => Some(CIVIDIS),
        "COOL" => Some(COOL),
        "CUBEHELIX" => Some(CUBEHELIX),
        "INFERNO" => Some(INFERNO),
        "MAGMA" => Some(MAGMA),
        "PLASMA" => Some(PLASMA),
        "TURBO" => Some(TURBO),
        "VIRIDIS" => Some(VIRIDIS),
        "WARM" => Some(WARM),
        _ => None,
    }
}

macro_rules! log {
    ( $( $t:tt )*) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(module = "/js/util.js")]
extern "C" {
    fn draw_bytes_to_canvas(
        img_bytes: &[u8],
        width: usize,
        height: usize,
        ctx: &CanvasRenderingContext2d,
    );
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setInterval)]
    fn set_interval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;

    #[wasm_bindgen(js_name = clearInterval)]
    fn clear_interval(handle: i32);
}

#[wasm_bindgen]
pub struct Interval {
    handle: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Interval {
    pub fn new<F>(millis: u32, f: F) -> Interval
    where
        F: FnMut() + 'static,
    {
        let _closure = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
        let handle = set_interval(&_closure, millis);
        Interval { _closure, handle }
    }

    pub fn handle(&self) -> i32 {
        self.handle
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        clear_interval(self.handle);
    }
}

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
pub fn new_canvas(
    id: &str,
    width: usize,
    height: usize,
) -> Result<HtmlCanvasElement, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.create_element("canvas")?;
    element.set_attribute("id", id)?;
    element.set_attribute("width", &width.to_string())?;
    element.set_attribute("height", &height.to_string())?;
    let canvas = element.dyn_into()?;
    Ok(canvas)
}

#[wasm_bindgen]
pub struct AnimState {
    keys: Vec<DMatrix<f32>>,
    data_size: Size,
    plaintext: DMatrix<f32>,
    current_matrix: DMatrix<f32>,
    pub current_index: usize,
    image_data: Vec<u8>,
    gradient: String,
}

fn render_image(gradient: &Gradient, data: &DMatrix<f32>) -> Vec<u8> {
    let mut result = vec![0; data.len() * 4];
    render_image_mut(gradient, data, &mut result);
    result
}

fn render_image_mut(
    gradient: &Gradient,
    data: &DMatrix<f32>,
    buf: &mut Vec<u8>,
) {
    assert!(buf.len() == data.len() * 4);
    for (i, val) in data.iter().enumerate() {
        let j = i * 4;
        let color = gradient.eval_continuous(*val as f64);
        buf[j] = color.r;
        buf[j + 1] = color.g;
        buf[j + 2] = color.b;
        buf[j + 3] = 255;
    }
}

#[wasm_bindgen]
impl AnimState {
    pub fn init(rows: usize, cols: usize, num_keys: usize) -> Self {
        let keys = gen_key_series(rows, num_keys);
        let plaintext = gen_plaintext(rows, cols);
        let current_matrix = plaintext.clone();
        let current_index = 0;
        let data_size = Size {
            width: cols,
            height: rows,
        };

        let gradient = "PLASMA".to_string();
        let image_data = render_image(&PLASMA, &current_matrix);

        AnimState {
            keys,
            data_size,
            plaintext,
            current_matrix,
            current_index,
            image_data,
            gradient,
        }
    }

    pub fn set_gradient(&mut self, name: &str) {
        if GRADIENT_NAMES.contains(&name) {
            self.gradient = name.to_string();
        } else {
            self.gradient = "PLASMA".to_string();
        }
    }

    fn gradient(&self) -> Gradient {
        let gradient = if let Some(gradient) = pick_gradient(&self.gradient) {
            gradient
        } else {
            TURBO
        };
        gradient
    }

    pub fn render_bytes(&mut self) {
        let gradient = self.gradient();
        render_image_mut(&gradient, &self.current_matrix, &mut self.image_data);
    }

    pub fn reset(&mut self) {
        self.current_matrix = self.plaintext.clone();
        self.current_index = 0;
        self.render_bytes();
    }

    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let bytes = self.image_data.as_slice();
        draw_bytes_to_canvas(
            bytes,
            self.data_size.width,
            self.data_size.height,
            ctx,
        );
    }

    pub fn next_step(&mut self) {
        // if we have keys 0..n,
        // and the current index is 0, we're at the plaintext.
        // So we encrypt with key 0.
        if self.current_index < self.keys.len() {
            let i = self.current_index;
            self.current_matrix = &self.keys[i] * &self.current_matrix;
            self.current_index += 1;
            self.render_bytes();
        }
    }

    pub fn prev_step(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            let i = self.current_index;
            let inv = self.keys[i].clone().try_inverse().unwrap();
            self.current_matrix = inv * &self.current_matrix;
            self.render_bytes();
        }
    }

    pub fn size(&self) -> Size {
        self.data_size.clone()
    }

    pub fn image_data(&self) -> *const u8 {
        self.image_data.as_slice().as_ptr()
    }

    pub fn image_data_len(&self) -> usize {
        self.image_data.len()
    }

    pub fn plaintext(&self) -> *const f32 {
        self.plaintext.as_slice().as_ptr()
    }

    pub fn plaintext_len(&self) -> usize {
        self.plaintext.len()
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
