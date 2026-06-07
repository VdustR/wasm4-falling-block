import { existsSync, readFileSync, statSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const required = [
  "dist/index.html",
  "dist/manifest.webmanifest",
  "dist/sw.js",
  "dist/icon.svg",
  "target/wasm32-unknown-unknown/release/stackline.wasm"
];

for (const file of required) {
  const path = join(root, file);
  if (!existsSync(path) || statSync(path).size === 0) {
    throw new Error(`Missing build artifact: ${file}`);
  }
}

const html = readFileSync(join(root, "dist/index.html"), "utf8");
const sw = readFileSync(join(root, "dist/sw.js"), "utf8");
const manifest = readFileSync(join(root, "dist/manifest.webmanifest"), "utf8");

for (const needle of [
  "<wasm4-app",
  "id=\"startGame\"",
  "id=\"updatePrompt\"",
  "navigator.serviceWorker.register"
]) {
  if (!html.includes(needle)) {
    throw new Error(`dist/index.html missing ${needle}`);
  }
}

if (!sw.includes("CACHE_ASSETS") || !sw.includes("SKIP_WAITING")) {
  throw new Error("dist/sw.js is missing offline/update behavior");
}

const parsedManifest = JSON.parse(manifest);
if (parsedManifest.display !== "standalone" || parsedManifest.start_url !== ".") {
  throw new Error("manifest is not configured for a standalone PWA");
}

console.log("Smoke test passed");
