export function draw_bytes_to_canvas(animData, width, height, ctx) {
  let imgData = ctx.getImageData(0, 0, width, height);
  let data = imgData.data;
  for (let i = 0; i < animData.length; i++) {
    data[i] = animData[i];
  }
  ctx.putImageData(imgData, 0, 0);
}
