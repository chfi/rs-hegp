mod utils;

use rand::prelude::*;

use nalgebra::{DMatrix, RowDVector};

use wasm_bindgen::{closure::Closure, prelude::*, JsCast};

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use colorous::{
    Gradient, CIVIDIS, COOL, CUBEHELIX, INFERNO, MAGMA, PLASMA, RAINBOW,
    SINEBOW, TURBO, VIRIDIS, WARM,
};

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )*) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

static GRADIENT_NAMES: [&str; 11] = [
    "CIVIDIS",
    "COOL",
    "CUBEHELIX",
    "INFERNO",
    "MAGMA",
    "PLASMA",
    "TURBO",
    "VIRIDIS",
    "WARM",
    "SINEBOW",
    "RAINBOW",
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
        "SINEBOW" => Some(SINEBOW),
        "RAINBOW" => Some(RAINBOW),
        _ => None,
    }
}

fn load_csv(string: &str) -> DMatrix<f32> {
    let mut lines_iter = string.split(|b| b == '\n');
    let header = lines_iter.next().unwrap();
    let _cols = header.len();
    let rows: Vec<RowDVector<f32>> = lines_iter
        .filter_map(|bs| {
            let line: Vec<_> = bs.split(|b| b == ',').collect();
            if line.len() <= 1 {
                None
            } else {
                let len = line.len();
                Some(RowDVector::from_iterator(
                    len,
                    line.iter().map(|s| {
                        let val = s.parse::<f32>().unwrap();
                        val / 2.0
                    }),
                ))
            }
        })
        .collect();
    let matrix = DMatrix::from_rows(rows.as_slice());
    matrix
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

fn rotation_matrix(dim: usize, angle: f32, a: usize, b: usize) -> DMatrix<f32> {
    assert!(dim > 0 && a < dim && b < dim && a != b);
    let mut mat = DMatrix::identity(dim, dim);
    mat[(a, a)] = angle.cos();
    mat[(a, b)] = -angle.sin();
    mat[(b, a)] = angle.sin();
    mat[(b, b)] = angle.cos();
    mat
}

fn random_rotation_matrix(dim: usize) -> DMatrix<f32> {
    assert!(dim > 0);
    let mut rng = thread_rng();
    let angle: f32 = rng.gen();
    let a: usize = rng.gen_range(0, dim);
    let mut b: usize = rng.gen_range(0, dim);
    while a == b {
        b = rng.gen_range(0, dim);
    }

    rotation_matrix(dim, angle, a, b)
}

fn generate_key_series(dim: usize, len: usize) -> Vec<DMatrix<f32>> {
    (0..len).map(|_| random_rotation_matrix(dim)).collect()
}

fn generate_plaintext(rows: usize, cols: usize) -> DMatrix<f32> {
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
    ciphertext: DMatrix<f32>,
    current_matrix: DMatrix<f32>,
    current_index: usize,
    image_data: Vec<u8>,
    gradient: String,
}

fn render_image_new(gradient: &Gradient, data: &DMatrix<f32>) -> Vec<u8> {
    log!("data len: {}", data.len());
    let mut result = vec![255; data.len() * 4];
    render_image_mut(gradient, data, &mut result);
    result
}

fn render_image_mut(
    gradient: &Gradient,
    data: &DMatrix<f32>,
    buf: &mut Vec<u8>,
) {
    assert!(buf.len() == data.len() * 4);
    let transposed = data.transpose();
    for (i, val) in transposed.iter().enumerate() {
        let j = i * 4;
        let color = gradient.eval_continuous(*val as f64);
        buf[j] = color.r;
        buf[j + 1] = color.g;
        buf[j + 2] = color.b;
    }
}

#[wasm_bindgen]
impl AnimState {
    fn init_with_plaintext(plaintext: DMatrix<f32>, num_keys: usize) -> Self {
        let (rows, cols) = plaintext.shape();

        let keys: Vec<_> = generate_key_series(rows, num_keys);
        let total_key = keys
            .iter()
            .fold(DMatrix::identity(rows, rows), |a, b| b * a);

        let ciphertext = total_key * &plaintext;
        let current_matrix = plaintext.clone();
        let current_index = 0;
        let data_size = Size {
            width: cols,
            height: rows,
        };

        let gradient = "RAINBOW".to_string();
        let image_data = render_image_new(&RAINBOW, &current_matrix);

        AnimState {
            keys,
            data_size,
            plaintext,
            ciphertext,
            current_matrix,
            current_index,
            image_data,
            gradient,
        }
    }

    pub fn init_bxd_chr1(num_keys: usize) -> Self {
        let plaintext = load_csv(include_str!("../chr1_sm.csv"));
        Self::init_with_plaintext(plaintext, num_keys)
    }

    pub fn init_random(rows: usize, cols: usize, num_keys: usize) -> Self {
        let plaintext = generate_plaintext(rows, cols);
        Self::init_with_plaintext(plaintext, num_keys)
    }

    pub fn set_gradient(&mut self, name: &str) {
        if GRADIENT_NAMES.contains(&name) {
            self.gradient = name.to_string();
        } else {
            self.gradient = "RAINBOW".to_string();
        }
    }

    fn gradient(&self) -> Gradient {
        if let Some(gradient) = pick_gradient(&self.gradient) {
            gradient
        } else {
            RAINBOW
        }
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

    pub fn goto_end(&mut self) {
        self.current_matrix = self.ciphertext.clone();
        self.current_index = self.keys.len();
        self.render_bytes();
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
