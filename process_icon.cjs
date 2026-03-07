const sharp = require("sharp");
const path = require("path");
const fs = require("fs");

async function processOriginal() {
  const rawImgPath =
    process.argv[2] || process.env.IMAGE_PATH || "./assets/icon.png";

  const imgPath = path.resolve(rawImgPath);

  try {
    const stat = fs.statSync(imgPath);
    if (!stat.isFile()) {
      console.error(`Error: "${imgPath}" is not a file.`);
      process.exit(1);
    }
  } catch {
    console.error(`Error: Image not found or not readable at "${imgPath}".`);
    process.exit(1);
  }

  // create squircle mask
  const width = 640;
  const height = 640;
  // standard apple squircle roughly rx = 0.225 * width
  const rx = Math.round(width * 0.225);

  const maskSvg = Buffer.from(
    `<svg width="${width}" height="${height}">
      <rect x="0" y="0" width="${width}" height="${height}" rx="${rx}" ry="${rx}" fill="white" />
    </svg>`,
  );

  const img = sharp(imgPath);

  // Apply the squircle mask
  // Wait, if the generated image's squircle is smaller or larger, this mask might clip it or leave a white border.
  // Let's first trim the white background from the original image using sharp's trim!

  const trimmed = await img.trim({ threshold: 50 }).toBuffer();
  const trimmedImg = sharp(trimmed);
  const trimmedMeta = await trimmedImg.metadata();
  console.log("Trimmed size:", trimmedMeta.width, trimmedMeta.height);

  // Now, the trimmed image is just the dark squircle.
  // We can just resize that to 824x824 (apple recommend size for the main shape)
  // And place it on a 1024x1024 transparent canvas.

  const finalRx = Math.round(824 * 0.225);
  const finalMask = Buffer.from(
    `<svg width="824" height="824">
      <rect x="0" y="0" width="824" height="824" rx="${finalRx}" ry="${finalRx}" fill="white" />
    </svg>`,
  );

  const squircled = await trimmedImg
    .resize(824, 824, { fit: "fill" })
    .composite([{ input: finalMask, blend: "dest-in" }])
    .png()
    .toBuffer();

  // Now put it inside 1024x1024 transparent.
  await sharp({
    create: {
      width: 1024,
      height: 1024,
      channels: 4,
      background: { r: 0, g: 0, b: 0, alpha: 0 },
    },
  })
    .composite([{ input: squircled, gravity: "center" }])
    .png()
    .toFile("src-tauri/icons/icon.png");

  console.log("Successfully generated icon.png");
}

processOriginal().catch((err) => {
  console.error(err);
  process.exit(1);
});
