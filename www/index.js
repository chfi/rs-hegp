import * as wasm from "hegp-rust-anim";
import { memory } from "hegp-rust-anim/hegp_rust_anim_bg";

const DATA_COLS = 100;
const DATA_ROWS = 50;
const NKEYS = 500;

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

const initialize = (animState) => {
  console.log("mat_size");
  let mat_size = animState.size();

  // Comment the above and uncomment the following two lines to
  // let mat_size = { width: dataWidth, height: dataHeight };
  // let animState = wasm.AnimState.init_random(mat_size.width, mat_size.height, numKeys);

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
  nextButton.addEventListener("click", ev => {
    pause();
    next();
  });
  prevButton.addEventListener("click", ev => {
    pause();
    prev();
  });

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


const mainBxd = (numKeys) => {
  let animState = wasm.AnimState.init_bxd_chr1(numKeys);
  initialize(animState);
};

const mainRandom = (dataWidth, dataHeight, numKeys) => {
  let animState = wasm.AnimState.init_random(mat_size.width, mat_size.height, numKeys);
  initialize(animState);
};

mainBxd(NKEYS);
// mainRandom(DATA_COLS, DATA_ROWS, NKEYS);
