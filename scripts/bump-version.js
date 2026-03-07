import fs from "fs";
import path from "path";

const bumpType = process.argv[2];

if (!["major", "minor", "patch"].includes(bumpType)) {
  console.error(
    "Please provide a valid bump type: 'major', 'minor', or 'patch'. Example: npm run bump patch",
  );
  process.exit(1);
}

const versionFilePath = path.resolve("VERSION");
const packageJsonPath = path.resolve("package.json");
const tauriConfPath = path.resolve("src-tauri/tauri.conf.json");
const cargoTomlPath = path.resolve("src-tauri/Cargo.toml");

// Read current version from VERSION file (single source of truth)
const currentVersion = fs.readFileSync(versionFilePath, "utf8").trim();
const [major, minor, patch] = currentVersion.split(".").map(Number);

let newVersion;
if (bumpType === "major") {
  newVersion = `${major + 1}.0.0`;
} else if (bumpType === "minor") {
  newVersion = `${major}.${minor + 1}.0`;
} else if (bumpType === "patch") {
  newVersion = `${major}.${minor}.${patch + 1}`;
}

// Update VERSION file
fs.writeFileSync(versionFilePath, newVersion + "\n");
console.log(`Updated VERSION to ${newVersion}`);

// Update package.json
const pkg = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
pkg.version = newVersion;
fs.writeFileSync(packageJsonPath, JSON.stringify(pkg, null, 2) + "\n");
console.log(`Updated package.json to ${newVersion}`);

// Update tauri.conf.json
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
tauriConf.version = newVersion;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + "\n");
console.log(`Updated tauri.conf.json to ${newVersion}`);

// Update Cargo.toml
let cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
cargoToml = cargoToml.replace(
  /version\s*=\s*"[^"]+"/,
  `version = "${newVersion}"`,
);
fs.writeFileSync(cargoTomlPath, cargoToml);
console.log(`Updated Cargo.toml to ${newVersion}`);

console.log(`\nVersion bumped successfully to ${newVersion} 🚀`);
