mod utils;

use rand::prelude::*;

use std::time::Duration;

use std::thread;

// use nalgebra as na;
use nalgebra::{DMatrix, DVector, Matrix, Unit};

use wasm_bindgen::{closure::Closure, prelude::*, JsCast};

use wasm_bindgen_futures::spawn_local;

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
    fn render(
        img_bytes: &[u8],
        width: usize,
        height: usize,
        ctx: &CanvasRenderingContext2d,
    );

    fn play_forward(state: AnimState, millis: u32) -> i32;
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

#[derive(Clone, PartialEq, Eq)]
enum PlayState {
    PlayForward(i32),
    PauseForward,
    PlayReverse(i32),
    PauseReverse,
}

fn cancel_timeout(handle: i32) {
    let window = web_sys::window().unwrap();
    window.clear_timeout_with_handle(handle);
}

// fn set_timeout<F: FnOnce()>(f: timeout: i32) -> i32 {
//     let window = web_sys::window().unwrap();
//     window.
// }

impl PlayState {
    fn is_stopped(&self) -> bool {
        *self == PlayState::PauseForward || *self == PlayState::PauseReverse
    }

    fn is_playing(&self) -> bool {
        !self.is_stopped()
    }

    fn callback_id(&self) -> Option<i32> {
        use PlayState::*;
        match self {
            PlayForward(id) => Some(*id),
            PlayReverse(id) => Some(*id),
            _ => None,
        }
    }

    fn is_forward(&self) -> bool {
        use PlayState::*;
        match self {
            PlayForward(_) => true,
            PauseForward => true,
            _ => false,
        }
    }

    fn pause(self) -> Self {
        use PlayState::*;
        match self {
            PlayForward(id) => {
                cancel_timeout(id);
                PauseForward
            }
            PlayReverse(id) => {
                cancel_timeout(id);
                PauseReverse
            }
            x => x,
        }
    }

    fn play(self, callback_id: i32) -> Self {
        if self.is_forward() {
            PlayState::PlayForward(callback_id)
        } else {
            PlayState::PlayReverse(callback_id)
        }
    }

    /*
    fn play_forward<F>(&self, f: F) -> Self
    where
        F: FnOnce(),
    {

    }
        */
}

async fn delay_log_impl(text: String, millis: u32) {
    // let dur = Duration::from_millis(millis as u64);
    let dur = Duration::from_millis(500);
    // thread::sleep(dur);
    log!("{}", text);
}

// #[wasm_bindgen]
// pub fn delay_log(text: String, millis: u32) {
//     let cb = Closure::wrap
//     // spawn_local(delay_log_impl(text, millis));
//     // log!("after spawn");
// }

#[wasm_bindgen]
pub struct AnimState {
    keys: Vec<DMatrix<f32>>,
    data_size: Size,
    plaintext: DMatrix<f32>,
    current_matrix: DMatrix<f32>,
    pub current_index: usize,
    image_data: Vec<u8>,
    gradient: String,
    play_state: PlayState,
    interval: Option<Interval>,
}

fn render_image_mut(
    gradient: &Gradient,
    data: &DMatrix<f32>,
    buf: &mut Vec<u8>,
) {
    assert!(buf.len() == data.len() * 4);
    for (i, val) in data.iter().enumerate() {
        let j = i * 4;
        // let t: colorous::Gradient = TURBO;
        let color = gradient.eval_continuous(*val as f64);
        buf[j] = color.r;
        buf[j + 1] = color.g;
        buf[j + 2] = color.b;
        buf[j + 3] = 255;
    }
}

fn render_image(gradient: &Gradient, data: &DMatrix<f32>) -> Vec<u8> {
    let mut result = vec![0; data.len() * 4];
    render_image_mut(gradient, data, &mut result);
    result
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

        let gradient = "TURBO".to_string();

        let image_data = render_image(&TURBO, &current_matrix);

        let play_state = PlayState::PauseForward;

        AnimState {
            keys,
            data_size,
            plaintext,
            current_matrix,
            current_index,
            image_data,
            gradient,
            play_state,
            interval: None,
        }
    }

    pub fn temp_play(&self) {
        let interval = play_forward(self, 500);
    }

    fn fetch_gradient(&self) -> Gradient {
        let gradient = if let Some(gradient) = pick_gradient(&self.gradient) {
            gradient
        } else {
            TURBO
        };
        gradient
    }

    pub fn set_gradient(&mut self, name: &str) {
        if GRADIENT_NAMES.contains(&name) {
            self.gradient = name.to_string();
        } else {
            self.gradient = "TURBO".to_string();
        }
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        let bytes = self.image_data.as_slice();
        render(bytes, self.data_size.width, self.data_size.height, ctx);
    }

    pub fn pause(&mut self) {
        // TODO handle actually removing the callback
        if self.play_state.is_playing() {}
    }

    pub fn play_forward(&mut self) {
        // TODO handle removing the callback if applicable
        // TODO add the animation callback
    }

    pub fn next_step(&mut self) {
        // if we have keys 0..n,
        // and the current index is 0, we're at the plaintext.
        // So we encrypt with key 0.
        if self.current_index < self.keys.len() {
            let i = self.current_index;
            self.current_matrix = &self.keys[i] * &self.current_matrix;
            self.current_index += 1;
            let gradient = self.fetch_gradient();
            render_image_mut(
                &gradient,
                &self.current_matrix,
                &mut self.image_data,
            );
        }
    }

    pub fn prev_step(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            let i = self.current_index;
            let inv = self.keys[i].clone().try_inverse().unwrap();
            self.current_matrix = inv * &self.current_matrix;
            let gradient = self.fetch_gradient();
            render_image_mut(
                &gradient,
                &self.current_matrix,
                &mut self.image_data,
            );
        }
    }

    pub fn width(&self) -> usize {
        self.data_size.width
    }

    pub fn height(&self) -> usize {
        self.data_size.height
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
