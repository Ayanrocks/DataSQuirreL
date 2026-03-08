const sharp = require("sharp");
const path = require("path");

async function analyze() {
  const imgPath = process.argv[2];
  if (!imgPath) {
    console.error("Usage: node analyze_icon.cjs <image_path>");
    process.exit(1);
  }
  const img = sharp(path.resolve(imgPath));
  const metadata = await img.metadata();
  console.log("Size:", metadata.width, metadata.height);

  const stats = await img.stats();

  // Let's get pixels at corners and middle
  const buffer = await img.raw().toBuffer();
  const getPixel = (x, y) => {
    const idx = (y * metadata.width + x) * metadata.channels;
    return `R:${buffer[idx]} G:${buffer[idx + 1]} B:${buffer[idx + 2]}`;
  };

  console.log("Top Left:", getPixel(0, 0));
  console.log("Top Right:", getPixel(metadata.width - 1, 0));
  console.log(
    "Center:",
    getPixel(Math.floor(metadata.width / 2), Math.floor(metadata.height / 2)),
  );
  console.log("Border Mid-Top:", getPixel(Math.floor(metadata.width / 2), 0));
  console.log("Mid-Left:", getPixel(0, Math.floor(metadata.height / 2)));

  // find boundary by scanning from left to right at y = 512
  let borderLeft = 0;
  let bg = getPixel(0, 512);
  for (let x = 0; x < metadata.width; x++) {
    if (getPixel(x, 512) !== bg) {
      borderLeft = x;
      break;
    }
  }
  console.log("Left content starts at:", borderLeft);
}

analyze().catch(console.error);
