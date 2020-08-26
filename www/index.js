import * as wasm from "hegp-rust-anim";
import { memory } from "hegp-rust-anim/hegp_rust_anim_bg";

const main = (dataWidth, dataHeight) => {
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

  draw();

  window.animState = animState;
  window.next = next;
  window.prev = prev;
};

main(10, 10);
