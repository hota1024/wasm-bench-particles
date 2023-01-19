const particles = [];
window["particles"] = particles;

const canvas = document.querySelector("#canvas");
const ctx = canvas.getContext("2d");

window["clear"] = () => {
  ctx.clearRect(0, 0, 300, 300);
};

window["dot"] = (x, y) => {
  ctx.fillStyle = "black";
  ctx.fillRect(x, y, 1, 1);
};

const jsFrame = () => {
  clear();
  particles.forEach((p) => {
    p.x += Math.cos(p.angle) * 1;
    p.y += Math.sin(p.angle) * 1;
    dot(p.x, p.y);
  });
};

const frameBench = (label, frame) =>
  new Promise((r) => {
    const frames = 100;
    let time = 0;
    let frameCount = 0;
    const id = setInterval(() => {
      const start = performance.now();
      frame();
      time += performance.now() - start;
      frameCount++;
      if (frameCount >= frames) {
        clearInterval(id);
        console.log(`${label} frame avg: ${time / frames} ms`);
        r();
      }
    }, 1000 / 60);
  });

const wasm = async (mod) => {
  console.time("wasm init");
  mod.init();
  console.timeEnd("wasm init");

  console.time("wasm frame");
  mod.frame();
  console.timeEnd("wasm frame");

  await frameBench("wasm", () => mod.frame());
};

const js = async () => {
  console.time("js init");
  for (let i = 0; i < 100000; ++i) {
    particles.push({
      x: Math.random() * 300,
      y: Math.random() * 300,
      angle: Math.random() * Math.PI * 2,
    });
  }
  console.timeEnd("js init");

  console.time("js frame");
  jsFrame();
  console.timeEnd("js frame");

  await frameBench("js", () => jsFrame());
};

import("../pkg/index.js")
  .then(async (mod) => {
    await wasm(mod);
    await js();
  })
  .catch(console.error);
