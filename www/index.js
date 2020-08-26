import * as wasm from "hegp-rust-anim";
import { memory } from "hegp-rust-anim/hegp_rust_anim_bg";

const main = () => {
  let animState = wasm.AnimState.init(10, 10, 5);

  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");

  let imgData = ctx.getImageData();


  window.animState = animState;
};

main();
