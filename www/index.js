import * as wasm from "hegp-rust-anim";
import { memory } from "hegp-rust-anim/hegp_rust_anim_bg";


const main = (dataWidth, dataHeight) => {
const canvas = document.getElementById("canvas");
const forwardButton = document.getElementById("forward");
const reverseButton = document.getElementById("reverse");
const stopButton = document.getElementById("stop");
const resetButton = document.getElementById("reset");
const prevButton = document.getElementById("prev");
const nextButton = document.getElementById("next");

  let mat_size = { width: dataWidth, height: dataHeight };
  let animState = wasm.AnimState.init(mat_size.width, mat_size.height, 5);

  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  ctx.imageSmoothingEnabled = false;

  let offscreen_canvas = wasm.new_canvas("offscreen", animState.width(), animState.height());

  let o_ctx = offscreen_canvas.getContext("2d");

  const draw = () => {
    animState.render(o_ctx);
    ctx.drawImage(offscreen_canvas, 0, 0, canvas.width, canvas.height);
  };

  const next = () => {
    animState.next_step();
    draw();
  };

  const prev = () => {
    animState.prev_step();
    draw();
  };

  let interval_delay = 1000;
  const reset = () => {
    pause();
    animState.reset();
    draw();
  }

  let interval_handle = null;

  const play_forward = () => {
    pause();
    next();
    let handle = setInterval(() => {
      next();
    }, interval_delay);
    interval_handle = handle;
  };

  const play_reverse = () => {
    pause();
    prev();
    let handle = setInterval(() => {
      prev();
    }, interval_delay);
    interval_handle = handle;
  }

  const pause = () => {
    clearInterval(interval_handle);
    interval_handle = null;
  };

  draw();


  forwardButton.addEventListener("click", play_forward);
  reverseButton.addEventListener("click", play_reverse);
  stopButton.addEventListener("click", pause);
  resetButton.addEventListener("click", reset);
  nextButton.addEventListener("click", next);
  prevButton.addEventListener("click", prev);


  window.animState = animState;
  window.next = next;
  window.prev = prev;

  window.play_forward = play_forward;
  window.play_reverse = play_reverse;
  window.pause = pause;
};

main(10, 10);
