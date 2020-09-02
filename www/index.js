import * as wasm from "hegp-rust-anim";
import { memory } from "hegp-rust-anim/hegp_rust_anim_bg";

const DATA_COLS = 10;
const DATA_ROWS = 10;
const NKEYS = 100;

const canvas = document.getElementById("canvas");
const forwardButton = document.getElementById("forward");
const reverseButton = document.getElementById("reverse");
const stopButton = document.getElementById("stop");
const resetButton = document.getElementById("reset");
const endButton = document.getElementById("end");
const prevButton = document.getElementById("prev");
const nextButton = document.getElementById("next");

const delayRange = document.getElementById("delay");

const readRange = () => {
  return delayRange.valueAsNumber;
};


const main = (dataWidth, dataHeight, numKeys) => {
  let mat_size = { width: dataWidth, height: dataHeight };
  let animState = wasm.AnimState.init(mat_size.width, mat_size.height, numKeys);
  let dataSize = animState.size();

  let ctx = canvas.getContext("2d");
  ctx.imageSmoothingEnabled = false;

  let offscreen_canvas = wasm.new_canvas("offscreen", dataSize.width, dataSize.height);

  let o_ctx = offscreen_canvas.getContext("2d");

  let forward = true;

  const draw = () => {
    animState.draw(o_ctx);
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

  const reset = () => {
    pause();
    animState.reset();
    draw();
  };

  const gotoEnd = () => {
    pause();
    animState.goto_end();
    draw();
  };

  let interval_handle = null;

  const play_forward = () => {
    pause();
    next();
    forward = true;
    let handle = setInterval(() => {
      next();
    }, readRange());
    interval_handle = handle;
  };

  const play_reverse = () => {
    pause();
    prev();
    forward = false;
    let handle = setInterval(() => {
      prev();
    }, readRange());
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
  endButton.addEventListener("click", gotoEnd);
  nextButton.addEventListener("click", next);
  prevButton.addEventListener("click", prev);

  delayRange.addEventListener("change", ev => {
    if (interval_handle !== null) {
      clearInterval(interval_handle);
      if (forward) {
        play_forward();
      } else {
        play_reverse();
      }
    }
  });


  window.animState = animState;
  window.next = next;
  window.prev = prev;

  window.play_forward = play_forward;
  window.play_reverse = play_reverse;
  window.pause = pause;
  window.readRange = readRange;
};

main(DATA_COLS, DATA_ROWS, NKEYS);
