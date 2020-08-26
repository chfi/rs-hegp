import * as wasm from "hegp-rust-anim";
import { memory } from "hegp-rust-anim/hegp_rust_anim_bg";

const render = (animState, ctx) => {
  console.log("in render");
  console.log("getting context imagedata");
  let imgData = ctx.getImageData(0, 0, 10, 10);
  console.log("getting data len");
  let anim_data_len = animState.image_data_len();
  console.log("anim data len " + anim_data_len);
  console.log("getting data");
  let anim_data_ptr = animState.image_data();
  let anim_data = new Uint8Array(memory.buffer, anim_data_ptr, anim_data_len);
  let data = imgData.data;
  console.log("copying");
  for (let i = 0; i < anim_data_len; i++) {
    data[i] = anim_data[i];
  }
  console.log("putting imagedata");
  console.log(anim_data[0]);
  ctx.putImageData(imgData, 0, 0, 500, 500);
};

const main = () => {
  console.log("creating animstate");
  let animState = wasm.AnimState.init(10, 10, 5);

  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");

  render(animState, ctx);

  // let imgData = ctx.getImageData();


  window.animState = animState;
};

main();
