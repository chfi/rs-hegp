export function render(animData, width, height, ctx) {
  let imgData = ctx.getImageData(0, 0, width, height);
  let data = imgData.data;
  for (let i = 0; i < animData.length; i++) {
    data[i] = animData[i];
  }
  ctx.putImageData(imgData, 0, 0);
}

export function play_forward(animState, millis) {
  let handle = setInterval(() => {
    animState.next_step();
  }, millis);

  return millis;
}
