import * as wasm from "hegp-rust-anim";
import { memory } from "hegp-rust-anim/hegp_rust_anim_bg";

const render = (animState, ctx) => {
  let imgData = ctx.getImageData(0, 0, 10, 10);
  let anim_data_len = animState.image_data_len();
  let anim_data_ptr = animState.image_data();
  let anim_data = new Uint8Array(memory.buffer, anim_data_ptr, anim_data_len);
  let data = imgData.data;
  for (let i = 0; i < anim_data_len; i++) {
    data[i] = anim_data[i];
  }
  ctx.putImageData(imgData, 0, 0);
};

const main = () => {
  console.log("creating animstate");
  let animState = wasm.AnimState.init(10, 10, 5);

  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  ctx.imageSmoothingEnabled = false;

  let new_canvas = wasm.new_canvas("offscreen", 10, 10);

  window.offscreen_canvas = new_canvas;
  let o_ctx = new_canvas.getContext("2d");

  const next = () => {
    animState.next_step();
    render(animState, o_ctx);
    ctx.drawImage(new_canvas, 0, 0, 500, 500);
  };

  const prev = () => {
    animState.prev_step();
    render(animState, o_ctx);
    ctx.drawImage(new_canvas, 0, 0, 500, 500);
  };

  render(animState, o_ctx);

  ctx.drawImage(new_canvas, 0, 0, 500, 500);

  window.animState = animState;
  window.next = next;
  window.prev = prev;
};

main();
